#![allow(dead_code)]

use crate::usart;
/*
 * CONST
 */
const BUFFER_SIZE: usize = 512;
const LINE_SIZE: usize = 128;
/*
 * VARIABLES
 */
static mut RX_BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
static mut RX_HEAD: usize = 0;
static mut RX_TAIL: usize = 0;

static mut LINE: [u8; LINE_SIZE] = [0; LINE_SIZE];
static mut LINE_POS: usize = 0;

/*
 * CALLBACKS
 */
static mut ON_LINE_RECEIVED: Option<fn(&str)> = None;

/*
 * ENUMS
 */
pub enum GPS_Frequency
{
    F1Hz,
    F5Hz,
    F10Hz,
}

pub enum GPS_Protocol
{
    NMEA,
    UBX,
    Both,
}

pub enum GPS_BaudRate
{
    B9600,
    B19200,
    B38400,
    B57600,
    B115200,
}

pub enum GPS_UpdateRate
{
    R1Hz,
    R5Hz,
    R10Hz,
    R20Hz,
    R100Hz,
}

pub enum GPS_OperationMode
{
    Normal,
    PowerSaving,
    Airborne,
}

pub enum GPS_NmeaSentence
{
    GGA,
    RMC,
    GLL,
    VTG,
    ZDA,
}

pub enum UBX_Class
{
    NAV = 0x01,
    RXM = 0x02,
    INF = 0x04,
    ACK = 0x05,
    CFG = 0x06,
    UPD = 0x09,
    MON = 0x0A,
    AID = 0x0B,
    TIM = 0x0D,
    ESF = 0x10,
    MGA = 0x13,
    LOG = 0x21,
    SEC = 0x27,
}

pub enum UBX_Id
{
    NAV_PVT = 0x07,
    CFG_PRT = 0x00,
    CFG_MSG = 0x01,
    CFG_RATE = 0x08,
    CFG_NAV5 = 0x24,
}

pub fn push_byte(byte: u8)
{
    unsafe
    {
        let next_head = (RX_HEAD + 1) % BUFFER_SIZE;
        if next_head != RX_TAIL
        {
            RX_BUFFER[RX_HEAD] = byte;
            RX_HEAD = next_head;
        }
    }
}

pub fn pop_byte() -> Option<u8>
{
    unsafe
    {
        if RX_HEAD == RX_TAIL
        {
            None
        }
        else
        {
            let byte = RX_BUFFER[RX_TAIL];
            RX_TAIL = (RX_TAIL + 1) % BUFFER_SIZE;
            Some(byte)
        }
    }
}

pub fn process_gps()
{
    while let Some(byte) = pop_byte()
    {
        unsafe
        {
            if byte == b'\n'
            {
                let line_str = str::from_utf8(&LINE[..LINE_POS]).unwrap_or("");

                if line_str.starts_with("$")
                {
                    deframe_nmea(line_str);
                }

                LINE_POS = 0;
            }
            else if LINE_POS < LINE_SIZE
            {
                LINE[LINE_POS] = byte;
                LINE_POS += 1;
            }
            else
            {
                LINE_POS = 0; // Overflow de linha
            }
        }
    }
}

pub fn calculate_ubx_checksum(data: &[u8]) -> (u8, u8)
{
    let mut ck_a: u8 = 0;
    let mut ck_b: u8 = 0;

    for &byte in data
    {
        ck_a = ck_a.wrapping_add(byte);
        ck_b = ck_b.wrapping_add(ck_a);
    }

    (ck_a, ck_b)
}

pub fn make_ubx_message(class: u8, id: u8, payload: &[u8], buffer: &mut [u8]) -> usize
{
    let payload_len = payload.len();
    let total_len = 6 + payload_len + 2;

    assert!(buffer.len() >= total_len);

    buffer[0] = 0xB5;
    buffer[1] = 0x62;
    buffer[2] = class;
    buffer[3] = id;

    let len = payload_len as u16;
    buffer[4] = (len & 0xFF) as u8;
    buffer[5] = (len >> 8) as u8;

    buffer[6..6+payload_len].copy_from_slice(payload);

    let checksum = calculate_ubx_checksum(&buffer[2..6+payload_len]);
    buffer[6+payload_len] = checksum.0;
    buffer[7+payload_len] = checksum.1;

    total_len
}

