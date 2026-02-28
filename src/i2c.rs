#![allow(dead_code)]

use core::u32;

use crate::rcc;
use crate::utils;
use crate::mcu;

const I2C_READ_BIT: u8 = 1;
const I2C_WRITE_BIT: u8 = 0;

pub enum I2C
{
    I2C1,
    I2C2,
}
pub enum I2CMode
{
    Standard,
    Fast,
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
pub enum I2C_CR1
{
    PE       = 0,  // Peripheral Enable
    SMBUS    = 1,  // SMBus Mode
    SMBTYPE  = 3,  // SMBus Type
    ENARP    = 4,  // ARP Enable
    ENPEC    = 5,  // PEC Enable
    ENGC     = 6,  // General Call Enable
    NOSTRETCH= 7,  // Clock Stretching Disable
    START    = 8,  // Start Generation
    STOP     = 9,  // Stop Generation
    ACK      = 10, // Acknowledge Enable
    POS      = 11, // Acknowledge/PEC Position (for data reception)
    PEC      = 12, // Packet Error Checking
    ALERT    = 13, // SMBus Alert
    SWRST    = 15, // Software Reset
} 
pub enum I2C_SR1
{
    SB       = 0,  // Start Bit
    ADDR     = 1,  // Address Sent/Matched
    BTF      = 2,  // Byte Transfer 
    ADD10    = 3,  // 10-bit Address Header Sent
    STOPF    = 4,  // Stop 
    RXNE     = 6,  // Receive Data Register Not Empty
    TXE      = 7,  // Transmit Data Register Empty
    BERR     = 8,  // Bus Error
    ARLO     = 9,  // Arbitration Lost
    AF       = 10, // Acknowledge Failure
    OVR      = 11, // Overrun/Underrun
    PECERR   = 12, // PEC Error in Reception
    TIMEOUT  = 14, // Timeout or Tlow Error
    SMBALERT = 15, // SMBus Alert
}

pub enum I2C_SR2
{
    MSL     = 0,  // Master/Slave
    BUSY    = 1,  // Bus Busy
    TRA     = 2,  // Transmitter/Receiver
    GENCALL = 4,  // General Call Address (Slave Mode)
    DUALF   = 7,  // Dual Flag (Slave Mode)
    PEC01   = 8,  // Packet Error Checking Register
}

fn get_base(i2c: &I2C) -> u32
{
    match i2c
    {
        I2C::I2C1 => mcu::I2C1_BASE,
        I2C::I2C2 => mcu::I2C2_BASE,
    }
}

fn reg(base: u32, offset: u32) -> *mut u32
{
    (base + offset) as *mut u32
}

fn start_condition(i2c_base: u32)
{
    // while utils::read_bit(reg(i2c_base, mcu::I2C_SR2), I2C_SR2::BUSY as u8) == 1 {}
    utils::set_bit(reg(i2c_base, mcu::I2C_CR1), I2C_CR1::START as u8); // START
    while utils::read_bit(reg(i2c_base, mcu::I2C_SR1), I2C_SR1::SB as u8) == 0 {}
}

fn stop_condition(i2c_base: u32)
{
    utils::set_bit(reg(i2c_base, mcu::I2C_CR1), I2C_CR1::STOP as u8); // STOP
}

pub fn start(i2c: I2C, mode: I2CMode, addressing_mode: I2CAddressingMode, clock_speed: I2CClockSpeed, data_format: I2CDataFormat)
{
    let i2c_base = match i2c
    {
        I2C::I2C1 => mcu::I2C1_BASE,
        I2C::I2C2 => mcu::I2C2_BASE,
    };

    // Enable I2C clock
    match i2c
    {
        I2C::I2C1 => rcc::apb1::enable(rcc::apb1::Apb1Peripheral::I2C1),
        I2C::I2C2 => rcc::apb1::enable(rcc::apb1::Apb1Peripheral::I2C2)
    }

    unsafe 
    {
        // Disable the I2C peripheral before configuration
        utils::clear_bit(reg(i2c_base, mcu::I2C_CR1), I2C_CR1::PE as u8);
        
        // ====== PCLK1 = 8 MHz ======
        // CR2: frequência do APB1 em MHz
        let pclk1 = rcc::get_pclk1_frequency();
        let pclk1_mhz = pclk1 / 1_000_000;
        
        utils::write_register(reg(i2c_base, mcu::I2C_CR2), pclk1_mhz as u32);

        match clock_speed
        {
            I2CClockSpeed::Standard100kHz =>
            {
                // CCR = 8_000_000 / (2 * 100_000) = 40
                // utils::write_register(reg(i2c_base, mcu::I2C_CCR), 40);
                let ccr = pclk1 / (2 * 100_000);
                utils::write_register(reg(i2c_base, mcu::I2C_CCR), ccr);

                // TRISE = PCLK1_MHz + 1 = 8 + 1 = 9
                // utils::write_register(reg(i2c_base, mcu::I2C_TRISE), 9);
                utils::write_register(reg(i2c_base, mcu::I2C_TRISE), pclk1_mhz + 1);
            }

            I2CClockSpeed::Fast400kHz =>
            {
                // Modo Fast (duty = 0 → 2)
                // CCR = 8_000_000 / (3 * 400_000) ≈ 6.66 → 7
                let ccr = 7 | (1 << 15); // Set FAST bit
                utils::write_register(reg(i2c_base, mcu::I2C_CCR), ccr);

                // TRISE (Fast mode)
                // 300ns max rise time
                // TRISE = (300ns / T_PCLK1) + 1
                // T_PCLK1 = 1 / 8MHz = 125ns
                // 300 / 125 = 2.4 → 3 + 1 = 4
                utils::write_register(reg(i2c_base, mcu::I2C_TRISE), 4);
            }
        }

        // Enable peripheral
        utils::set_bit(reg(i2c_base, mcu::I2C_CR1), I2C_CR1::PE as u8);
    }
}

// pub fn get_status(i2c:I2C, bit:I2CStatusBit)
// {
//     let i2c_base = match i2c
//     {
//         I2C::I2C1 => mcu::I2C1_BASE,
//         I2C::I2C2 => mcu::I2C2_BASE,
//     };

//     let sr1 = (i2c_base + mcu::I2C_SR1) as *const u32;
//     let sr2 = (i2c_base + mcu::I2C_SR2) as *const u32;

//     unsafe
//     {
//         let status1 = utils::read_register(sr1);
//         let status2 = utils::read_register(sr2);
//     }
// }

pub mod slave { }

pub mod master
{
    use super::*;

