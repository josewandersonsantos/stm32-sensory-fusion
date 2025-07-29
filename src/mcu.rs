// This file defines the memory-mapped addresses for GPIO ports and specific pins
// for STM32F103C8T6.

pub const AFIO_BASE: u32 = 0x4001_0000; // AFIO base address
pub const AFIO_EXTICR1: u32 = AFIO_BASE + 0x08; // AFIO_EXTICR1 offset
pub const AFIO_EXTICR2: u32 = AFIO_BASE + 0x0C; // AFIO_EXTICR2 offset
pub const AFIO_EXTICR3: u32 = AFIO_BASE + 0x10; // AFIO_EXTICR3 offset
pub const AFIO_EXTICR4: u32 = AFIO_BASE + 0x14; // AFIO_EXTICR4 offset

pub const GPIO00:u32 = 0x00;
pub const GPIO01:u32 = 0x01;
pub const GPIO02:u32 = 0x02;
pub const GPIO03:u32 = 0x03;
pub const GPIO04:u32 = 0x04;
pub const GPIO05:u32 = 0x05;
pub const GPIO06:u32 = 0x06;
pub const GPIO07:u32 = 0x07;
pub const GPIO08:u32 = 0x08;
pub const GPIO09:u32 = 0x09;
pub const GPIO10:u32 = 0x0A;
pub const GPIO11:u32 = 0x0B;
pub const GPIO12:u32 = 0x0C;
pub const GPIO13:u32 = 0x0D;
pub const GPIO14:u32 = 0x0E;
pub const GPIO15:u32 = 0x0F;

pub const PORTA_ADDR: u32 = 0x4001_0800; // GPIOA base address
pub const PORTB_ADDR: u32 = 0x4001_0C00; // GPIOB base address
pub const PORTC_ADDR: u32 = 0x4001_1000; // GPIOC base address
pub const PORTD_ADDR: u32 = 0x4001_1400; // GPIOD base address
pub const PORTE_ADDR: u32 = 0x4001_1800; // GPIOE base address
pub const PORTF_ADDR: u32 = 0x4001_1C00; // GPIOF base address
pub const PORTG_ADDR: u32 = 0x4001_2000; // GPIOG base address

