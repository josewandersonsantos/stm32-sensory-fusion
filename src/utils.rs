#![allow(dead_code)]

use crate::{mcu, rcc, utils};

pub fn delay_ms(ms: u32)
{
    let ticks = (rcc::get_pclk1_frequency() / 1000) * ms;

    unsafe
    {
        let systick_ctrl = mcu::SYSTICK_CTRL as *mut u32;
        let systick_load = mcu::SYSTICK_LOAD as *mut u32;
        let systick_val  = mcu::SYSTICK_VAL  as *mut u32;

        utils::write_register(systick_load, ticks - 1);
        utils::write_register(systick_val, 0);
        utils::write_register(systick_ctrl, 0b101); // enable + core clock

        while (utils::read_bit(systick_ctrl, 16)) == 0 {}

        utils::write_register(systick_ctrl, 0); // stop
    }
}

/// Lê um valor de 32 bits de um endereço de registrador
pub unsafe fn read_register(addr: *const u32) -> u32
{
    core::ptr::read_volatile(addr)
}

/// Escreve um valor de 32 bits em um endereço de registrador
pub unsafe fn write_register(addr: *mut u32, value: u32)
{
    core::ptr::write_volatile(addr, value)
}

pub fn read_bit(register: *mut u32, bit: u8) -> u8
{
    unsafe
    {
        let value = read_register(register);
        ((value >> bit) & 1) as u8
    }
}

pub fn set_bit(register: *mut u32, bit: u8)
{
    unsafe
    {
        let value = read_register(register);
        write_register(register, value | (1 << bit));
    }
}

pub fn clear_bit(register: *mut u32, bit: u8)
{
    unsafe
    {
        let value = read_register(register);
        write_register(register, value & !(1 << bit));
    }
}

/// Inverte os bits especificados pela máscara no registrador
pub unsafe fn toggle_register(addr: *mut u32, mask: u32)
{
    let current = read_register(addr as *const u32);
    let toggled = current ^ mask;
    write_register(addr, toggled);
}

/// Escreve 4 bits no registrador, após limpar os bits naquela posição
pub unsafe fn write_bits(addr: *mut u32, shift: u32, value: u32)
{
    let current = read_register(addr as *const u32); // Cast necessário aqui
    let mask = !(0xF << shift);
    let updated = (current & mask) | ((value & 0xF) << shift);
    write_register(addr, updated);
}
