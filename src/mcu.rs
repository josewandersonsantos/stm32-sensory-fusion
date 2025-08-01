// This file defines the memory-mapped addresses for GPIO ports and specific pins
// for STM32F103C8T6.

#![allow(dead_code)]
pub const CLOCK_FREQUENCY: u32 = 8_000_000; // 8 MHz clock frequency
/*
 * Memory-mapped addresses for GPIO ports
 */
pub const AFIO_BASE: u32 = 0x4001_0000; // AFIO base address
pub const AFIO_EXTICR1: u32 = AFIO_BASE + 0x08; // AFIO_EXTICR1 offset
pub const AFIO_EXTICR2: u32 = AFIO_BASE + 0x0C; // AFIO_EXTICR2 offset
pub const AFIO_EXTICR3: u32 = AFIO_BASE + 0x10; // AFIO_EXTICR3 offset
pub const AFIO_EXTICR4: u32 = AFIO_BASE + 0x14; // AFIO_EXTICR4 offset

/*
 * GPIO pin definitions
 */
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

/*
 * Memory-mapped addresses for USART
 */
pub const USART1_BASE: u32 = 0x4001_3800;
pub const USART2_BASE: u32 = 0x4000_4400;
pub const USART3_BASE: u32 = 0x4000_4800;

pub const USART_SR: u32 = 0x00;   // Status register
pub const USART_DR: u32 = 0x04;   // Data register (read = RX, write = TX)
pub const USART_BRR: u32 = 0x08;  // Baud rate register
pub const USART_CR1: u32 = 0x0C;  // Control register 1
pub const USART_CR2: u32 = 0x10;  // Control register 2
pub const USART_CR3: u32 = 0x14;  // Control register 3
pub const USART_GTPR: u32 = 0x18; // Guard time and prescaler register

pub const USART_SR_RXNE: u32 = 1 << 5; // Read data register not empty
pub const USART_SR_TXE: u32  = 1 << 7; // Transmit data register empty

/*
 * Memory-mapped addresses for I2C
 */
pub const I2C1_BASE: u32 = 0x4000_5400; // I2C1 base address
pub const I2C2_BASE: u32 = 0x4000_5800; // I2C2 base address
pub const I2C3_BASE: u32 = 0x4000_5C00; // I2C3 base address

pub const I2C_CR1: u32 = 0x00; // Control register 1
pub const I2C_CR2: u32 = 0x04; // Control register 2
pub const I2C_OAR1: u32 = 0x08; // Own address register 1
pub const I2C_OAR2: u32 = 0x0C; // Own address register 2
pub const I2C_DR: u32 = 0x10; // Data register
pub const I2C_SR1: u32 = 0x14; // Status register 1
pub const I2C_SR2: u32 = 0x18; // Status register 2
pub const I2C_CCR: u32 = 0x1C; // Clock control register
pub const I2C_TRISE: u32 = 0x20; // TRISE register
pub const I2C_SR1_SB: u32 = 1 << 0; // Start bit
pub const I2C_SR1_ADDR: u32 = 1 << 1; // Address sent (master mode) or matched (slave mode)
pub const I2C_SR1_BTF: u32 = 1 << 2; // Byte transfer finished
pub const I2C_SR1_RXNE: u32 = 1 << 6; // Data register not empty (receiving)
pub const I2C_SR1_TXE: u32 = 1 << 7; // Data register empty (transmitting)
pub const I2C_SR1_STOPF: u32 = 1 << 4; // Stop detection (slave mode)
pub const I2C_SR1_AF: u32 = 1 << 10; // Acknowledge failure
pub const I2C_SR1_ARLO: u32 = 1 << 9; // Arbitration lost (master mode)
pub const I2C_SR1_BUSY: u32 = 1 << 1; // Bus busy
pub const I2C_SR1_OVR: u32 = 1 << 11; // Overrun/underrun (slave mode)

/*
 * Memory-mapped addresses for SPI
 */

/*
 * Memory-mapped addresses for TIM
 */