pub const RCC_ADDR: u32 = 0x4002_1000; // RCC base address
pub const RCC_APB2ENR: u32 = RCC_ADDR + 0x18; // RCC_APB2ENR offset
pub const RCC_APB2ENR_AFIOEN: u32 = 0x0000_0000; // AFIO clock enable
pub const RCC_APB2ENR_RESERVED: u32 = 0x0000_0001; // Reserved
pub const RCC_APB2ENR_IOPAEN: u32 = 0x0000_0002; // GPIOA clock enable
pub const RCC_APB2ENR_IOPBEN: u32 = 0x0000_0003; // GPIOB clock enable
pub const RCC_APB2ENR_IOPCEN: u32 = 0x0000_0004; // GPIOC clock enable
pub const RCC_APB2ENR_IOPDEN: u32 = 0x0000_0008; // GPIOD clock enable
pub const RCC_APB2ENR_IOPEEN: u32 = 0x0000_0010; // GPIOE clock enable
pub const RCC_APB2ENR_IOPFEN: u32 = 0x0000_0020; // GPIOF clock enable
pub const RCC_APB2ENR_IOPGEN: u32 = 0x0000_0040; // GPIOG clock enable
pub const RCC_APB2ENR_ADC1EN: u32 = 0x0000_0200; // ADC1 clock enable
pub const RCC_APB2ENR_ADC2EN: u32 = 0x0000_0800; // ADC2 clock enable
pub const RCC_APB2ENR_ADC3EN: u32 = 0x0000_1000; // ADC3 clock enable
pub const RCC_APB2ENR_USART1EN: u32 = 0x0000_4000; // USART1 clock enable
pub const RCC_APB2ENR_USART2EN: u32 = 0x0000_8000; // USART2 clock enable
pub const RCC_APB2ENR_USART3EN: u32 = 0x0001_0000; // USART3 clock enable
pub const RCC_APB2ENR_SPI1EN: u32 = 0x0002_0000; // SPI1 clock enable
pub const RCC_APB2ENR_SPI2EN: u32 = 0x0004_0000; // SPI2 clock enable
pub const RCC_APB2ENR_I2C1EN: u32 = 0x0008_0000; // I2C1 clock enable
pub const RCC_APB2ENR_I2C2EN: u32 = 0x0010_0000; // I2C2 clock enable
pub const RCC_APB2ENR_TIM1EN: u32 = 0x0020_0000; // TIM1 clock enable
pub const RCC_APB2ENR_TIM2EN: u32 = 0x0040_0000; // TIM2 clock enable
pub const RCC_APB2ENR_TIM3EN: u32 = 0x0080_0000; // TIM3 clock enable
pub const RCC_APB2ENR_TIM4EN: u32 = 0x0100_0000; // TIM4 clock enable
pub const RCC_APB2ENR_TIM5EN: u32 = 0x0200_0000; // TIM5 clock enable
pub const RCC_APB2ENR_TIM6EN: u32 = 0x0400_0000; // TIM6 clock enable
pub const RCC_APB2ENR_TIM7EN: u32 = 0x0800_0000; // TIM7 clock enable
pub const RCC_APB2ENR_TIM8EN: u32 = 0x1000_0000; // TIM8 clock enable
pub const RCC_APB2ENR_DMA1EN: u32 = 0x2000_0000; // DMA1 clock enable
pub const RCC_APB2ENR_DMA2EN: u32 = 0x4000_0000; // DMA2 clock enable
pub const RCC_APB2ENR_USBEN: u32 = 0x8000_0000; // USB clock enable
pub const RCC_APB2ENR_CANEN: u32 = 0x0000_0001; // CAN clock enable
pub const RCC_APB2ENR_SDIOEN: u32 = 0x0000_0002; // SDIO clock enable
pub const RCC_APB2ENR_FSMCEN: u32 = 0x0000_0004; // FSMC clock enable
pub const RCC_APB2ENR_ETHMACEN: u32 = 0x0000_0008; // Ethernet MAC clock enable
pub const RCC_APB2ENR_ETHMACTXEN: u32 = 0x0000_0010; // Ethernet MAC TX clock enable
pub const RCC_APB2ENR_ETHMACRXEN: u32 = 0x0000_0020; // Ethernet MAC RX clock enable
pub const RCC_APB2ENR_ETHMACPTPEN: u32 = 0x0000_0040; // Ethernet MAC PTP clock enable

pub const IRQ_ADDR_WWDG:u32 = 0x0000_0040; // Window Watchdog interrupt
pub const IRQ_ADDR_PVD:u32 = 0x0000_0044; // PVD through EXTI Line detection interrupt
pub const IRQ_ADDR_TAMPER:u32 = 0x0000_0048; // Tamper interrupt
pub const IRQ_ADDR_RTC:u32 = 0x0000_004C; // RTC global interrupt
pub const IRQ_ADDR_FLASH:u32 = 0x0000_0050; // Flash global interrupt
pub const IRQ_ADDR_RCC:u32 = 0x0000_0054; // RCC global interrupt
pub const IRQ_ADDR_EXTI0:u32 = 0x0000_0058; // EXTI0 interrupt
pub const IRQ_ADDR_EXTI1:u32 = 0x0000_005C; // EXTI1 interrupt
pub const IRQ_ADDR_EXTI2:u32 = 0x0000_0060; // EXTI2 interrupt

pub const EXTI_BASE: u32 = 0x4001_0400;         // EXTI base address
pub const EXTI_IMR: u32 = EXTI_BASE + 0x00;     // Interrupt Mask Register
pub const EXTI_EMR: u32 = EXTI_BASE + 0x04;     // Event Mask Register
pub const EXTI_RTSR: u32 = EXTI_BASE + 0x08;    // Rising Trigger Selection Register
pub const EXTI_FTSR: u32 = EXTI_BASE + 0x0C;    // Falling Trigger Selection Register
pub const EXTI_SWIER: u32 = EXTI_BASE + 0x10;   // Software Interrupt Event Register
pub const EXTI_PR: u32 = EXTI_BASE + 0x14;      // Pending Register

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

impl IRQn
{
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
}