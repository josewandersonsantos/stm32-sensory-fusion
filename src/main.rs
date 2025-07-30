// https://github.com/niekiran/embedded-rust
// cargo flash --chip STM32F103C8T6
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod startup_stm32f103;
mod utils;
mod mcu;
mod rcc;
mod gpio;
mod usart;
mod irq;
mod led;
mod gps;

#[no_mangle]
fn main() -> !
{
    // Initialize LED on PC13
    gpio::enable_gpio_clock(mcu::GPIOC_BASE);
    gpio::configure_pin(mcu::GPIOC_BASE, mcu::GPIO13, gpio::GpioMode::Output, gpio::GpioConfig::PushPull, Some(gpio::GpioSpeed::Speed2MHz));

    // USART1 CFG
    rcc::apb2::enable(rcc::apb2::Apb2Peripheral::IoPa);
    // USART1 TX
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO09, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfPushPull, Some(gpio::GpioSpeed::Speed50MHz));
    // USART1 RX
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO10, gpio::GpioMode::Input, gpio::GpioConfig::Floating, None);
    usart::start( usart::Usart::Usart1, usart::UsartMode::TxRx, usart::UsartInterrupt::RxInterrupt, usart::UsartBaudRate::B9600, usart::UsartWordLength::Length8Bits, usart::UsartStopBits::Stop1Bit, usart::UsartParity::None);
    irq::enable_irq(37); // USART1_IRQn = 37

    loop
    {
        // Toggle LED on PC13
        led::led_toggle(mcu::GPIOC_BASE, mcu::GPIO13);
        // Process GPS data        
        gps::process_gps();        
        // Delay (simple busy-wait loop)
        for _ in 0..50_000 {}
    }    
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop {}
}

#[no_mangle]
pub extern "C" fn USART1_Handler()
{
    // Handle USART1 interrupt
    unsafe
    {
        let usart1_sr = (mcu::USART1_BASE + mcu::USART_SR) as *const u32;
        let usart1_dr = (mcu::USART1_BASE + mcu::USART_DR) as *mut u32;

        let sr = utils::read_register(usart1_sr);
        if (sr & mcu::USART_SR_RXNE) != 0
        {
            let data = utils::read_register(usart1_dr) as u8;

            while (utils::read_register(usart1_sr) & mcu::USART_SR_TXE) == 0 {}
            utils::write_register(usart1_dr, data as u32);

            gps::push_byte(data);
        }
    }
}