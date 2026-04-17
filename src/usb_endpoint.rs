/*
 * References about PMA and BTABLE
 * https://community.st.com/t5/stm32-mcus/how-to-configure-the-packet-memory-area-in-stm32-usb-controllers/ta-p/834991#toc-hId--1738762380
 * https://community.st.com/t5/stm32-mcus/practical-use-cases-on-how-to-configure-packet-memory-area-in/ta-p/846312
 * 
 */
 
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use crate::utils;
use crate::mcu;
use crate::usb_types;

#[derive(Clone, Copy, PartialEq)]
enum EndpointState
{
    Idle,       // Waiting for SETUP packet
    Setup,      // SETUP packet received
    DataIn,     // Sending data to host (IN)
    DataOut,    // Receiving data from host (OUT) - not used in this minimal version
    StatusIn,   // Status stage (IN)
    StatusOut,  // Status stage (OUT)
}

#[derive(Clone, Copy)]
struct Endpoint
{
    number: usb_types::Endpoints,
    address: u8,
    ep_type: usb_types::EndpointType,
    
    device_descriptor: [u8; 18],
    config_descriptor: [u8; 9],
    string0: [u8; 4],
    string1: [u8; 10],
    string2: [u8; 16],
    string3: [u8; 10],

    state: EndpointState,
    data_buffer: [u8; 64], // Buffer for data to send/receive
    length: usize,         // Total length of data to transfer
    position: usize,       // Current position in the data buffer
    tx_buffer_addr: u16,   // PMA address for TX buffer
    rx_buffer_addr: u16,   // PMA address for RX buffer
    tx_count: u16,         // Number of bytes to send in the next IN transaction
    rx_count: u16,         // Number of bytes received in the last OUT transaction
}

const DEFAULT_EP: Endpoint = Endpoint
{
    number: usb_types::Endpoints::EP0, // vai sobrescrever depois
    address: 0x00,
    ep_type: usb_types::EndpointType::CONTROL,
    
    device_descriptor: [0; 18],
    config_descriptor: [0; 9],
    string0: [0; 4],
    string1: [0; 10],
    string2: [0; 16],
    string3: [0; 10],

    state: EndpointState::Idle,
    data_buffer: [0; 64],
    length: 0,
    position: 0,
    tx_buffer_addr: 0,  // ADDR_TX field in BTABLE
    rx_buffer_addr: 0,  // ADDR_RX field in BTABLE
    tx_count: 0,        // COUNT_TX field in BTABLE
    rx_count: 0,        // COUNT_RX field in BTABLE (for OUT endpoints, this is set by hardware to the number of bytes received)
};

static mut ENDPOINTS_HANDLERS: [Endpoint; 8] =
[
    //Endpoint 0 (Control Endpoint)
    Endpoint
    {
        number: usb_types::Endpoints::EP0,
        address: 0x00,
        ep_type: usb_types::EndpointType::CONTROL,
        device_descriptor: 
        [
            0x12,         // bLength
            1,            // bDescriptorType = DEVICE
            0x00, 0x02,   // bcdUSB = 2.00
            0x00,         // bDeviceClass
            0x00,         // bDeviceSubClass
            0x00,         // bDeviceProtocol
            0x40,         // bMaxPacketSize0 = 64 bytes
            0x34, 0x12,   // idVendor  (0x1234)
            0x78, 0x56,   // idProduct (0x5678)
            0x00, 0x01,   // bcdDevice
            0x01,         // iManufacturer
            0x2,          // iProduct
            0x3,          // iSerialNumber
            0x1           // bNumConfigurations
        ],
        config_descriptor:
        [
            0x09, 0x02,
            0x09, 0x00,
            0x01,
            0x01,
            0x00,
            0x80,
            0x32,
        ],
        string0:
        [
            0x04, 0x03,
            0x09, 0x04,
        ],
        string1:
        [
            10, 0x03,
            b'A', 0, b'C', 0, b'M', 0, b'E', 0
        ],
        string2:
        [
            16, 0x03,
            b'U',0, b'S',0, b'B',0, b' ',0,
            b'D',0, b'e',0, b'v',0
        ],
        string3:
        [
            10, 0x03,
            b'1',0, b'2',0, b'3',0, b'4',0
        ],
        state: EndpointState::Idle,
        data_buffer: [0; 64],
        length: 0,
        position: 0,
        tx_buffer_addr: 0x40,   // ADDR_TX field in BTABLE
        rx_buffer_addr: 0x80,   // ADDR_RX field in BTABLE
        tx_count: 0x00,         // COUNT_TX field in BTABLE
        rx_count: 0x8400,       // COUNT_RX field in BTABLE
    },
    Endpoint { number: usb_types::Endpoints::EP1, ..DEFAULT_EP },
    Endpoint { number: usb_types::Endpoints::EP2, ..DEFAULT_EP },
    Endpoint { number: usb_types::Endpoints::EP3, ..DEFAULT_EP },
    Endpoint { number: usb_types::Endpoints::EP4, ..DEFAULT_EP },
    Endpoint { number: usb_types::Endpoints::EP5, ..DEFAULT_EP },
    Endpoint { number: usb_types::Endpoints::EP6, ..DEFAULT_EP },
    Endpoint { number: usb_types::Endpoints::EP7, ..DEFAULT_EP },
];

