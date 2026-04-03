#![allow(dead_code)]

use crate::utils;

const NVIC_BASE: u32 = 0xE000_E100; // NVIC base address
const NVIC_ISER: u32 = NVIC_BASE + 0x00; // Interrupt Set-Enable Register
const NVIC_ICER: u32 = NVIC_BASE + 0x80; // Interrupt Clear-Enable Register
const NVIC_IPR: u32  = NVIC_BASE + 0x400; // Interrupt Priority Register

#[allow(non_camel_case_types)]
pub enum IRQn
{
    WWDG = 0,              // Window Watchdog interrupt
    PVD,                   // PVD through EXTI Line detection interrupt
    TAMPER,                // Tamper interrupt
    RTC,                   // RTC global interrupt
    FLASH,                 // Flash global interrupt
    RCC,                   // RCC global interrupt
    EXTI0,                 // EXTI Line0 interrupt
    EXTI1,                 // EXTI Line1 interrupt
    EXTI2,                 // EXTI Line2 interrupt
    EXTI3,                 // EXTI Line3 interrupt
    EXTI4,                 // EXTI Line4 interrupt
    DMA1_CHANNEL1,         // DMA1 Channel 1 interrupt
    DMA1_CHANNEL2,         // DMA1 Channel 2 interrupt
    DMA1_CHANNEL3,         // DMA1 Channel 3 interrupt
    DMA1_CHANNEL4,         // DMA1 Channel 4 interrupt
    DMA1_CHANNEL5,         // DMA1 Channel 5 interrupt
    DMA1_CHANNEL6,         // DMA1 Channel 6 interrupt
    DMA1_CHANNEL7,         // DMA1 Channel 7 interrupt
    ADC1_2,                // ADC1 and ADC2 global interrupt
    USB_HP_CAN1_TX,        // USB High Priority or CAN1 TX interrupts
    USB_LP_CAN1_RX0,       // USB Low Priority or CAN1 RX0 interrupts
    CAN1_RX1,              // CAN1 RX1 interrupt
    CAN1_SCE,              // CAN1 SCE interrupt
    EXTI9_5,               // EXTI Line[9:5] interrupts
    TIM1_BRK,              // TIM1 Break interrupt
    TIM1_UP,               // TIM1 Update interrupt
    TIM1_TRG_COM,          // TIM1 Trigger and Commutation interrupt
    TIM1_CC,               // TIM1 Capture Compare interrupt
    TIM2,                  // TIM2 global interrupt
    TIM3,                  // TIM3 global interrupt
    TIM4,                  // TIM4 global interrupt
    I2C1_EV,               // I2C1 Event interrupt
    I2C1_ER,               // I2C1 Error interrupt
    I2C2_EV,               // I2C2 Event interrupt
    I2C2_ER,               // I2C2 Error interrupt
    SPI1,                  // SPI1 global interrupt
    SPI2,                  // SPI2 global interrupt
    USART1,                // USART1 global interrupt
    USART2,                // USART2 global interrupt
    USART3,                // USART3 global interrupt
    EXTI15_10,             // EXTI Line[15:10] interrupts
    RTC_ALARM,             // RTC Alarm through EXTI Line interrupt
    USB_WAKEUP,            // USB Wakeup from suspend through EXTI Line interrupt
}

pub fn from_pin(pin: u32) -> Option<u32>
{
    match pin
    {
        0 => Some(IRQn::EXTI0 as u32),
        1 => Some(IRQn::EXTI1 as u32),
        2 => Some(IRQn::EXTI2 as u32),
        3 => Some(IRQn::EXTI3 as u32),
        4 => Some(IRQn::EXTI4 as u32),
        5..=9 => Some(IRQn::EXTI9_5 as u32),
        10..=15 => Some(IRQn::EXTI15_10 as u32),
        _ => None,
    }
}

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