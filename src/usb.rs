#![allow(dead_code)]

use crate::utils;
use crate::rcc;
use crate::mcu;

pub fn init()
{
    // 1. Enable USB clock
    rcc::apb1::enable(rcc::apb1::Apb1Peripheral::Usb);

    // 2. Reset USB peripheral
    let rcc_apb1rstr = mcu::RCC_APB1RSTR as *mut u32;

    utils::set_bit(rcc_apb1rstr, 23);   // USBRST
    utils::clear_bit(rcc_apb1rstr, 23);

    // 3. Clear power down
    let usb_cntr = mcu::USB_CNTR as *mut u32;

    utils::clear_bit(usb_cntr, 1); // PDWN = 0

    // 4. Wait a bit (important)
    delay();

    // 5. Enable pull-up (connect to host)
    let usb_bcdr = mcu::USB_BCDR as *mut u32;

    utils::set_bit(usb_bcdr, 15); // DPPU
}

fn delay()
{
    for _ in 0..10_000
    {
        unsafe { core::ptr::read_volatile(&0); }
    }
}

pub fn write(data :u8)
{
    
}
pub fn write_bytes(data: &[u8], len: u16)
{

}