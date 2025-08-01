use core::ptr;
/*
 * Define the vector table to MCU F103
 */

#[no_mangle]
extern "C" fn HardFault_Handler() { loop {} }
#[no_mangle]
extern "C" fn NMI_Handler() { loop {} }
#[no_mangle]
extern "C" fn Default_Handler() { loop {} }

extern "C"
{
    fn BusFault_Handler();
    fn MemManage_Handler();
    fn PendSV_Handler();
    fn SVCall_Handler();
    fn SysTick_Handler();
    fn UsageFault_Handler();
    fn ADC1_2_Handler();
    fn ADC3_Handler();
    fn CAN_RX1_Handler();
    fn CAN_SCE_Handler();
    fn DMA1_Channel1_Handler();
    fn DMA1_Channel2_Handler();
    fn DMA1_Channel3_Handler();
    fn DMA1_Channel4_Handler();
    fn DMA1_Channel5_Handler();
    fn DMA1_Channel6_Handler();
    fn DMA1_Channel7_Handler();
    fn DMA2_Channel1_Handler();
    fn DMA2_Channel2_Handler();
    fn DMA2_Channel3_Handler();
    fn DMA2_Channel4_5_Handler();
    fn EXTI0_Handler();
    fn EXTI15_10_Handler();
    fn EXTI1_Handler();
    fn EXTI2_Handler();
    fn EXTI3_Handler();
    fn EXTI4_Handler();
    fn EXTI9_5_Handler();
    fn FLASH_Handler();
    fn FSMC_Handler();
    fn I2C1_ER_Handler();
    fn I2C1_EV_Handler();
    fn I2C2_ER_Handler();
    fn I2C2_EV_Handler();
    fn PVD_Handler();
    fn RCC_Handler();
    fn RTCAlarm_Handler();
    fn RTC_Handler();
    fn SDIO_Handler();
    fn SPI1_Handler();
    fn SPI2_Handler();
    fn SPI3_Handler();
    fn TAMPER_Handler();
    fn TIM1_BRK_Handler();
    fn TIM1_CC_Handler();
    fn TIM1_TRG_COM_Handler();
    fn TIM1_UP_Handler();
    fn TIM2_Handler();
    fn TIM3_Handler();
    fn TIM4_Handler();
    fn TIM5_Handler();
    fn TIM6_Handler();
    fn TIM7_Handler();
    fn TIM8_BRK_Handler();
    fn TIM8_CC_Handler();
    fn TIM8_TRG_COM_Handler();
    fn TIM8_UP_Handler();
    fn UART4_Handler();
    fn UART5_Handler();
    fn USART1_Handler();
    fn USART2_Handler();
    fn USART3_Handler();
    fn USB_HP_CAN_TX_Handler();
    fn USB_LP_CAN_RX0_Handler();
    fn WWDG_Handler();
}
/*
 * Define the memory layout of the MCU
 */
#[used]
#[link_section = ".isr_vector"]
static VECTOR_TABLE: [Option<unsafe extern "C" fn()>; 75] =
[
    Some(Reset_Handler),
    Some(NMI_Handler),
    Some(HardFault_Handler),
    Some(MemManage_Handler),
    Some(BusFault_Handler),
    Some(UsageFault_Handler),
    None,
    None,
    None,
    None,
    Some(SVCall_Handler),
    None,
    None,
    Some(PendSV_Handler),
    Some(SysTick_Handler),
    Some(WWDG_Handler),
    Some(PVD_Handler),
    Some(TAMPER_Handler),
    Some(RTC_Handler),
    Some(FLASH_Handler),
    Some(RCC_Handler),
    Some(EXTI0_Handler),
    Some(EXTI1_Handler),
    Some(EXTI2_Handler),
    Some(EXTI3_Handler),
    Some(EXTI4_Handler),
    Some(DMA1_Channel1_Handler),
    Some(DMA1_Channel2_Handler),
    Some(DMA1_Channel3_Handler),
    Some(DMA1_Channel4_Handler),
    Some(DMA1_Channel5_Handler),
    Some(DMA1_Channel6_Handler),
    Some(DMA1_Channel7_Handler),
    Some(ADC1_2_Handler),
    Some(USB_HP_CAN_TX_Handler),
    Some(USB_LP_CAN_RX0_Handler),
    Some(CAN_RX1_Handler),
    Some(CAN_SCE_Handler),
    Some(EXTI9_5_Handler),
    Some(TIM1_BRK_Handler),
    Some(TIM1_UP_Handler),
    Some(TIM1_TRG_COM_Handler),
    Some(TIM1_CC_Handler),
    Some(TIM2_Handler),
    Some(TIM3_Handler),
    Some(TIM4_Handler),
    Some(I2C1_EV_Handler),
    Some(I2C1_ER_Handler),
    Some(I2C2_EV_Handler),
    Some(I2C2_ER_Handler),
    Some(SPI1_Handler),
    Some(SPI2_Handler),
    Some(USART1_Handler),
    Some(USART2_Handler),
    Some(USART3_Handler),
    Some(EXTI15_10_Handler),
    Some(RTCAlarm_Handler),
    None,
    Some(TIM8_BRK_Handler),
    Some(TIM8_UP_Handler),
    Some(TIM8_TRG_COM_Handler),
    Some(TIM8_CC_Handler),
    Some(ADC3_Handler),
    Some(FSMC_Handler),
    Some(SDIO_Handler),
    Some(TIM5_Handler),
    Some(SPI3_Handler),
    Some(UART4_Handler),
    Some(UART5_Handler),
    Some(TIM6_Handler),
    Some(TIM7_Handler),
    Some(DMA2_Channel1_Handler),
    Some(DMA2_Channel2_Handler),
    Some(DMA2_Channel3_Handler),
    Some(DMA2_Channel4_5_Handler),
];

extern
{
    static _sidata:u32; /* Start address of .data section in FLASH */
    static mut _sdata:u32;  /* Start address of .data section in RAM */
    static mut _edata:u32;  /* End address of .data section in RAM */
    static mut _sbss:u32;   /* Start address of .bss section in RAM */
    static mut _ebss:u32;   /* End address of .bss section in RAM */
}

/*
 * Define the reset handler
*/
#[no_mangle]
#[allow(non_snake_case)]
extern "C" fn Reset_Handler()
{
    // --- Copy the .data section from flash to RAM, if needed.
    // ref of staticc variable to C like raw pointer
    unsafe
    {
        let mut src_in_flash = ptr::addr_of!(_sidata);
        let mut dst_in_ram: *mut u32 = ptr::addr_of_mut!(_sdata);
        let data_end_is_ram = ptr::addr_of_mut!(_edata);

        while dst_in_ram < data_end_is_ram
        {
                // Copy the data from flash to RAM
                *dst_in_ram = *src_in_flash;
                dst_in_ram = dst_in_ram.add(1);
                src_in_flash = src_in_flash.add(1);
        }
        }

    // Initialize the .bss section to zero in RAM, if needed.
    unsafe
    {
        let mut bss = ptr::addr_of_mut!(_sbss);
        let bss_end = ptr::addr_of_mut!(_ebss);
        while bss < bss_end
        {
                // Set the bss section to zero
                *bss = 0;
                bss = bss.add(1);
        }
    }
    
    // Call main();
    crate::main();
}

/*
 * Define the exception handler
*/