pub const TIM2_BASE: u32 = 0x4000_0000; // TIM2 base address
pub const TIM3_BASE: u32 = 0x4000_0400; // TIM3 base address
pub const TIM4_BASE: u32 = 0x4000_0800; // TIM4 base address
pub const TIM5_BASE: u32 = 0x4000_0C00; // TIM5 base address
pub const TIM6_BASE: u32 = 0x4000_1000; // TIM6 base address
pub const TIM7_BASE: u32 = 0x4000_1400; // TIM7 base address
pub const TIM8_BASE: u32 = 0x4001_0000; // TIM8 base address
pub const TIM_CR1: u32 = 0x00; // Control register 1
pub const TIM_CR2: u32 = 0x04; // Control register 2
pub const TIM_SMCR: u32 = 0x08; // Slave mode control register
pub const TIM_DIER: u32 = 0x0C; // DMA/interrupt enable register
pub const TIM_SR: u32 = 0x10; // Status register
pub const TIM_EGR: u32 = 0x14; // Event generation register
pub const TIM_CCMR1: u32 = 0x18; // Capture/compare mode register 1
pub const TIM_CCMR2: u32 = 0x1C; // Capture/compare mode register 2
pub const TIM_CCER: u32 = 0x20; // Capture/compare enable register
pub const TIM_CNT: u32 = 0x24; // Counter register
pub const TIM_PSC: u32 = 0x28; // Prescaler register
pub const TIM_ARR: u32 = 0x2C; // Auto-reload register
pub const TIM_CCR1: u32 = 0x34; // Capture/compare register 1
pub const TIM_CCR2: u32 = 0x38; // Capture/compare register 2
pub const TIM_CCR3: u32 = 0x3C; // Capture/compare register 3
pub const TIM_CCR4: u32 = 0x40; // Capture/compare register 4
pub const TIM_BDTR: u32 = 0x44; // Break and dead-time register
pub const TIM_DCR: u32 = 0x48; // DMA control register

/*
 * Memory-mapped addresses for DMA
 */
pub const DMA1_BASE: u32 = 0x4002_0000; // DMA1 base address
pub const DMA2_BASE: u32 = 0x4002_0400; // DMA2 base address
/*
 * Memory-mapped addresses for ADC
 */
pub const ADC1_BASE: u32 = 0x4001_2400; // ADC1 base address
pub const ADC2_BASE: u32 = 0x4001_2800; // ADC2 base address
pub const ADC3_BASE: u32 = 0x4001_2C00; // ADC3 base address

/*
 * Memory-mapped addresses for GPIO ports
 */
pub const GPIOA_BASE: u32 = 0x4001_0800; // GPIOA base address
pub const GPIOB_BASE: u32 = 0x4001_0C00; // GPIOB base address
pub const GPIOC_BASE: u32 = 0x4001_1000; // GPIOC base address
pub const GPIOD_BASE: u32 = 0x4001_1400; // GPIOD base address
pub const GPIOE_BASE: u32 = 0x4001_1800; // GPIOE base address
pub const GPIOF_BASE: u32 = 0x4001_1C00; // GPIOF base address
pub const GPIOG_BASE: u32 = 0x4001_2000; // GPIOG base address

/*
 * Memory-mapped addresses for RCC
 */
pub const RCC_ADDR: u32 = 0x4002_1000; // RCC base address

pub const RCC_APB1ENR: u32 = RCC_ADDR + 0x1C; // RCC_APB1ENR offset
pub const RCC_APB1ENR_TIM2: u32 = 0x0000_0001; // TIM2 clock enable
pub const RCC_APB1ENR_TIM3: u32 = 0x0000_0002; // TIM3 clock enable
pub const RCC_APB1ENR_USART2: u32 = 0x0000_0004; // USART2 clock enable
pub const RCC_APB1ENR_RESERVED: u32 = 0x0000_0008; // Reserved
pub const RCC_APB1ENR_WWDG: u32 = 0x0000_0010; // Window Watchdog clock enable
pub const RCC_APB1ENR_I2C1: u32 = 0x0000_0020; // I2C1 clock enable
pub const RCC_APB1ENR_I2C2: u32 = 0x0000_0040; // I2C2 clock enable
pub const RCC_APB1ENR_CAN: u32 = 0x0000_0080; // CAN clock enable
pub const RCC_APB1ENR_BKP: u32 = 0x0000_0100; // Backup interface clock enable
pub const RCC_APB1ENR_PWR: u32 = 0x0000_0200; // Power interface clock enable

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

/*
 * Memory-mapped addresses for NVIC
 */
pub const IRQ_ADDR_WWDG:u32 = 0x0000_0040; // Window Watchdog interrupt
pub const IRQ_ADDR_PVD:u32 = 0x0000_0044; // PVD through EXTI Line detection interrupt
pub const IRQ_ADDR_TAMPER:u32 = 0x0000_0048; // Tamper interrupt
pub const IRQ_ADDR_RTC:u32 = 0x0000_004C; // RTC global interrupt
pub const IRQ_ADDR_FLASH:u32 = 0x0000_0050; // Flash global interrupt
pub const IRQ_ADDR_RCC:u32 = 0x0000_0054; // RCC global interrupt
pub const IRQ_ADDR_EXTI0:u32 = 0x0000_0058; // EXTI0 interrupt
pub const IRQ_ADDR_EXTI1:u32 = 0x0000_005C; // EXTI1 interrupt
pub const IRQ_ADDR_EXTI2:u32 = 0x0000_0060; // EXTI2 interrupt

/*
 * Memory-mapped addresses for EXTI
 */
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