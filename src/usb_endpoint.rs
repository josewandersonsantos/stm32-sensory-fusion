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

/// Global state for Endpoint 0 (Control Endpoint)
static mut EP0_STATE: Ep0State = Ep0State::Idle;
static mut EP0_DATA: [u8; 64] = [0; 64];   // Buffer for EP0 data stage
static mut EP0_LEN: usize = 0;              // Total length to transfer
static mut EP0_POS: usize = 0;              // Current position in the buffer

/// Standard USB Device Descriptor (minimal, for testing)
const DEVICE_DESCRIPTOR: [u8; 18] =
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
];

/// Configures Endpoint 0 (Control Endpoint) buffers and registers
pub fn configure_ep0(usb_types: &usb_types::Endpoints)
{
    unsafe
    {
        // === Buffer Description Table (BTABLE) entries for EP0 ===
        // BTABLE está em 0x0000 da PMA

        // TX Buffer (IN direction) - endereço recomendado: 0x40
        utils::write_register16((usb_types::PMA_BASE + 0x00) as *mut u16, 0x40);   // ADDR_TX = 0x40
        utils::write_register16((usb_types::PMA_BASE + 0x02) as *mut u16, 0x00);   // COUNT_TX = 0

        // RX Buffer (OUT/SETUP direction) - endereço recomendado: 0x80 (64 bytes após TX)
        utils::write_register16((usb_types::PMA_BASE + 0x04) as *mut u16, 0x80);   // ADDR_RX = 0x80
        utils::write_register16((usb_types::PMA_BASE + 0x06) as *mut u16, 0x8400); // COUNT_RX = 64 bytes (BL_SIZE=1, NUM_BLOCK=2)
    }

    // === Configure EP0R Register ===
    let ep0r = mcu::USB_EP0R as *mut u16;
    let daddr = mcu::USB_DADDR as *mut u16;
    unsafe
    {
        let mut val: u16 = 0;

        // Bits [3:0]  = EA[3:0]  → Endpoint Address = 0
        // Bits [8:9]  = EP_TYPE  → 01 = Control
        val |= (usb_types::EndpointType::Control as u16) << (usb_types::USBEPnR::EA as u8);
        core::ptr::write_volatile(ep0r, val);
        core::ptr::write_volatile(daddr, 0x0000);
    }

    set_stat_tx_nak();
    set_stat_rx_valid();
}

