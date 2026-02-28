#![allow(dead_code)]

use crate::utils;
use crate::mcu;

pub mod apb1
{
    use super::*;
    pub enum Apb1Peripheral
    {
        Tim2,
        Tim3,
        Usart2,
        Usart3,
        I2C1,
        I2C2,
        Usb,
        /*
         *
         */
    }

    fn bit_of(peripheral: Apb1Peripheral) -> u8
    {
        match peripheral
        {
            Apb1Peripheral::Tim2 => 0,
            Apb1Peripheral::Tim3 => 1,
            Apb1Peripheral::Usart2 => 17,
            Apb1Peripheral::Usart3 => 18,
            Apb1Peripheral::I2C1 => 21,
            Apb1Peripheral::I2C2 => 22,
            Apb1Peripheral::Usb => 23,
            /* 
             *
             */
        }
    }

    pub fn enable(peripheral: Apb1Peripheral)
    {
        let bit = bit_of(peripheral);
        let reg = mcu::RCC_APB1ENR as *mut u32;
        utils::set_bit(reg, bit);
    }

    pub fn disable(peripheral: Apb1Peripheral)
    {
        let bit = bit_of(peripheral);
        let reg = mcu::RCC_APB1ENR as *mut u32;
        utils::clear_bit(reg, bit);
    }
}

pub mod apb2
{
    use super::*;

    pub enum Apb2Peripheral
    {
        Afio,
        IoPa,
        IoPb,
        IoPc,
        IoPd,
        IoPe,
        IoPf,
        IoPg,
        Adc1,
        Adc2,
        Tim1,
        Spi1,
        Tim8,
        Usart1,
        Adc3,
    }

    fn bit_of(peripheral: Apb2Peripheral) -> u8
    {
        match peripheral
        {
            Apb2Peripheral::Afio => 0,
            Apb2Peripheral::IoPa => 2,
            Apb2Peripheral::IoPb => 3,
            Apb2Peripheral::IoPc => 4,
            Apb2Peripheral::IoPd => 5,
            Apb2Peripheral::IoPe => 6,
            Apb2Peripheral::IoPf => 7,
            Apb2Peripheral::IoPg => 8,
            Apb2Peripheral::Adc1 => 9,
            Apb2Peripheral::Adc2 => 10,
            Apb2Peripheral::Tim1 => 11,
            Apb2Peripheral::Spi1 => 12,
            Apb2Peripheral::Tim8 => 13,
            Apb2Peripheral::Usart1 => 14,
            Apb2Peripheral::Adc3 => 15,
        }
    }

    pub fn enable(peripheral: Apb2Peripheral)
    {
        let bit = bit_of(peripheral);
        let reg = mcu::RCC_APB2ENR as *mut u32;
        utils::set_bit(reg, bit);
    }

    pub fn disable(peripheral: Apb2Peripheral)
    {
        let bit = bit_of(peripheral);
        let reg = mcu::RCC_APB2ENR as *mut u32;
        utils::clear_bit(reg, bit);
    }
}

pub fn get_pclk1_frequency() -> u32 
{
    let cfgr = unsafe { utils::read_register(mcu::RCC_CFGR as *mut u32) };

    // Get SYSCLK
    let sysclk = match (cfgr >> 2) & 0b11 
    {
        0b00 => 8_000_000, // HSI
        0b01 => 8_000_000, // HSE (Bluepill crystal)
        0b10 => 
        {
            // PLL
            let pllmul = ((cfgr >> 18) & 0b1111) + 2;
            8_000_000 * pllmul
        }
        _ => 8_000_000,
    };

    // AHB prescaler
    let hpre = (cfgr >> 4) & 0b1111;

    let hclk = match hpre 
    {
        0b0000..=0b0111 => sysclk,
        0b1000 => sysclk / 2,
        0b1001 => sysclk / 4,
        0b1010 => sysclk / 8,
        0b1011 => sysclk / 16,
        0b1100 => sysclk / 64,
        0b1101 => sysclk / 128,
        0b1110 => sysclk / 256,
        0b1111 => sysclk / 512,
        _ => sysclk,
    };

    // APB1 prescaler 
    let ppre1 = (cfgr >> 8) & 0b111;

    let pclk1 = match ppre1 
    {
        0b000..=0b011 => hclk,
        0b100 => hclk / 2,
        0b101 => hclk / 4,
        0b110 => hclk / 8,
        0b111 => hclk / 16,
        _ => hclk,
    };

    pclk1
}