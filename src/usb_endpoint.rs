#![allow(dead_code)]
#[allow(non_camel_case_types)]

use crate::utils;
use crate::mcu;
use crate::usb_types;

#[derive(Clone, Copy, PartialEq)]
enum Ep0State
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
    ep_type: usb_types::EndpointType,
    descriptor: [u8; 18], 

    state: Ep0State,
    data_buffer: [u8; 64], // Buffer for data to send/receive
    length: usize,         // Total length of data to transfer
    position: usize,       // Current position in the data buffer
    tx_buffer_addr: u16,   // PMA address for TX buffer
    rx_buffer_addr: u16,   // PMA address for RX buffer
    tx_count: u16,         // Number of bytes to send in the next IN transaction
    rx_count: u16,         // Number of bytes received in the last OUT transaction
}

const DEFAULT_EP: Endpoint = Endpoint {
    number: usb_types::Endpoints::EP0, // vai sobrescrever depois
    ep_type: usb_types::EndpointType::Control,
    descriptor: [0; 18],
    state: Ep0State::Idle,
    data_buffer: [0; 64],
    length: 0,
    position: 0,
    tx_buffer_addr: 0,
    rx_buffer_addr: 0,
    tx_count: 0,
    rx_count: 0,
};

static mut ENDPOINTS_HANDLERS: [Endpoint; 8] =
[
    //Endpoint 0 (Control Endpoint)
    Endpoint
    {
        number: usb_types::Endpoints::EP0,
        ep_type: usb_types::EndpointType::Control,
        descriptor: 
        [
            18,           // bLength
            1,            // bDescriptorType = DEVICE
            0x00, 0x02,   // bcdUSB = 2.00
            0x00,         // bDeviceClass
            0x00,         // bDeviceSubClass
            0x00,         // bDeviceProtocol
            64,           // bMaxPacketSize0 = 64 bytes
            0x34, 0x12,   // idVendor  (0x1234)
            0x78, 0x56,   // idProduct (0x5678)
            0x00, 0x01,   // bcdDevice
            1,            // iManufacturer
            2,            // iProduct
            3,            // iSerialNumber
            1             // bNumConfigurations
        ],        
        state: Ep0State::Idle,
        data_buffer: [0; 64],
        length: 0,
        position: 0,
        tx_buffer_addr: 0x40,   // PMA address for EP0 TX buffer
        rx_buffer_addr: 0x80,   // PMA address for EP0 RX buffer
        tx_count: 0,
        rx_count: 0,
    },
    Endpoint { number: usb_types::Endpoints::EP1, ..DEFAULT_EP },
    Endpoint { number: usb_types::Endpoints::EP2, ..DEFAULT_EP },
    Endpoint { number: usb_types::Endpoints::EP3, ..DEFAULT_EP },
    Endpoint { number: usb_types::Endpoints::EP4, ..DEFAULT_EP },
    Endpoint { number: usb_types::Endpoints::EP5, ..DEFAULT_EP },
    Endpoint { number: usb_types::Endpoints::EP6, ..DEFAULT_EP },
    Endpoint { number: usb_types::Endpoints::EP7, ..DEFAULT_EP },
];

