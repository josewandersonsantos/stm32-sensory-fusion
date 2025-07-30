#![allow(dead_code)]
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

pub fn deframe_nmea(sentence: &str)
{
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
}

