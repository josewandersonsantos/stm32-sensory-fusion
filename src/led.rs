#![allow(dead_code)]

use crate::utils;
use crate::gpio;

pub fn led_init(port:u32, pin: u32)
{
    // Enable the GPIO clock for the port // Bit 4 = GPIOC enable
    gpio::enable_gpio_clock(0x04);
    // Set the pin as output with 10MHz speed and push-pull mode
    //gpio::set_mode_gpio(port, pin, gpio::MODE_OUTPUT_10MHZ_PP);
}

pub fn led_on(port: u32, pin: u32)
{
    let gpio_bsrr = (port + 0x10) as *mut u32; // BSRR
    unsafe
    {
        utils::write_register(gpio_bsrr, 1 << pin);
    }
}

pub fn led_off(port: u32, pin: u32)
{
    let gpio_bsrr = (port + 0x10) as *mut u32; // BSRR
    unsafe
    {
        utils::write_register(gpio_bsrr, 1 << (pin + 16)); // reset bit
    }
}

pub fn led_toggle(port: u32, pin: u32)
{
    let gpio_odr = (port + 0x0C) as *mut u32; // ODR
    unsafe
    {
        utils::toggle_register(gpio_odr, 1 << pin);
    }
}
