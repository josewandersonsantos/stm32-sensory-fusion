#![allow(dead_code)]

use crate::utils;
use crate::mcu;
use crate::rcc;

#[derive(Clone, Copy)]
pub enum UsartMode
{
    Tx,
    Rx,
    TxRx,
}

#[derive(Clone, Copy)]
pub enum UsartBaudRate
{
    B9600,
    B115200,
    B230400,
    B460800,
}

#[derive(Clone, Copy)]
pub enum UsartWordLength
{
    Length8Bits,
    Length9Bits,
}

#[derive(Clone, Copy)]
pub enum UsartStopBits
{
    Stop1Bit,
    Stop2Bits,
}

#[derive(Clone, Copy)]
pub enum UsartParity
{
    None,
    Even,
    Odd,
}

#[derive(Clone, Copy)]
pub enum Usart
{
    Usart1,
    Usart2,
    Usart3,
}

pub enum UsartInterrupt
{
    TxInterrupt,
    RxInterrupt,
    None,
}

fn enable_it(usart: Usart, interrupt: UsartInterrupt)
{
    let usart_base = match usart
    {
        Usart::Usart1 => mcu::USART1_BASE,
        Usart::Usart2 => mcu::USART2_BASE,
        Usart::Usart3 => mcu::USART3_BASE,
    };

    let cr1 = (usart_base + mcu::USART_CR1) as *mut u32;

    unsafe
    {
        let mut cr1_val = utils::read_register(cr1);
        match interrupt
        {
            UsartInterrupt::TxInterrupt => cr1_val |= 1 << 7, // TXEIE
            UsartInterrupt::RxInterrupt => cr1_val |= 1 << 5, // RXNEIE
            UsartInterrupt::None => {}
        }
        utils::write_register(cr1, cr1_val);
    }
}

fn calculate_brr(baud_rate: UsartBaudRate, clock_freq: u32) -> u32
{
    match baud_rate
    {
        UsartBaudRate::B9600   => clock_freq / 9600,
        UsartBaudRate::B115200 => clock_freq / 115200,
        UsartBaudRate::B230400 => clock_freq / 230400,
        UsartBaudRate::B460800 => clock_freq / 460800,
    }
}

pub fn start( usart: Usart, mode: UsartMode, use_it:UsartInterrupt, baud_rate: UsartBaudRate, word_length: UsartWordLength, stop_bits: UsartStopBits, parity: UsartParity)
{
    let usart_base = match usart
    {
        Usart::Usart1 => mcu::USART1_BASE,
        Usart::Usart2 => mcu::USART2_BASE,
        Usart::Usart3 => mcu::USART3_BASE,
    };

    match usart
    {
        Usart::Usart1 => rcc::apb2::enable(rcc::apb2::Apb2Peripheral::Usart1),
        Usart::Usart2 => rcc::apb1::enable(rcc::apb1::Apb1Peripheral::Usart2),
        Usart::Usart3 => rcc::apb1::enable(rcc::apb1::Apb1Peripheral::Usart3),
    };

    let cr1 = (usart_base + mcu::USART_CR1) as *mut u32;
    let cr2 = (usart_base + mcu::USART_CR2) as *mut u32;
    let brr = (usart_base + mcu::USART_BRR) as *mut u32;

    unsafe
    {
        // Zera os registradores antes de configurar
        utils::write_register(cr1, 0);
        utils::write_register(cr2, 0);

        // Configura word length e parity (CR1)
        let mut cr1_val = match word_length
        {
            // M = 0
            UsartWordLength::Length8Bits => 0,
            // M = 1
            UsartWordLength::Length9Bits => 1 << 12,
        };

        cr1_val |= match parity
        {
            UsartParity::None => 0,
            // PCE = 1, PS = 0
            UsartParity::Even => 1 << 10,
            // PCE = 1, PS = 1
            UsartParity::Odd  => (1 << 10) | (1 << 9),
        };

        // Modo TX/RX
        cr1_val |= match mode
        {
            UsartMode::Tx   => 1 << 3, // TE
            UsartMode::Rx   => 1 << 2, // RE
            UsartMode::TxRx => (1 << 3) | (1 << 2),
        };

        // Habilita USART (UE = 1)
        cr1_val |= 1 << 13;

        utils::write_register(cr1, cr1_val);

        // Configura stop bits (CR2)
        let cr2_val = match stop_bits
        {
            UsartStopBits::Stop1Bit => 0b00 << 12,
            UsartStopBits::Stop2Bits => 0b10 << 12,
        };

        utils::write_register(cr2, cr2_val);

        // Configura baud rate (BRR)
        let brr_val = calculate_brr(baud_rate, mcu::CLOCK_FREQUENCY);
        utils::write_register(brr, brr_val);
    }

    enable_it(usart, use_it);
}

pub fn write(usart: Usart, data: u8)
{
    let usart_base = match usart
    {
        Usart::Usart1 => mcu::USART1_BASE,
        Usart::Usart2 => mcu::USART2_BASE,
        Usart::Usart3 => mcu::USART3_BASE,
    };

    let usart_dr = (usart_base + mcu::USART_DR) as *mut u32;
    let usart_sr = (usart_base + mcu::USART_SR) as *mut u32;

    unsafe
    {
        while (utils::read_register(usart_sr) & mcu::USART_SR_TXE) == 0 {}
        utils::write_register(usart_dr, data as u32);
    }
}