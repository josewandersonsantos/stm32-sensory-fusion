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
        Usart1,
        Adc1
        /*
         *
         */
    }

    fn bit_of(peripheral: Apb2Peripheral) -> u8
    {
        match peripheral
        {
            Apb2Peripheral::Afio => 0,
            Apb2Peripheral::IoPa => 2,
            Apb2Peripheral::IoPb => 3,
            Apb2Peripheral::IoPc => 4,
            Apb2Peripheral::Usart1 => 14,
            Apb2Peripheral::Adc1 => 9,
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