    pub fn write_register8(i2c: &I2C, device_addr: u8, reg_addr: u8, value: u8)
    {
        let i2c_base = get_base(i2c);

        unsafe
        {
            start_condition(i2c_base);

            // Write device address with write bit (0)
            utils::write_register(reg(i2c_base, mcu::I2C_DR), ((device_addr << 1) | I2C_WRITE_BIT) as u32);

            // Wait ADDR
            while utils::read_bit(reg(i2c_base, mcu::I2C_SR1), I2C_SR1::ADDR as u8) == 0 {}

            // Clear ADDR
            let _ = utils::read_register(reg(i2c_base, mcu::I2C_SR1));
            let _ = utils::read_register(reg(i2c_base, mcu::I2C_SR2));

            // Wait TXE
            while utils::read_bit(reg(i2c_base, mcu::I2C_SR1), I2C_SR1::TXE as u8) == 0 {}

            // Send register address
            utils::write_register(reg(i2c_base, mcu::I2C_DR), reg_addr as u32);

            // Wait TXE
            while utils::read_bit(reg(i2c_base, mcu::I2C_SR1), I2C_SR1::TXE as u8) == 0 {}

            // Send data
            utils::write_register(reg(i2c_base, mcu::I2C_DR), value as u32);

            // Wait BTF
            while utils::read_bit(reg(i2c_base, mcu::I2C_SR1), I2C_SR1::BTF as u8) == 0 {}

            stop_condition(i2c_base);
        }
    }
    
    pub fn read_register8(i2c: &I2C, device_addr: u8, reg_addr: u8) -> u8
    {
        let i2c_base = get_base(i2c);

        unsafe
        {
            // ---- Write register address ----
            start_condition(i2c_base);

            utils::write_register(reg(i2c_base, mcu::I2C_DR), ((device_addr << 1) | I2C_WRITE_BIT) as u32);

            while utils::read_bit(reg(i2c_base, mcu::I2C_SR1), I2C_SR1::ADDR as u8) == 0 {}

            let _ = utils::read_register(reg(i2c_base, mcu::I2C_SR1));
            let _ = utils::read_register(reg(i2c_base, mcu::I2C_SR2));

            while utils::read_bit(reg(i2c_base, mcu::I2C_SR1), I2C_SR1::TXE as u8) == 0 {}

            utils::write_register(reg(i2c_base, mcu::I2C_DR), reg_addr as u32);

            while utils::read_bit(reg(i2c_base, mcu::I2C_SR1), I2C_SR1::BTF as u8) == 0 {}

            // ---- Repeated START ----
            start_condition(i2c_base);

            utils::write_register(reg(i2c_base, mcu::I2C_DR), ((device_addr << 1) | I2C_READ_BIT) as u32);

            while utils::read_bit(reg(i2c_base, mcu::I2C_SR1), I2C_SR1::ADDR as u8) == 0 {}
            
            let _ = utils::read_register(reg(i2c_base, mcu::I2C_SR1));
            let _ = utils::read_register(reg(i2c_base, mcu::I2C_SR2));
            
            utils::clear_bit(reg(i2c_base, mcu::I2C_CR1), I2C_CR1::ACK as u8); // ACK = 0

            stop_condition(i2c_base);

            while utils::read_bit(reg(i2c_base, mcu::I2C_SR1), I2C_SR1::RXNE as u8) == 0 {}

            let value = utils::read_register(reg(i2c_base, mcu::I2C_DR)) as u8;

            // Wait STOPF
            // while utils::read_bit(reg(i2c_base, mcu::I2C_SR1), I2C_SR1::STOPF as u8) == 0 {}

            value
        }
    }
}