fn enable_usb_peripheral()
{
    utils::set_bit16(mcu::USB_DADDR as *mut u16, usb_types::USBDADDR::EF as u8);
}

fn get_ep_register(epn: usize) -> *mut u16
{
    let ep = match epn
    {
        0 => mcu::USB_EP0R as *mut u16,
        1 => mcu::USB_EP1R as *mut u16,
        2 => mcu::USB_EP2R as *mut u16,
        3 => mcu::USB_EP3R as *mut u16,
        4 => mcu::USB_EP4R as *mut u16,
        5 => mcu::USB_EP5R as *mut u16,
        6 => mcu::USB_EP6R as *mut u16,
        7 => mcu::USB_EP7R as *mut u16,
        _ => panic!("ep num invalid")
    };

    ep
}

fn set_stat_bits(epr: *mut u16, mask: u16, target: u16)
{
    unsafe
    {
        let mut val = core::ptr::read_volatile(epr);

        let current = val & mask;

        // precisa togglar só os bits diferentes
        let toggle = current ^ target;

        val ^= toggle;

        core::ptr::write_volatile(epr, val);
    }
}

/// Sets STAT_RX to VALID (toggles the bits)
fn set_stat_rx_valid(epn: usize)
{
    let epr = get_ep_register(epn);
    unsafe
    {
        let mut val = core::ptr::read_volatile(epr);
        val ^= (usb_types::STATRX_Status::VALID as u16) << (usb_types::USBEPnR::STAT_RX as u8);
        core::ptr::write_volatile(epr, val);
    }
}

// Sets STAT_TX to VALID (toggles the bits)
fn set_stat_tx_valid(epn: usize)
{
    let epr = get_ep_register(epn);
    unsafe
    {
        let mut val = core::ptr::read_volatile(epr);
        val ^= (usb_types::STATTX_Status::VALID as u16) << (usb_types::USBEPnR::STAT_TX as u8);
        core::ptr::write_volatile(epr, val);
    }
}

fn set_stat_rx_nak(epn: usize)
{
    let epr = get_ep_register(epn);
    unsafe
    {
        let mut val = core::ptr::read_volatile(epr);
        val ^= (usb_types::STATRX_Status::NAK as u16) << (usb_types::USBEPnR::STAT_RX as u8);
        core::ptr::write_volatile(epr, val);
    }
}

fn set_stat_tx_nak(epn: usize)
{
    let epr = get_ep_register(epn);
    unsafe
    {
        let mut val = core::ptr::read_volatile(epr);
        val ^= (usb_types::STATTX_Status::NAK as u16) << (usb_types::USBEPnR::STAT_TX as u8);
        core::ptr::write_volatile(epr, val);
    }
}