pub fn send_ubx_message(uart: usart::Usart, class: u8, id: u8, payload: &[u8])
{
    let mut buffer: [u8; 256] = [0; 256];
    let message_len = make_ubx_message(class, id, payload, &mut buffer);
    usart::write_bytes(uart, &buffer[..message_len]);
    return;

    // Get ACK
    let mut ack_buffer: [u8; 10] = [0; 10];
    let mut ack_index = 0;
    while ack_index < 10
    {
        if let Some(byte) = pop_byte()
        {
            ack_buffer[ack_index] = byte;
            ack_index += 1;

            if ack_index >= 2 && &ack_buffer[ack_index-2..ack_index] == [0xB5, 0x62]
            {
                break; // Início do ACK recebido
            }
        }
    }
}

pub fn send_ubx_cfg_msg_rate(uart: usart::Usart, rate_ms: u16)
{
    let payload = [
        (rate_ms & 0xFF) as u8,
        (rate_ms >> 8) as u8,
        0x01, // navRate
        0x00, // timeRef
    ];

    send_ubx_message(uart, UBX_Class::CFG as u8, UBX_Id::CFG_RATE as u8, &payload);
}

pub fn send_ubx_cfg_msg_nav5(uart: usart::Usart, mode: GPS_OperationMode)
{
    let mut payload: [u8; 36] = [0; 36];
    payload[0] = 0x05; // mask: set dynamic model
    payload[1] = 0x00;
    payload[2] = match mode
    {
        GPS_OperationMode::Normal => 0x00, // Portable
        GPS_OperationMode::PowerSaving => 0x02, // Stationary
        GPS_OperationMode::Airborne => 0x06, // Airborne <1g
    };

    send_ubx_message(uart, UBX_Class::CFG as u8, UBX_Id::CFG_NAV5 as u8, &payload);
}

pub fn send_ubx_cfg_msg_msg(uart: usart::Usart, msg_class: u8, msg_id: u8, rate: u8)
{
    let payload = [
        msg_class,
        msg_id,
        rate,
    ];

    send_ubx_message(uart, UBX_Class::CFG as u8, UBX_Id::CFG_MSG as u8, &payload);
}

pub fn send_ubx_cfg_msg_prt(uart: usart::Usart, port: u8, mode: u8)
{
    let payload = [
        port,
        mode,
    ];

    send_ubx_message(uart, UBX_Class::CFG as u8, UBX_Id::CFG_PRT as u8, &payload);
}

