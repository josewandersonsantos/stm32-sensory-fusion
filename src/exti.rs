use crate::mcu; 
use crate::utils;

pub fn enable_interrupt(pin: u32)
{
    let exti_imr = mcu::EXTI_IMR as *mut u32;
    utils::set_bit(exti_imr, pin as u8);
}

pub fn disable_interrupt(pin: u32)
{
    let exti_imr = mcu::EXTI_IMR as *mut u32;
    utils::clear_bit(exti_imr, pin as u8);
}

pub fn clear_pending_interrupt(pin: u32)
{
    let exti_pr = mcu::EXTI_PR as *mut u32;
    utils::set_bit(exti_pr, pin as u8);
}

pub fn cfg_by_port(port: u32) -> u32
{
    match port
    {
        mcu::GPIOA_BASE => 0b0000, // GPIOA
        mcu::GPIOB_BASE => 0b0001, // GPIOB
        mcu::GPIOC_BASE => 0b0010, // GPIOC
        mcu::GPIOD_BASE => 0b0011, // GPIOD
        mcu::GPIOE_BASE => 0b0100, // GPIOE
        mcu::GPIOF_BASE => 0b0101, // GPIOF
        mcu::GPIOG_BASE => 0b0110, // GPIOG
        _ => panic!("Invalid port number"),
    }
}

pub fn configure_afio(port: u32, pin: u32)
{
    // Configure the system configuration controller to map the GPIO pin to the EXTI line
    let reg_offset = (pin / 4) * 4; // Each AFIO_EXTICR register covers 4 pins
    let afio_exticr = (mcu::AFIO_EXTICR1 + reg_offset) as *mut u32;
    unsafe
    {
        let mut value = utils::read_register(afio_exticr);
        value &= !(0x0F << ((pin % 4) * 4));
        value |= (port & 0x0F) << ((pin % 4) * 4); // Map the port to the pin
        utils::write_register(afio_exticr, value);
    }
}

pub mod gpio
{
    use super::*;
    pub enum EdgeTrigger
    {
        Falling,
        Rising,
        RisingFalling,
    }

    pub fn set_edge(pin: u32, trigger: EdgeTrigger)
    {
        let exti_rtsr = mcu::EXTI_RTSR as *mut u32;
        let exti_ftsr = mcu::EXTI_FTSR as *mut u32;
        match trigger
        {
            EdgeTrigger::Falling =>
            {
                utils::clear_bit(exti_ftsr,  pin as u8);
                utils::set_bit(exti_ftsr,  pin as u8);
            },
            
            EdgeTrigger::Rising =>
            {
                utils::clear_bit(exti_rtsr,  pin as u8);
                utils::set_bit(exti_rtsr,  pin as u8);
            },

            EdgeTrigger::RisingFalling =>
            {
                utils::set_bit(exti_rtsr,  pin as u8); 
                utils::set_bit(exti_ftsr,  pin as u8);
            }
        }
    }
}