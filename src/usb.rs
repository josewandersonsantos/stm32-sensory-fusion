#![allow(dead_code)]
#[allow(non_camel_case_types)]

use crate::irq;
use crate::usb;
use crate::utils;
use crate::rcc;
use crate::mcu;
use crate::usb_types;
use crate::usb_endpoint;

/// USB Low Priority Interrupt Handler
pub fn handle_usb_interrupt()
{
    usb_endpoint::handler_endpoint_interrupt();
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

pub fn reconnect()
{
    let usb_bcdr = mcu::USB_BCDR as *mut u16;
    // Disconnect by clearing DPPU
    utils::clear_bit16(usb_bcdr, usb_types::USBBCDR::DPPU as u8); // DPPU = 0

    // Small delay to ensure host detects disconnection
    utils::delay_ms(200);
    // Reconnect by setting DPPU
    utils::set_bit16(usb_bcdr, usb_types::USBBCDR::DPPU as u8); // DPPU = 1
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
    utils::clear_bit16(usb_bcdr, usb_types::USBBCDR::DPPU as u8); // DPPU = 0

    // Clear Power Down
    let usb_cntr = mcu::USB_CNTR as *mut u16;
    utils::clear_bit16(usb_cntr, usb_types::USBCNTR::PDWN as u8); // PDWN = 0
    
    // Small delay after waking up the peripheral
    utils::delay_ms(50);

    // Force Reset bits
    utils::clear_bit16(usb_cntr, usb_types::USBCNTR::FRES as u8); // FRES = 0

    unsafe 
    {
        let usb_istr = mcu::USB_ISTR as *mut u16;
        utils::write_register16(usb_istr, 0x0000);
    }

    // Setup BTABLE and Endpoint 0
    enable_btable();

    // Setup Endpoint 0
    usb_endpoint::configure_ep(usb_types::Endpoints::EP0, usb_types::EndpointType::Control);
    // Setup Endpoint 1
    // usb_endpoint::configure_ep(usb_types::Endpoints::EP1, usb_types::EndpointType::Control);
    // // Setup Endpoint 2
    // usb_endpoint::configure_ep(usb_types::Endpoints::EP2, usb_types::EndpointType::Control);

    //unsafe {utils::write_register16(usb_cntr, 0xFFFF);} // ALL
    // Enable Correct Transfer interrupt
    utils::set_bit16(usb_cntr, usb_types::USBCNTR::CTRM as u8); // CTRM
    // Enable Reset interrupt
    utils::set_bit16(usb_cntr, usb_types::USBCNTR::RESETM as u8); // RESETM
    // Enable Suspend interrupt
    utils::set_bit16(usb_cntr, usb_types::USBCNTR::SUSPM as u8); // SUSPM
    // Enable Wakeup interrupt
    utils::set_bit16(usb_cntr, usb_types::USBCNTR::WKUPM as u8); // WKUPM

    // Enable USB Low Priority interrupt in NVIC
    irq::enable_irq(irq::IRQn::USB_LP_CAN1_RX0 as u32);
    // irq::set_irq_priority(irq::IRQn::USB_LP_CAN1_RX0 as u32, 8);

    utils::delay_ms(50);

    // Connect to USB host by enabling internal pull-up on D+
    let usb_bcdr = mcu::USB_BCDR as *mut u16;
    utils::set_bit16(usb_bcdr, usb_types::USBBCDR::DPPU as u8); // DPPU = 1
    
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