/// Stalls both directions of Endpoint 0 (used for unsupported requests)
fn stall_ep(epn: usize)
{
    let epr = get_ep_register(epn);
    unsafe
    {
        let mut val = core::ptr::read_volatile(epr);
        val ^= (usb_types::STATRX_Status::STALL as u16) << (usb_types::USBEPnR::STAT_TX as u8);
        val ^= (usb_types::STATRX_Status::STALL as u16) << (usb_types::USBEPnR::STAT_RX as u8);
        core::ptr::write_volatile(epr, val);
    }
}

// Reads data from Packet Memory Area (PMA) into a buffer
// Note: PMA is 16-bit wide, so we handle byte packing manually
fn pma_read(addr: u16, buffer: &mut [u8], len: usize)
{
    unsafe
    {
        let n_bytes = (len + 1) >> 1;
        let mut pma = usb_types::PMA_BASE as *mut u16;
        pma = pma.add(addr as usize);
        
        for i in 0..n_bytes
        {
            let word     = core::ptr::read_volatile(pma);
            buffer[i * 2]     = (word & 0xff) as u8;
            buffer[i * 2 + 1] = (word >> 8) as u8;
            pma = pma.add(2);
        }
    }
}

fn pma_write(addr: u16, buffer: &[u8])
{
    unsafe
    {
        let n_bytes = (buffer.len() + 1) >> 1;
        let mut pma = usb_types::PMA_BASE as *mut u16;
        pma = pma.add(addr as usize);

        for i in 0..n_bytes
        {
            let mut word: u16 = 0;
            // LSB
            word |= buffer[i * 2] as u16;
            // MSB
            if i * 2 + 1 < buffer.len()
            {
                word |= (buffer[i * 2 + 1] as u16) << 8;
            }
            core::ptr::write_volatile(pma, word);
            pma = pma.add(2);
        }
    }
}

/// Sends the next chunk of data during a Data IN stage
fn send_next_packet(epn: usize, addr_tx: u16, len: usize, pos: &mut usize, data: &[u8])
{
    let chunk =
    {
        let remaining = len - *pos;
        remaining.min(64)
    };
    
    // Copy data to PMA
    pma_write(addr_tx, &data[*pos..*pos + chunk]);
    *pos += chunk;

    // Update TX count and set TX status to VALID
    write_count_tx(epn, chunk as u16);
    //set_stat_rx_nak(epn);
    for _ in 0..1000 { core::hint::spin_loop(); }
    set_stat_tx_valid(epn);
}

/// Writes the TX byte count for Endpoint 0 into PMA
fn write_count_tx(epn: usize, count: u16)
{
    unsafe
    {
        let pma = usb_types::PMA_BASE as *mut u16;
        // GET COUNT_TX BY ENDPOINT
        let addr = match epn
        {
            0 => pma.add(usb_types::BTABLE_ADDRESS::EP0_COUNT_TX as usize) as *mut u16,
            1 => pma.add(usb_types::BTABLE_ADDRESS::EP1_COUNT_TX as usize) as *mut u16,
            2 => pma.add(usb_types::BTABLE_ADDRESS::EP2_COUNT_TX as usize) as *mut u16,
            _ => return
        };
        core::ptr::write_volatile(addr, count & 0x03FF);
    }
}

/// Clears the CTR_RX flag in EP0R
fn clear_ctr_rx(ep: *mut u16)
{
    unsafe
    {
        // let val = core::ptr::read_volatile(ep);
        // core::ptr::write_volatile(ep, val & !(1 << usb_types::USBEPnR::CTR_RX as u16));
        utils::clear_bit16(ep, usb_types::USBEPnR::CTR_RX as u8);
    }
}

/// Clears the CTR_TX flag in EP0R
fn clear_ctr_tx(ep: *mut u16)
{
    unsafe 
    {
        // let val = core::ptr::read_volatile(ep);
        // core::ptr::write_volatile(ep, val & !(1 << usb_types::USBEPnR::CTR_TX as u16));
        utils::clear_bit16(ep, usb_types::USBEPnR::CTR_TX as u8);
    }
}