pub fn configure_ep(epn: usb_types::Endpoints, ep_type: usb_types::EndpointType)
{
    match epn
    {
        // Configures Endpoint 0 (Control Endpoint) buffers and registers
        usb_types::Endpoints::EP0 =>
        {
            // Clear EP0R register
            let ep0r = mcu::USB_EP0R as *mut u16;
            unsafe { core::ptr::write_volatile(ep0r, 0x0000); }
            
            // === Buffer Description Table (BTABLE) entries for EP0 ===
            unsafe
            {
                // BTABLE are in 0x0000 from PMA
                // TX Buffer (IN direction) - endereço recomendado: 0x40
                utils::write_register16((usb_types::PMA_BASE + usb_types::BTABLE_ADDRESS::ADDR_TX as u32) as *mut u16, ENDPOINTS_HANDLERS[0].tx_buffer_addr);   // ADDR_TX = 0x40
                utils::write_register16((usb_types::PMA_BASE + usb_types::BTABLE_ADDRESS::COUNT_TX as u32) as *mut u16, ENDPOINTS_HANDLERS[0].tx_count);   // COUNT_TX = 0

                // RX Buffer (OUT/SETUP direction) - endereço recomendado: 0x80 (64 bytes após TX)
                utils::write_register16((usb_types::PMA_BASE + usb_types::BTABLE_ADDRESS::ADDR_RX as u32) as *mut u16, ENDPOINTS_HANDLERS[0].rx_buffer_addr);   // ADDR_RX = 0x80
                utils::write_register16((usb_types::PMA_BASE + usb_types::BTABLE_ADDRESS::COUNT_RX as u32) as *mut u16, ENDPOINTS_HANDLERS[0].rx_count); // COUNT_RX = 64 bytes (BL_SIZE=1, NUM_BLOCK=2)
            }

            // === Configure EP0R Register ===
            let ep0r = mcu::USB_EP0R as *mut u16;
            let daddr = mcu::USB_DADDR as *mut u16;
            unsafe
            {
                let mut val: u16 = 0;

                // Bits [3:0]  = EA[3:0]  → Endpoint Address = 0
                // Bits [8:9]  = EP_TYPE  → 01 = Control
                val |= (ep_type as u16) << (usb_types::USBEPnR::EA as u8);
                core::ptr::write_volatile(ep0r, val);
                core::ptr::write_volatile(daddr, 0x0000);
            }

            set_stat_tx_nak(0);
            set_stat_rx_valid(0);
        }
        _ => return

    }
    
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
            handler_endpoint(ep_id as usize);

            istr &= !(1 << usb_types::USBISTR::RESET as u16);
        }

        // SUSP (Suspend)
        if istr & (1 << usb_types::USBISTR::SUSP as u16) != 0
        {
            let usb_cntr = mcu::USB_CNTR as *mut u16;
            // entra em low power mode
            utils::set_bit16(usb_cntr, 1); // LP_MODE = 1
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

/// Sets STAT_TX to VALID (toggles the bits)
fn set_stat_tx_valid()
{
    let ep = mcu::USB_EP0R as *mut u16;
    unsafe
    {
        let mut val = core::ptr::read_volatile(ep);
        let stat_tx_valid = (usb_types::STATTX_Status::VALID as u16) << (usb_types::USBEPnR::STAT_TX as u8);
        val ^= stat_tx_valid;        // Toggle STAT_TX bits
        core::ptr::write_volatile(ep, val);
    }
}

/// Reads data from Packet Memory Area (PMA) into a buffer
/// Note: PMA is 16-bit wide, so we handle byte packing manually
fn pma_read(addr: u16, buffer: &mut [u8])
{
    let pma = usb_types::PMA_BASE as *const u16;

    for i in 0..buffer.len()
    {
        let word = unsafe { core::ptr::read_volatile(pma.add((addr as usize / 2) + i / 2)) };

        if i % 2 == 0
        {
            buffer[i] = (word & 0xFF) as u8;
        }
        else
        {
            buffer[i] = (word >> 8) as u8;
        }
    }
}

/// Writes data from a buffer into the Packet Memory Area (PMA)
fn pma_write(addr: u16, data: &[u8])
{
    let pma = usb_types::PMA_BASE as *mut u16;

    for i in 0..data.len()
    {
        let index = (addr as usize / 2) + (i / 2);
        let mut word = unsafe { core::ptr::read_volatile(pma.add(index)) };

        if i % 2 == 0
        {
            word = (word & 0xFF00) | (data[i] as u16);
        }
        else
        {
            word = (word & 0x00FF) | ((data[i] as u16) << 8);
        }

        unsafe { core::ptr::write_volatile(pma.add(index), word); }
    }
}

/// Sends the next chunk of data during a Data IN stage
fn send_next_packet(addr_tx: u16, len: usize, pos: &mut usize, data: &[u8])
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
    write_count_tx(chunk as u16);
    set_stat_tx_valid();
}

/// Writes the TX byte count for Endpoint 0 into PMA
fn write_count_tx(count: u16)
{
    let addr = (usb_types::PMA_BASE + 0x02) as *mut u16;   // COUNT_TX field
    unsafe
    {
        core::ptr::write_volatile(addr, count);
    }
}

/// Clears the CTR_RX flag in EP0R
fn clear_ctr_rx(ep: *mut u16)
{
    unsafe
    {
        let val = core::ptr::read_volatile(ep);
        core::ptr::write_volatile(ep, val & !(1 << usb_types::USBEPnR::CTR_RX as u16));
    }
}

/// Clears the CTR_TX flag in EP0R
fn clear_ctr_tx(ep: *mut u16)
{
    unsafe 
    {
        let val = core::ptr::read_volatile(ep);
        core::ptr::write_volatile(ep, val & !(1 << usb_types::USBEPnR::CTR_TX as u16));
    }
}

