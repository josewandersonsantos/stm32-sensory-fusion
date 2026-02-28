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
mod i2c;
mod gps_neo6m;
mod mpu6050;

#[no_mangle]
fn main() -> !
{
    rcc::apb2::enable(rcc::apb2::Apb2Peripheral::Afio);
    rcc::apb2::enable(rcc::apb2::Apb2Peripheral::IoPc);
    rcc::apb2::enable(rcc::apb2::Apb2Peripheral::IoPb);
    rcc::apb2::enable(rcc::apb2::Apb2Peripheral::IoPa);
    
    // PC13 (LED)
    gpio::configure_pin(mcu::GPIOC_BASE, mcu::GPIO13, gpio::GpioMode::Output, gpio::GpioConfig::PushPull, Some(gpio::GpioSpeed::Speed2MHz));
    
    // USART1 (GPS)
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO09, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfPushPull, Some(gpio::GpioSpeed::Speed50MHz));
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO10, gpio::GpioMode::Input, gpio::GpioConfig::Floating, None);
    usart::start(usart::Usart::Usart1, usart::UsartMode::TxRx, usart::UsartInterrupt::RxInterrupt, usart::UsartBaudRate::B9600, usart::UsartWordLength::Length8Bits, usart::UsartStopBits::Stop1Bit, usart::UsartParity::None);
    // irq::enable_irq(mcu::IRQn::USART1 as u32);

    // USART2 (DEBUG)
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO02, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfPushPull, Some(gpio::GpioSpeed::Speed50MHz));
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO03, gpio::GpioMode::Input, gpio::GpioConfig::Floating, None);
    usart::start( usart::Usart::Usart2, usart::UsartMode::TxRx, usart::UsartInterrupt::RxInterrupt, usart::UsartBaudRate::B9600, usart::UsartWordLength::Length8Bits, usart::UsartStopBits::Stop1Bit, usart::UsartParity::None);
    
    // I2C1 (MPU6050)
    gpio::configure_pin(mcu::GPIOB_BASE, mcu::GPIO06, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfOpenDrain, Some(gpio::GpioSpeed::Speed50MHz));
    gpio::configure_pin(mcu::GPIOB_BASE, mcu::GPIO07, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfOpenDrain, Some(gpio::GpioSpeed::Speed50MHz));
    i2c::start(i2c::I2C::I2C1, i2c::I2CMode::Standard, i2c::I2CAddressingMode::SevenBit, i2c::I2CClockSpeed::Standard100kHz, i2c::I2CDataFormat::Data8Bit);
    //let bt = i2c::master::read_register8(i2c::I2C::I2C1, 0x68, 0x75); // WHO_AM_I register of MPU6050
    if mpu6050::check(&i2c::I2C::I2C1)
    {
        mpu6050::init(&i2c::I2C::I2C1, mpu6050::AccelRange::G2, mpu6050::GyroRange::D500, mpu6050::Dlpf::Hz94);
    }

    loop
    {
        // Toggle LED on PC13
        led::led_toggle(mcu::GPIOC_BASE, mcu::GPIO13);
        // Process GPS data
        gps_neo6m::process_gps();
        // Read MPU6050 data
        let (x, y, z)    = mpu6050::accel_g(&i2c::I2C::I2C1, mpu6050::AccelRange::G2);
        let (gx, gy, gz) = mpu6050::gyro_dps(&i2c::I2C::I2C1, mpu6050::GyroRange::D500);
        let temp_c                 = mpu6050::temperature_c(&i2c::I2C::I2C1);
        
        utils::delay_ms(500);
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

            // gps_neo6m::push_byte(data);
            usart::write(usart::Usart::Usart2, data);
        }
    }
}