pub fn init(uart: usart::Usart, frequency: GPS_Frequency, protocol: GPS_Protocol, baud_rate: GPS_BaudRate, update_rate: GPS_UpdateRate, operation_mode: GPS_OperationMode, nmea_sentences: &[GPS_NmeaSentence], callback: fn(&str))
{
    unsafe { ON_LINE_RECEIVED = Some(callback) };

    // --------------------------------------------------
    // SET UPDATE RATE (CFG-RATE)
    // --------------------------------------------------
    let rate_ms: u16 = match update_rate
    {
        GPS_UpdateRate::R1Hz  => 1000,
        GPS_UpdateRate::R5Hz  => 200,
        GPS_UpdateRate::R10Hz => 100,
        GPS_UpdateRate::R20Hz => 50,
        GPS_UpdateRate::R100Hz => 10,
    };

    send_ubx_cfg_msg_rate(uart, rate_ms);
    // --------------------------------------------------
    // SET OPERATION MODE (CFG-NAV5)
    // --------------------------------------------------
    send_ubx_cfg_msg_nav5(uart, operation_mode);

    // --------------------------------------------------
    // DISABLE ALL NMEA MSGS
    // --------------------------------------------------
    let all_msgs = [
        (0xF0, 0x00), // GGA
        (0xF0, 0x01), // GLL
        (0xF0, 0x02), // GSA
        (0xF0, 0x03), // GSV
        (0xF0, 0x04), // RMC
        (0xF0, 0x05), // VTG
        (0xF0, 0x08), // ZDA
    ];

    for (class, id) in all_msgs
    {
        send_ubx_cfg_msg_msg(uart, class, id, 0); // 0 = desabilita
    }

    // --------------------------------------------------
    // ENABLE NME MSGS
    // --------------------------------------------------
    for nmea_sentence in nmea_sentences
    {
        match nmea_sentence
        {
            GPS_NmeaSentence::GGA =>
                send_ubx_cfg_msg_msg(uart, 0xF0, 0x00, 1),

            GPS_NmeaSentence::GLL =>
                send_ubx_cfg_msg_msg(uart, 0xF0, 0x01, 1),

            GPS_NmeaSentence::RMC =>
                send_ubx_cfg_msg_msg(uart, 0xF0, 0x04, 1),

            GPS_NmeaSentence::VTG =>
                send_ubx_cfg_msg_msg(uart, 0xF0, 0x05, 1),

            GPS_NmeaSentence::ZDA =>
                send_ubx_cfg_msg_msg(uart, 0xF0, 0x08, 1),
        }
    }

    // --------------------------------------------------
    // SET PROTOCOL (UBX/NMEA)
    // --------------------------------------------------
    // match protocol
    // {
    //     GPS_Protocol::NMEA =>
    //     {
    //     }

    //     GPS_Protocol::UBX =>
    //     {
    //     }

    //     GPS_Protocol::Both =>
    //     {
    //     }
    // }

    // --------------------------------------------------
    // SET BAUD RATE (CFG-PRT)
    // --------------------------------------------------
    // let baud: u32 = match baud_rate
    // {
    //     GPS_BaudRate::B9600   => 9600,
    //     GPS_BaudRate::B19200  => 19200,
    //     GPS_BaudRate::B38400  => 38400,
    //     GPS_BaudRate::B57600  => 57600,
    //     GPS_BaudRate::B115200 => 115200,
    // };

    // let mut payload: [u8; 20] = [0; 20];

    // payload[0] = 1; // UART1
    // payload[1] = 0;

    // // mode (8N1)
    // payload[4] = 0xD0;
    // payload[5] = 0x08;

    // // baudrate
    // payload[8]  = (baud & 0xFF) as u8;
    // payload[9]  = ((baud >> 8) & 0xFF) as u8;
    // payload[10] = ((baud >> 16) & 0xFF) as u8;
    // payload[11] = ((baud >> 24) & 0xFF) as u8;

    // // in/out proto (UBX + NMEA)
    // payload[12] = 0x07;
    // payload[14] = 0x07;

    // send_ubx_message(uart, UBX_Class::CFG as u8, UBX_Id::CFG_PRT as u8, &payload);

}

pub fn deframe_nmea(sentence: &str)
{
    unsafe
    {
        if let Some(cb) = ON_LINE_RECEIVED
        {
            cb(sentence);
        }
    }

    if sentence.starts_with("$GPGGA")
    {
        let mut fields: [&str; 15] = [""; 15]; // NMEA GGA 15 fields
        let mut field_count = 0;
        let mut start = 0;

        for (i, c) in sentence.char_indices()
        {
            if c == ',' || c == '*' || c == '\r' || c == '\n'
            {
                if field_count < fields.len()
                {
                    fields[field_count] = &sentence[start..i];
                    field_count += 1;
                }
                start = i + 1;
            }
        }

        if field_count > 5
        {
            // let lat = fields[2];
            // let lon = fields[4];
            /*
             *
             */
        }
    }
    else if sentence.starts_with("$GPRMC")
    {
        let mut fields: [&str; 12] = [""; 12]; // NMEA RMC 12 fields
        let mut field_count = 0;
        let mut start = 0;

        for (i, c) in sentence.char_indices()
        {
            if c == ',' || c == '*' || c == '\r' || c == '\n'
            {
                if field_count < fields.len()
                {
                    fields[field_count] = &sentence[start..i];
                    field_count += 1;
                }
                start = i + 1;
            }
        }

        if field_count > 8
        {
            // let lat = fields[3];
            // let lon = fields[5];
            /*
             *
             */
        }
    }
}

