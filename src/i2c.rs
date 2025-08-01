#![allow(dead_code)]

//use crate::utils;
use crate::mcu;

pub enum I2C
{
    I2C1,
    I2C2,
    I2C3
}
pub enum I2CMode
{
    Standard,
    Fast,
    FastPlus
}
pub enum I2CAddressingMode
{
    SevenBit,
    TenBit
}
pub enum I2CClockSpeed
{
    Standard100kHz,
    Fast400kHz
}
pub enum I2CDataFormat
{
    Data8Bit,
    Data16Bit
}

pub fn start(i2c: I2C, mode: I2CMode, addressing_mode: I2CAddressingMode, clock_speed: I2CClockSpeed, data_format: I2CDataFormat)
{
    let i2c_base = match i2c
    {
        I2C::I2C1 => mcu::I2C1_BASE,
        I2C::I2C2 => mcu::I2C2_BASE,
        I2C::I2C3 => mcu::I2C3_BASE,
    };

    // Configure the I2C peripheral based on the parameters
    // This is a placeholder for actual register configuration code
    // ...
}