#![allow(dead_code)]
#[allow(non_camel_case_types)]

use crate::irq;
use crate::utils;
use crate::rcc;
use crate::mcu;

/// Packet Memory Area (PMA) base address in the USB peripheral
const PMA_BASE: u32 = 0x40006000;

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

#[repr(u8)]
enum UsbRequest
{
    GetDescriptor = 6,
}

enum USBCNTR
{
    FRES = 0,   // Force Reset
    PDWN = 1,   // Power Down
    LPMODE = 2, // Low-power mode
    FSUSP = 3,  // Force Suspend
    RESUME = 4, // Resume request
    ESOFM = 8,  // Start of Frame interrupt mask
    SOFM = 9,   // Start of Frame interrupt mask
    RESETM = 10, // USB Reset interrupt mask
    SUSPM = 11,  // Suspend mode interrupt mask
    WKUPM = 12,  // Wakeup interrupt mask
    ERRM = 13,   // Error interrupt mask
    PMAOVRM = 14, // Packet Memory Area Over/underrun interrupt mask
    CTRM = 15,  // Correct Transfer interrupt mask
}

enum USBFNR
{
    FN = 0,     // Frame Number (11 bits)
    LSOFL = 11, // Lost SOF HIGH
    LSOFH = 12, // Lost SOF HIGH
    LCK = 13,   // Locked
    RXDM = 14,  // Receive Data Minus (1 bit)
    RXDP = 15,  // Receive Data Plus (1 bit)
}
enum USBDADDR
{
    ADD = 0,    // Device Address (7 bits)
    EF = 7,     // Enable Function
}

enum USBBTABLE
{
    BTABLE = 3, // Base address of the buffer table (in 512-byte units)
}

enum USBEPnR
{
    EA = 0,     // Endpoint Address (4 bits)
    STAT_TX = 4, // Status bits for transmission
    DTOG_TX = 6, // Data Toggle for transmission
    CTR_TX = 7,  // Correct Transfer for transmission
    STAT_RX = 12, // Status bits for reception
    DTOG_RX = 14, // Data Toggle for reception
    CTR_RX = 15,  // Correct Transfer for reception
}

enum STATRX_Status
{
    VALID = 0,  // Valid
    NAK = 1,    // NAK
    STALL = 2,  // STALL
    DISABLED = 3, // Disabled
}

enum STATTX_Status
{
    VALID = 0,  // Valid
    NAK = 1,    // NAK
    STALL = 2,  // STALL
    DISABLED = 3, // Disabled
}

enum EndpointType
{
    Bulk = 0,
    Control = 1,
    Isochronous = 2,
    Interrupt = 3,
}

enum USBISTR
{
    ESOF = 8,   // Start of Frame
    SOF = 9,    // Start of Frame
    RESET = 10, // USB Reset
    SUSP = 11,  // Suspend mode
    WKUP = 12,  // Wakeup
    ERR = 13,   // Error
    PMAOVR = 14, // Packet Memory Area Over/underrun
    CTR = 15,   // Correct Transfer
}

enum USBBCDR
{
    DPPU = 15, // D+ Pull-up
}

/// Sets STAT_RX to VALID (toggles the bits)
fn set_stat_rx_valid()
{
    let ep = mcu::USB_EP0R as *mut u16;
    unsafe
    {
        let mut val = core::ptr::read_volatile(ep);
        // mantém CTR bits
        // val &= (1 << 15) | (1 << 7);
        // val ^= (0b11 << 12);       // Toggle STAT_RX bits
        // core::ptr::write_volatile(ep, val);
        core::ptr::write_volatile(ep, val ^ (0b11 << 12));
    }
}

fn set_stat_tx_nak()
{
    let ep = mcu::USB_EP0R as *mut u16;

    unsafe
    {
        let val = core::ptr::read_volatile(ep);
        core::ptr::write_volatile(ep, val ^ (0b10 << 4));
    }
}

/// Enables the Buffer Table (BTABLE) and clears EP0 register
fn enable_btable()
{
    // Set BTABLE address to 0x0000 (start of PMA)
    let usb_btable = mcu::USB_BTABLE as *mut u16;
    unsafe { core::ptr::write_volatile(usb_btable, 0x0000); }

    // Clear EP0R register
    let ep0r = mcu::USB_EP0R as *mut u16;
    unsafe { core::ptr::write_volatile(ep0r, 0x0000); }
}