/// USB Low Priority Interrupt Handler
pub fn handler_endpoint_interrupt()
{
    unsafe
    {
        let usb_istr = mcu::USB_ISTR as *mut u16;
        let mut istr = utils::read_register16(usb_istr);

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
            // Extract endpoint number
            let ep_id = istr & 0x0F;
            
            match ep_id
            {
                // Reset EP0
                0 => ep0_handler(),
                _ => {}
            }

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
            // Extract endpoint number
            let ep_id = istr & 0x0F;

            match ep_id
            {
                0 => ep0_handler(),
                _ => {}
            }
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
fn send_next_packet()
{
    let addr_tx = 0x40;     // TX buffer address in PMA (defined in configure_ep0)

    let chunk = unsafe
    {
        let remaining = EP0_LEN - EP0_POS;
        remaining.min(64)
    };
    
    unsafe
    {
        // Copy data to PMA
        pma_write(addr_tx, &EP0_DATA[EP0_POS..EP0_POS + chunk]);

        EP0_POS += chunk;
    }

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
fn handle_get_descriptor(setup: &[u8])
{
    let desc_type = setup[3];

    let data = match desc_type
    {
        1 => &DEVICE_DESCRIPTOR,        // Device Descriptor
        _ =>
        {
            stall_ep0();
            return;
        }
    };

    unsafe
    {
        // Limit response size to what the host requested
        EP0_LEN = core::cmp::min(setup[6] as usize, data.len());
        EP0_POS = 0;

        EP0_DATA[..EP0_LEN].copy_from_slice(&data[..EP0_LEN]);

        EP0_STATE = Ep0State::DataIn;
    }

    send_next_packet();
}

/// Handles SETUP packets (Standard Device Requests)
fn handle_setup()
{
    let mut setup = [0u8; 8];

    // Read 8-byte SETUP packet from PMA
    pma_read(0x80, &mut setup);     // RX buffer starts at 0x80

    unsafe
    {
        EP0_STATE = Ep0State::Setup;
    }

    let request = setup[1];     // bRequest

    match request
    {
        6 => 
        {
            handle_get_descriptor(&setup)     // GET_DESCRIPTOR
        },
        _ => stall_ep0(),                       // Unsupported request → STALL
    }
}
/// Called when an IN transaction completes
fn handle_in()
{
    unsafe
    {
        match EP0_STATE
        {
            Ep0State::DataIn =>
            {
                if EP0_POS < EP0_LEN
                {
                    // More data to send
                    send_next_packet();
                } 
                else
                {
                    // Data stage finished → go to Status OUT stage
                    EP0_STATE = Ep0State::StatusOut;
                    set_stat_rx_valid();
                }
            }
            Ep0State::StatusIn =>
            {
                // Status stage completed
                EP0_STATE = Ep0State::Idle;
            }
            _ => {}
        }
    }
}

/// Called when an OUT transaction completes
fn handle_out()
{
    unsafe
    {
        match EP0_STATE
        {
            Ep0State::StatusOut =>
            {
                // Status stage completed
                EP0_STATE = Ep0State::Idle;
            }
            _ => {}
        }
    }
}

/// Main handler for Endpoint 0 (Control Endpoint)
pub fn ep0_handler()
{
    let ep0r = mcu::USB_EP0R as *mut u16;
    let ep = unsafe { core::ptr::read_volatile(ep0r) };

    // ========================
    // RX Side (SETUP or OUT packet received)
    // ========================
    if ep & (1 << usb_types::USBEPnR::CTR_RX as u16) != 0 // CTR_RX flag set
    {
        if ep & (1 << usb_types::USBEPnR::SETUP as u16) != 0 // SETUP bit set
        {
            handle_setup();
        }
        else
        {
            // Regular OUT data packet
            handle_out();
        }

        clear_ctr_rx(ep0r);
    }

    // ========================
    // TX Side (IN packet transmission completed)
    // ========================
    if ep & (1 << usb_types::USBEPnR::CTR_TX as u16) != 0
    {
        handle_in();
        clear_ctr_tx(ep0r);
    }
}

/// Stalls both directions of Endpoint 0 (used for unsupported requests)
fn stall_ep0()
{
    let ep = mcu::USB_EP0R as *mut u16;
    unsafe
    {
        let mut val = core::ptr::read_volatile(ep);
        val ^= (usb_types::STATRX_Status::STALL as u16) << (usb_types::USBEPnR::STAT_TX as u8);
        val ^= (usb_types::STATRX_Status::STALL as u16) << (usb_types::USBEPnR::STAT_RX as u8);
        core::ptr::write_volatile(ep, val);
    }
}

/// Sets STAT_RX to VALID (toggles the bits)
fn set_stat_rx_valid()
{
    let ep = mcu::USB_EP0R as *mut u16;
    unsafe
    {
        let mut val = core::ptr::read_volatile(ep);
        let stat_rx = (usb_types::STATRX_Status::VALID as u16) << (usb_types::USBEPnR::STAT_RX as u8);
        core::ptr::write_volatile(ep, val ^ stat_rx);
    }
}

fn set_stat_tx_nak()
{
    let ep = mcu::USB_EP0R as *mut u16;

    unsafe
    {
        let val = core::ptr::read_volatile(ep);
        let stat_tx_nak = (usb_types::STATTX_Status::NAK as u16) << (usb_types::USBEPnR::STAT_TX as u8);
        core::ptr::write_volatile(ep, val ^ stat_tx_nak);
    }
}