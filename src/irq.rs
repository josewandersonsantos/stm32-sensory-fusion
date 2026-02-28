#![allow(dead_code)]

use crate::utils;

const NVIC_BASE: u32 = 0xE000_E100; // NVIC base address
const NVIC_ISER: u32 = NVIC_BASE + 0x00; // Interrupt Set-Enable Register
const NVIC_ICER: u32 = NVIC_BASE + 0x80; // Interrupt Clear-Enable Register
const NVIC_IPR: u32  = NVIC_BASE + 0x400; // Interrupt Priority Register

pub fn enable_irq(pin: u32)
{
    let reg_offset = (pin / 32) * 4;
    let bit_pos = pin % 32;
    let nvic_iser = (NVIC_ISER + reg_offset) as *mut u32;
    
    unsafe
    {
        let current = utils::read_register(nvic_iser);
        let new_value = current | (1 << bit_pos);
        utils::write_register(nvic_iser, new_value);
    }
}

pub fn disable_irq(pin: u32)
{
    let reg_offset = (pin / 32) * 4;
    let bit_pos = pin % 32;
    let nvic_icer = (NVIC_ICER + reg_offset) as *mut u32;
    
    unsafe
    {
        let current = utils::read_register(nvic_icer);
        let new_value = current & !(1 << bit_pos);
        utils::write_register(nvic_icer, new_value);
    }
}