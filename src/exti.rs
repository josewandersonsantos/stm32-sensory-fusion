use crate::{mcu::{self, RCC_APB2ENR}, utils};

pub fn enable_interrupt(pin: u32)
{
    let exti_imr = mcu::EXTI_IMR as *mut u32;
    unsafe { utils::write_bits(exti_imr, pin, 0x01) };
}

pub fn disable_interrupt(pin: u32)
{
    let exti_imr = mcu::EXTI_IMR as *mut u32;
    unsafe { utils::clear_bits(utils::read_register(exti_imr), 1 << pin) };
}

pub fn clear_pending_interrupt(pin: u32)
{
    let exti_pr = mcu::EXTI_PR as *mut u32;
    unsafe { utils::write_bits(exti_pr, pin, 0x01) };
}

pub fn configure_afio(port: u32, pin: u32)
{
    // Configure the system configuration controller to map the GPIO pin to the EXTI line
    let reg_offset = (pin / 4) * 4; // Each AFIO_EXTICR register covers 4 pins
    let afio_exticr = (mcu::AFIO_EXTICR1 + reg_offset) as *mut u32;
    let rcc_apb2_afio_en = (mcu::RCC_APB2ENR + mcu::RCC_APB2ENR_AFIOEN) as *mut u32;
    unsafe
    {
        utils::write_bits(rcc_apb2_afio_en, 0x00, 0x01);

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
    }

    pub fn set_edge(pin: u32, trigger: EdgeTrigger)
    {
        let exti_rtsr = mcu::EXTI_RTSR as *mut u32;
        let exti_ftsr = mcu::EXTI_FTSR as *mut u32;
        match trigger
        {
            EdgeTrigger::Falling => unsafe { utils::write_bits(exti_ftsr, pin, 0x01) },
            EdgeTrigger::Rising => unsafe { utils::write_bits(exti_rtsr, pin, 0x01) },
        }
    }
}