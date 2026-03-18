// https://github.com/niekiran/embedded-rust
// cargo flash --chip STM32F103C8T6
#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr, sync::atomic::AtomicBool, sync::atomic::Ordering};

mod startup_stm32f103;
mod utils;
mod checksum;
mod mcu;
mod rcc;
mod gpio;
mod usart;
mod irq;
mod led;
mod i2c;
mod bridge;
mod gps_neo6m;
mod mpu6050;
mod mpu9250;
mod fusion;
mod kalman_filter;

/*
 * PANIC HANDLER
 */
#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop {}
}

/*
 * CALLBACKS IT
 */
#[no_mangle]
pub extern "C" fn USART1_Handler()
{
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

            gps_neo6m::push_byte(data);
            //usart::write(usart::Usart::Usart2, data);
        }
    }
}

/*
 * BRIDGE TX
 */
static MTX_SEND_FRAME: AtomicBool = AtomicBool::new(false);
fn send_frame(frame: &bridge::FrameTx)
{
    if MTX_SEND_FRAME.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err()
    {
        return;
    }
    
    usart::write(usart::Usart::Usart2, 0x7E);
    usart::write_bytes(usart::Usart::Usart2, utils::as_bytes(&frame.header));
    usart::write_bytes(usart::Usart::Usart2, frame.payload);
    usart::write_bytes(usart::Usart::Usart2, utils::as_bytes(&frame.crc));
    
    MTX_SEND_FRAME.store(false, Ordering::Release);
}

fn send_mpu_data()
{
    // Read MPU6050 data
    let (acc_x, acc_y, acc_z) = mpu6050::accel_g(&i2c::I2C::I2C1, mpu6050::AccelRange::G2);
    let (gyr_x, gyr_y, gyr_z) = mpu6050::gyro_dps(&i2c::I2C::I2C1, mpu6050::GyroRange::D500);
    let temp_c                          = mpu6050::temperature_c(&i2c::I2C::I2C1);

    let mut payload = [0u8; 28];
    let frame = bridge::get_package_mpu_data(&mut payload, acc_x, acc_y, acc_z, gyr_x, gyr_y, gyr_z, temp_c);
    send_frame(&frame);
}

fn send_acc_data(x: f32, y: f32, z: f32)
{
    let mut payload = [0u8; 12];
    let frame = bridge::get_package_acc_data(&mut payload, x, y, z);
    send_frame(&frame);
}

fn send_gyr_data(x: f32, y: f32, z: f32)
{
    let mut payload = [0u8; 12];
    let frame = bridge::get_package_gyr_data(&mut payload, x, y, z);
    send_frame(&frame);
}

/*
 * CALLBACKS DATA
 */
fn cb_line_from_gps(line: &str)
{
    let frame: bridge::FrameTx = bridge::get_gps_data(line.as_bytes());
    send_frame(&frame);
}

/*
 * MAIN
 */
#[no_mangle]
fn main() -> !
{
    rcc::apb2::enable(rcc::apb2::Apb2Peripheral::Afio);
    rcc::apb2::enable(rcc::apb2::Apb2Peripheral::IoPc);
    rcc::apb2::enable(rcc::apb2::Apb2Peripheral::IoPb);
    rcc::apb2::enable(rcc::apb2::Apb2Peripheral::IoPa);
    
    // PC13 (LED)
    gpio::configure_pin(mcu::GPIOC_BASE, mcu::GPIO13, gpio::GpioMode::Output, gpio::GpioConfig::PushPull, Some(gpio::GpioSpeed::Speed2MHz));
    
    // USART2 (DEBUG)
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO02, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfPushPull, Some(gpio::GpioSpeed::Speed50MHz));
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO03, gpio::GpioMode::Input, gpio::GpioConfig::Floating, None);
    usart::start( usart::Usart::Usart2, usart::UsartMode::TxRx, usart::UsartInterrupt::RxInterrupt, usart::UsartBaudRate::B9600, usart::UsartWordLength::Length8Bits, usart::UsartStopBits::Stop1Bit, usart::UsartParity::None);
    
    // USART1 (GPS)
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO09, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfPushPull, Some(gpio::GpioSpeed::Speed50MHz));
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO10, gpio::GpioMode::Input, gpio::GpioConfig::Floating, None);
    usart::start(usart::Usart::Usart1, usart::UsartMode::TxRx, usart::UsartInterrupt::RxInterrupt, usart::UsartBaudRate::B9600, usart::UsartWordLength::Length8Bits, usart::UsartStopBits::Stop1Bit, usart::UsartParity::None);
    gps_neo6m::init(usart::Usart::Usart1, gps_neo6m::GPS_Frequency::F10Hz, gps_neo6m::GPS_Protocol::NMEA, gps_neo6m::GPS_BaudRate::B9600, gps_neo6m::GPS_UpdateRate::R1Hz, gps_neo6m::GPS_OperationMode::Normal, &[gps_neo6m::GPS_NmeaSentence::GGA, gps_neo6m::GPS_NmeaSentence::RMC], cb_line_from_gps);
    irq::enable_irq(mcu::IRQn::USART1 as u32);

    // I2C1 (MPU6050)
    gpio::configure_pin(mcu::GPIOB_BASE, mcu::GPIO06, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfOpenDrain, Some(gpio::GpioSpeed::Speed50MHz));
    gpio::configure_pin(mcu::GPIOB_BASE, mcu::GPIO07, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfOpenDrain, Some(gpio::GpioSpeed::Speed50MHz));
    i2c::start(i2c::I2C::I2C1, i2c::I2CMode::Standard, i2c::I2CAddressingMode::SevenBit, i2c::I2CClockSpeed::Standard100kHz, i2c::I2CDataFormat::Data8Bit);
    mpu6050::init(&i2c::I2C::I2C1, mpu6050::AccelRange::G2, mpu6050::GyroRange::D500, mpu6050::Dlpf::Hz94);
    //mpu9250::init(&i2c::I2C::I2C1, mpu9250::AccelRange::G2, mpu9250::GyroRange::D500, mpu9250::Dlpf::Hz94);

    loop
    {
        // Toggle LED on PC13
        led::led_toggle(mcu::GPIOC_BASE, mcu::GPIO13);
        // Process GPS data
        gps_neo6m::process_gps();
        // Send MPU6050 data
        send_mpu_data();
        utils::delay_ms(100);
    }
}