fn handle_set_address(epn: usize, wlength: u16)
{

}

fn get_descriptor(epn: usize, wvalue: u16) -> Option<&'static [u8]>
{
    let desc_type  = (wvalue >> 8) as u8;
    let desc_index = (wvalue & 0xFF) as u8;

    unsafe 
    {
        match desc_type
        {
            1 => Some(&ENDPOINTS_HANDLERS[epn].device_descriptor),
            2 => Some(&ENDPOINTS_HANDLERS[epn].config_descriptor),
            3 => 
            {
                match desc_index
                {
                    0 => Some(&ENDPOINTS_HANDLERS[epn].string0),
                    1 => Some(&ENDPOINTS_HANDLERS[epn].string1),
                    2 => Some(&ENDPOINTS_HANDLERS[epn].string2),
                    3 => Some(&ENDPOINTS_HANDLERS[epn].string3),
                    _ => None
                }
            },
            _ => None
        }
    }
}

/// Handles GET_DESCRIPTOR request
fn handle_get_descriptor(epn: usize, wvalue:u16, wlength: u16)
{
    if let Some(data) = get_descriptor(epn, wvalue)
    {
        let len = core::cmp::min(data.len(), wlength as usize);
        unsafe
        {
            ENDPOINTS_HANDLERS[epn].state = EndpointState::DataIn;
            ENDPOINTS_HANDLERS[epn].length = len;
            ENDPOINTS_HANDLERS[epn].position = 0;
            ENDPOINTS_HANDLERS[epn].data_buffer[..len].copy_from_slice(&data[..len]);

            send_next_packet(epn, ENDPOINTS_HANDLERS[epn].tx_buffer_addr, len, &mut ENDPOINTS_HANDLERS[epn].position, &ENDPOINTS_HANDLERS[epn].data_buffer);
        }
    }
    else
    {
        stall_ep(epn);
    }
}

/// Handles STATUS packets (Standard Device Requests)
fn handle_get_status(epn: usize, wlength: u16)
{
    unsafe
    {
        let ep = &mut ENDPOINTS_HANDLERS[epn];
        
        // Response for GET_STATUS (Device): 00 00
        ep.data_buffer[0] = 0x00;   // bit0 = self-powered? (0 = no)
        ep.data_buffer[1] = 0x00;   // bit1 = remote wakeup? (0 = no)
        
        ep.length = core::cmp::min(wlength as usize, 2);
        ep.position = 0;
        ep.state = EndpointState::DataIn;

        send_next_packet(epn, ep.tx_buffer_addr, ep.length, &mut ep.position, &ep.data_buffer);
    }
}

/// Handles SETUP packets (Standard Device Requests)
fn handle_setup(epn: usize)
{
    let mut setup = [0u8; 8];
    unsafe
    {
        // Read 8-byte SETUP packet from PMA
        // let base = usb_types::PMA_BASE as *const u16;
        // let addr_tx  = core::ptr::read_volatile(base.add(0));   // ADDR_TX  (offset 0x00)
        // let count_tx = core::ptr::read_volatile(base.add(2)) & 0x3FF;   // COUNT_TX (offset 0x02)
        // let addr_rx  = core::ptr::read_volatile(base.add(4));   // ADDR_RX  (offset 0x04)
        // let count_rx = core::ptr::read_volatile(base.add(6)) & 0x3FF;   // COUNT_RX (offset 0x06)

        pma_read(ENDPOINTS_HANDLERS[epn].rx_buffer_addr, &mut setup, 8);
        ENDPOINTS_HANDLERS[epn].state = EndpointState::Setup;
    }

    // bRequest
    let brequesttype = setup[0];
    let brequest     = setup[1];
    let wvalue      = ((setup[3] as u16) << 8) | (setup[2] as u16);
    let windex      = ((setup[5] as u16) << 8) | (setup[4] as u16);
    let wlength     = ((setup[7] as u16) << 8) | (setup[6] as u16);

    match brequest
    {
        // GET_STATUS
        0 => 
        {
            handle_get_status(epn, wlength)
            // handle_get_descriptor(epn, 1, 18)
        },
        5 => 
        {
            handle_set_address(epn, wlength)
        },
        // GET_DESCRIPTOR
        6 => 
        {
            handle_get_descriptor(epn, wvalue, wlength)
        },
        // Unsupported request → STALL
        _ => stall_ep(epn)
    }
}

