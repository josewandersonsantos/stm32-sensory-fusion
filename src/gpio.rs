#![allow(dead_code)]

use crate::utils;
use crate::mcu;

#[derive(Copy, Clone)]
pub enum GpioMode
{
    Input,
    Output,
    AlternateFunction,
}

#[derive(Copy, Clone)]
pub enum GpioConfig
{
    Analog,           // CNF = 00 (input)
    Floating,         // CNF = 01 (input)
    PullUpDown,       // CNF = 10 (input)
    PushPull,         // CNF = 00 (output)
    OpenDrain,        // CNF = 01 (output)
    AfPushPull,       // CNF = 10 (AF)
    AfOpenDrain,      // CNF = 11 (AF)
}

#[derive(Copy, Clone)]
pub enum GpioSpeed
{
    Speed10MHz,
    Speed2MHz,
    Speed50MHz,
}

pub fn enable_gpio_clock(port_base: u32)
{
    let rcc_apb2enr = mcu::RCC_APB2ENR as *mut u32;
    unsafe
    {
        let current = utils::read_register(rcc_apb2enr);
        let new_value = current | (1 << port_index(port_base));
        utils::write_register(rcc_apb2enr, new_value);
    }
}

fn port_index(port_base: u32) -> u32
{
    match port_base
    {
        0x40010800 => 2, // GPIOA
        0x40010C00 => 3, // GPIOB
        0x40011000 => 4, // GPIOC
        0x40011400 => 5, // GPIOD
        _ => panic!("Invalid GPIO port base"),
    }
}

/// Combina os bits MODE[1:0] e CNF[1:0] conforme o modo escolhido
fn build_mode_bits(mode: GpioMode, config: GpioConfig, speed: Option<GpioSpeed>) -> u32
{
    let mode_bits = match mode
    {
        GpioMode::Input => 0b00,
        GpioMode::Output | GpioMode::AlternateFunction => match speed.unwrap()
        {
            GpioSpeed::Speed10MHz => 0b01,
            GpioSpeed::Speed2MHz => 0b10,
            GpioSpeed::Speed50MHz => 0b11,
        },
    };

    let cnf_bits = match (mode, config)
    {
        (GpioMode::Input, GpioConfig::Analog) => 0b00,
        (GpioMode::Input, GpioConfig::Floating) => 0b01,
        (GpioMode::Input, GpioConfig::PullUpDown) => 0b10,
        (GpioMode::Output, GpioConfig::PushPull) => 0b00,
        (GpioMode::Output, GpioConfig::OpenDrain) => 0b01,
        (GpioMode::AlternateFunction, GpioConfig::AfPushPull) => 0b10,
        (GpioMode::AlternateFunction, GpioConfig::AfOpenDrain) => 0b11,
        _ => panic!("Invalid mode/config combination"),
    };

    (cnf_bits << 2) | mode_bits
}

/// Aplica a configuração final no registrador CRL ou CRH
pub fn configure_pin(port_base: u32, pin: u32, mode: GpioMode, config: GpioConfig, speed: Option<GpioSpeed>)
{
    let shift = (pin % 8) * 4;
    let config_reg = if pin < 8
    {
        (port_base + 0x00) as *mut u32 // CRL
    }
    else
    {
        (port_base + 0x04) as *mut u32 // CRH
    };

    let mut mode_bits = build_mode_bits(mode, config, speed);
    unsafe
    {
        utils::write_bits(config_reg, shift, mode_bits);
    }
}