/// Configures Endpoint 0 (Control Endpoint) buffers and registers
fn configure_ep0()
{
    unsafe
    {
        // === Buffer Description Table (BTABLE) entries for EP0 ===
        // BTABLE está em 0x0000 da PMA

        // TX Buffer (IN direction) - endereço recomendado: 0x40
        utils::write_register16((PMA_BASE + 0x00) as *mut u16, 0x40);   // ADDR_TX = 0x40
        utils::write_register16((PMA_BASE + 0x02) as *mut u16, 0x00);   // COUNT_TX = 0

        // RX Buffer (OUT/SETUP direction) - endereço recomendado: 0x80 (64 bytes após TX)
        utils::write_register16((PMA_BASE + 0x04) as *mut u16, 0x80);   // ADDR_RX = 0x80
        utils::write_register16((PMA_BASE + 0x06) as *mut u16, 0x8400); // COUNT_RX = 64 bytes (BL_SIZE=1, NUM_BLOCK=2)
    }

    // === Configure EP0R Register ===
    let ep0r = mcu::USB_EP0R as *mut u16;
    let daddr = mcu::USB_DADDR as *mut u16;
    unsafe
    {
        let mut val: u16 = 0;

        // Bits [3:0]  = EA[3:0]  → Endpoint Address = 0 (já é 0)
        // Bits [8:9]  = EP_TYPE  → 01 = Control
        val |= (0b01 << 9);

        // STAT_TX [5:4] = 01 = NAK     (não tem nada para enviar ainda)
        // STAT_RX [13:12] = 11 = VALID (pronto para receber SETUP)
        // val |= (0b01 << 4) | (0b11 << 12);

        core::ptr::write_volatile(ep0r, val);
        core::ptr::write_volatile(daddr, 0x0000);
    }

    set_stat_tx_nak();
    set_stat_rx_valid();
}

pub fn reconnect()
{
    let usb_bcdr = mcu::USB_BCDR as *mut u16;
    unsafe
    {
        // Disconnect by clearing DPPU
        utils::clear_bit16(usb_bcdr, 15); // DPPU = 0
    }

    // Small delay to ensure host detects disconnection
    utils::delay_ms(200);
    
    unsafe
    {
        // Reconnect by setting DPPU
        utils::set_bit16(usb_bcdr, 15); // DPPU = 1
    }
}

/// Initializes the USB peripheral on STM32F103 (BluePill)
pub fn init()
{
    // Enable USB clock
    rcc::apb1::enable(rcc::apb1::Apb1Peripheral::Usb);

    // Reset USB peripheral
    rcc::apb1::reset(rcc::apb1::Apb1Peripheral::Usb);

    // Discconnect to USB host by disabling internal pull-up on D+
    let usb_bcdr = mcu::USB_BCDR as *mut u16;
    utils::clear_bit16(usb_bcdr, USBBCDR::DPPU as u8); // DPPU = 0

    // Clear Power Down
    let usb_cntr = mcu::USB_CNTR as *mut u16;
    utils::clear_bit16(usb_cntr, USBCNTR::PDWN as u8); // PDWN = 0
    
    // Small delay after waking up the peripheral
    utils::delay_ms(50);

    // Force Reset bits
    utils::clear_bit16(usb_cntr, USBCNTR::FRES as u8); // FRES = 0

    unsafe 
    {
        let usb_istr = mcu::USB_ISTR as *mut u16;
        utils::write_register16(usb_istr, 0x0000);
    }

    // Setup BTABLE and Endpoint 0
    enable_btable();

    // Setup Endpoint 0
    configure_ep0();

    //unsafe {utils::write_register16(usb_cntr, 0xFFFF);} // ALL
    // Enable Correct Transfer interrupt
    utils::set_bit16(usb_cntr, USBCNTR::CTRM as u8); // CTRM
    // Enable Reset interrupt
    utils::set_bit16(usb_cntr, USBCNTR::RESETM as u8); // RESETM
    // Enable Suspend interrupt
    utils::set_bit16(usb_cntr, USBCNTR::SUSPM as u8); // SUSPM
    // Enable Wakeup interrupt
    utils::set_bit16(usb_cntr, USBCNTR::WKUPM as u8); // WKUPM

    // Enable USB Low Priority interrupt in NVIC
    irq::enable_irq(irq::IRQn::USB_LP_CAN1_RX0 as u32);
    // irq::set_irq_priority(irq::IRQn::USB_LP_CAN1_RX0 as u32, 8);

    utils::delay_ms(50);

    // Connect to USB host by enabling internal pull-up on D+
    let usb_bcdr = mcu::USB_BCDR as *mut u16;
    utils::set_bit16(usb_bcdr, USBBCDR::DPPU as u8); // DPPU = 1
    
    reconnect();

}

/// Placeholder for future serial write (CDC or custom)
pub fn write(data: u8)
{
    // TODO: Implement data transmission
}

/// Placeholder for future bulk write
pub fn write_bytes(data: &[u8], len: u16)
{
    // TODO: Implement data transmission
}