/// Called when an IN transaction completes
fn handle_in(epn: usize)
{
    unsafe
    {
        match ENDPOINTS_HANDLERS[epn].state
        {
            EndpointState::DataIn =>
            {
                if ENDPOINTS_HANDLERS[epn].position < ENDPOINTS_HANDLERS[epn].length
                {
                    // More data to send
                    send_next_packet(epn, ENDPOINTS_HANDLERS[epn].tx_buffer_addr, ENDPOINTS_HANDLERS[epn].length, &mut ENDPOINTS_HANDLERS[epn].position, &ENDPOINTS_HANDLERS[epn].data_buffer);
                } 
                else
                {
                    // Data stage finished → go to Status OUT stage
                    ENDPOINTS_HANDLERS[epn].state = EndpointState::StatusOut;
                    set_stat_rx_valid(epn);
                    set_stat_tx_nak(epn);
                }
            }
            EndpointState::StatusIn =>
            {
                // Status stage completed
                ENDPOINTS_HANDLERS[epn].state = EndpointState::Idle;
            }
            _ => {}
        }
    }
}

/// Called when an OUT transaction completes
fn handle_out(epn: usize)
{
    unsafe
    {
        match ENDPOINTS_HANDLERS[epn].state
        {
            EndpointState::StatusOut =>
            {
                // Status stage completed
                ENDPOINTS_HANDLERS[epn].state = EndpointState::Idle;
                set_stat_rx_valid(epn);
                set_stat_tx_nak(epn);
            }
            _ => {}
        }
    }
}

/// Main handler for Endpoint 0 (Control Endpoint)
pub fn handler_endpoint(epn: usize)
{
    let epr = get_ep_register(epn);
    let ep = unsafe { core::ptr::read_volatile(epr) };

    // ========================
    // RX Side (SETUP or OUT packet received)
    // ========================
    if ep & (1 << usb_types::USBEPnR::CTR_RX as u16) != 0 // CTR_RX flag set
    {
        if ep & (1 << usb_types::USBEPnR::SETUP as u16) != 0 // SETUP bit set
        {
            handle_setup(epn);
        }
        else
        {
            // Regular OUT data packet
            handle_out(epn);
        }
        clear_ctr_rx(epr);
    }
    
    // ========================
    // TX Side (IN packet transmission completed)
    // ========================
    if ep & (1 << usb_types::USBEPnR::CTR_TX as u16) != 0
    {
        handle_in(epn);
        clear_ctr_tx(epr);
    }

}