/// Handles GET_DESCRIPTOR request
fn handle_get_descriptor(epn: usize, setup: &[u8])
{
    unsafe
    {
        let desc_type = setup[3];
        let data = match desc_type
        {
            // Device Descriptor
            1 => 
            {
                &ENDPOINTS_HANDLERS[epn].descriptor
            }
            _ =>
            {
                stall_ep(epn);
                return;
            }
        };

        // Limit response size to what the host requested
        ENDPOINTS_HANDLERS[epn].length   = core::cmp::min(setup[6] as usize, data.len());
        ENDPOINTS_HANDLERS[epn].position = 0;
        ENDPOINTS_HANDLERS[epn].data_buffer[..ENDPOINTS_HANDLERS[epn].length].copy_from_slice(&data[..ENDPOINTS_HANDLERS[epn].length]);
        ENDPOINTS_HANDLERS[epn].state = Ep0State::DataIn;
        
        send_next_packet(ENDPOINTS_HANDLERS[epn].tx_buffer_addr, ENDPOINTS_HANDLERS[epn].length, &mut ENDPOINTS_HANDLERS[epn].position, &ENDPOINTS_HANDLERS[epn].data_buffer);
    }
}

/// Handles SETUP packets (Standard Device Requests)
fn handle_setup(epn: usize)
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
        _ => return
    };

    let mut setup = [0u8; 8];

    unsafe 
    {
        // Read 8-byte SETUP packet from PMA
        pma_read(ENDPOINTS_HANDLERS[epn].rx_buffer_addr, &mut setup);
        ENDPOINTS_HANDLERS[epn].state = Ep0State::Setup;
    }

    // bRequest
    let request = setup[1];

    match request
    {
        // GET_DESCRIPTOR
        6 => 
        {
            handle_get_descriptor(epn,  &setup)
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
            Ep0State::DataIn =>
            {
                if ENDPOINTS_HANDLERS[epn].position < ENDPOINTS_HANDLERS[epn].length
                {
                    // More data to send
                    send_next_packet(ENDPOINTS_HANDLERS[epn].tx_buffer_addr, ENDPOINTS_HANDLERS[epn].length, &mut ENDPOINTS_HANDLERS[epn].position, &ENDPOINTS_HANDLERS[epn].data_buffer);
                } 
                else
                {
                    // Data stage finished → go to Status OUT stage
                    ENDPOINTS_HANDLERS[epn].state = Ep0State::StatusOut;
                    set_stat_rx_valid(epn);
                }
            }
            Ep0State::StatusIn =>
            {
                // Status stage completed
                ENDPOINTS_HANDLERS[epn].state = Ep0State::Idle;
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
            Ep0State::StatusOut =>
            {
                // Status stage completed
                ENDPOINTS_HANDLERS[epn].state = Ep0State::Idle;
            }
            _ => {}
        }
    }
}

/// Main handler for Endpoint 0 (Control Endpoint)
pub fn handler_endpoint(epn: usize)
{
    let epr = match epn
    {
        0 => mcu::USB_EP0R as *mut u16,
        1 => mcu::USB_EP1R as *mut u16,
        2 => mcu::USB_EP2R as *mut u16,
        3 => mcu::USB_EP3R as *mut u16,
        4 => mcu::USB_EP4R as *mut u16,
        5 => mcu::USB_EP5R as *mut u16,
        6 => mcu::USB_EP6R as *mut u16,
        7 => mcu::USB_EP7R as *mut u16,
        _ => return
    };

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

/// Stalls both directions of Endpoint 0 (used for unsupported requests)
fn stall_ep(epn: usize)
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
        _ => return
    };

    unsafe
    {
        let mut val = core::ptr::read_volatile(ep);
        val ^= (usb_types::STATRX_Status::STALL as u16) << (usb_types::USBEPnR::STAT_TX as u8);
        val ^= (usb_types::STATRX_Status::STALL as u16) << (usb_types::USBEPnR::STAT_RX as u8);
        core::ptr::write_volatile(ep, val);
    }
}

/// Sets STAT_RX to VALID (toggles the bits)
fn set_stat_rx_valid(epn: usize)
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
        _ => return
    };

    unsafe
    {
        let mut val = core::ptr::read_volatile(ep);
        let stat_rx = (usb_types::STATRX_Status::VALID as u16) << (usb_types::USBEPnR::STAT_RX as u8);
        core::ptr::write_volatile(ep, val ^ stat_rx);
    }
}

fn set_stat_tx_nak(epn: usize)
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
        _ => return
    };

    unsafe
    {
        let val = core::ptr::read_volatile(ep);
        let stat_tx_nak = (usb_types::STATTX_Status::NAK as u16) << (usb_types::USBEPnR::STAT_TX as u8);
        core::ptr::write_volatile(ep, val ^ stat_tx_nak);
    }
}