pub fn configure_ep(epn: usb_types::Endpoints, ep_type: usb_types::EndpointType)
{
    match epn
    {
        // Configures Endpoint 0 (Control Endpoint) buffers and registers
        usb_types::Endpoints::EP0 =>
        {
            // === Buffer Description Table (BTABLE) entries for EP0 ===
            unsafe
            {
                // === Configure EP0R Register ===
                let ep0r = mcu::USB_EP0R as *mut u16;            
                let mut val: u16 = 0;
                // Bits [3:0]  = EA[3:0]  → Endpoint Address = 0
                // Bits [8:9]  = EP_TYPE  → 01 = Control
                val |= (ep_type as u16) << (usb_types::USBEPnR::EP_TYPE as u8);
                core::ptr::write_volatile(ep0r, val);

                let pma = usb_types::PMA_BASE as *mut u16;
                core::ptr::write_volatile(pma.add(usb_types::BTABLE_ADDRESS::EP0_COUNT_RX as usize), ENDPOINTS_HANDLERS[0].rx_count);      // COUNT_RX
                core::ptr::write_volatile(pma.add(usb_types::BTABLE_ADDRESS::EP0_ADDR_RX as usize), ENDPOINTS_HANDLERS[0].rx_buffer_addr); // ADDR_RX
                core::ptr::write_volatile(pma.add(usb_types::BTABLE_ADDRESS::EP0_COUNT_TX as usize), ENDPOINTS_HANDLERS[0].tx_count);      // COUNT_TX
                core::ptr::write_volatile(pma.add(usb_types::BTABLE_ADDRESS::EP0_ADDR_TX as usize), ENDPOINTS_HANDLERS[0].tx_buffer_addr); // ADDR_TX
                // let addr_tx  = core::ptr::read_volatile(pma.add(0));   // ADDR_TX  (offset 0x00)
                // let addr_rx  = core::ptr::read_volatile(pma.add(4));   // ADDR_RX  (offset 0x04)
                // let count_rx = core::ptr::read_volatile(pma.add(6));   // COUNT_RX (offset 0x06)
                // let count_tx = core::ptr::read_volatile(pma.add(2));   // COUNT_TX (offset 0x02)
                // let count_tx = core::ptr::read_volatile(pma.add(2)) & 0x3FF;   // COUNT_TX (offset 0x02)
            }

            set_stat_tx_nak(usb_types::Endpoints::EP0 as usize);
            set_stat_rx_valid(usb_types::Endpoints::EP0 as usize);
        }
        _ => return

    }
    // Enable USB peripheral
    enable_usb_peripheral();    
}

/// USB Low Priority Interrupt Handler
pub fn handler_endpoint_interrupt()
{
    unsafe
    {
        let usb_istr = mcu::USB_ISTR as *mut u16;
        let mut istr = utils::read_register16(usb_istr);
        // Extract endpoint number
        let ep_id = istr & 0x0F;

        // ESOF
        if istr & (1 << usb_types::USBISTR::ESOF as u16) != 0
        {
            istr &= !(1 << usb_types::USBISTR::ESOF as u16);
        }

        // SOF
        if istr & (1 << usb_types::USBISTR::SOF as u16) != 0
        {
            istr &= !(1 << usb_types::USBISTR::SOF as u16);
        }        

        // RESET
        if istr & (1 << usb_types::USBISTR::RESET as u16) != 0
        {
            // Handle EP0 interrupt on USB reset
            configure_ep(usb_types::Endpoints::EP0, usb_types::EndpointType::CONTROL);
            istr &= !(1 << usb_types::USBISTR::RESET as u16);
        }

        // SUSP (Suspend)
        if istr & (1 << usb_types::USBISTR::SUSP as u16) != 0
        {
            // entra em low power mode
            // let usb_cntr = mcu::USB_CNTR as *mut u16;
            // utils::set_bit16(usb_cntr, 1); // LP_MODE = 1
            istr &= !(1 << usb_types::USBISTR::SUSP as u16);
        }
        
        // WKP (Wakeup)
        if istr & (1 << usb_types::USBISTR::WKUP as u16) != 0
        {
            istr &= !(1 << usb_types::USBISTR::WKUP as u16);
        }

        // ERR (Error)
        if istr & (1 << usb_types::USBISTR::ERR as u16) != 0
        {
            istr &= !(1 << usb_types::USBISTR::ERR as u16);
        }

        // PMAOVR (PMA Over/underrun)
        if istr & (1 << usb_types::USBISTR::PMAOVR as u16) != 0
        {
            istr &= !(1 << usb_types::USBISTR::PMAOVR as u16);
        }

        // Correct Transfer (CTR) interrupt
        if istr & (1 << usb_types::USBISTR::CTR as u16) != 0
        {
            handler_endpoint(ep_id as usize);
            istr &= !(1 << usb_types::USBISTR::CTR as u16);
        }

        // Escreve de volta no ISTR para limpar as flags que tratamos
        utils::write_register16(usb_istr, istr);
    }
}