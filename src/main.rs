// https://github.com/niekiran/embedded-rust
// cargo flash --chip STM32F103C8T6
#![no_std]
#![no_main]

use core::{panic::PanicInfo, sync::atomic::AtomicBool, sync::atomic::Ordering};

mod startup_stm32f103;
mod utils;
mod checksum;
mod mcu;
mod rcc;
mod gpio;
mod usart;
// mod usb;
// mod crc;
mod watchdog;
mod irq;
mod led;
mod i2c;
mod bridge;
mod gps_neo6m;
// mod mpu6050;
// mod mpu9250;
mod icm20948;
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
            gps_neo6m::push_byte(data);

            //while (utils::read_register(usart1_sr) & mcu::USART_SR_TXE) == 0 {}
            // utils::write_register(usart1_dr, data as u32);
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
    // Read ICM20948 data
    let (acc_x, acc_y, acc_z) = icm20948::accel_g(&i2c::I2C::I2C1, icm20948::AccelRange::G2);
    let (gyr_x, gyr_y, gyr_z) = icm20948::gyro_dps(&i2c::I2C::I2C1, icm20948::GyroRange::D500);
    let temp_c                          = icm20948::temperature_c(&i2c::I2C::I2C1);
    let (mag_x, mag_y, mag_z) = icm20948::mag_raw(&i2c::I2C::I2C1);

    // if acc_x == 0.0 || acc_y == 0.0 || acc_z == 0.0 || gyr_x == 0.0 || gyr_y == 0.0 || gyr_z == 0.0
    // {
    //     return;
    // }

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

fn cb_coords_from_gps(lat: f32, lng: f32, height : f32)
{
    let mut payload = [0u8; 12];
    let frame: bridge::FrameTx = bridge::get_gps_coords(&mut payload, lat, lng, height);
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
    //rcc::apb1::enable(rcc::apb1::Apb1Peripheral::Usb);
    
    // IWDG
    watchdog::iwdg::init(500);
    
    // PC13 (LED)
    gpio::configure_pin(mcu::GPIOC_BASE, mcu::GPIO13, gpio::GpioMode::Output, gpio::GpioConfig::PushPull, Some(gpio::GpioSpeed::Speed2MHz));
    
    // USB
    //usb::init();
    
    // USART2 (DEBUG)
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO02, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfPushPull, Some(gpio::GpioSpeed::Speed50MHz));
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO03, gpio::GpioMode::Input, gpio::GpioConfig::Floating, None);
    usart::start( usart::Usart::Usart2, usart::UsartMode::TxRx, usart::UsartInterrupt::RxInterrupt, usart::UsartBaudRate::B230400, usart::UsartWordLength::Length8Bits, usart::UsartStopBits::Stop1Bit, usart::UsartParity::None);
    
    // USART1 (GPS)
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO09, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfPushPull, Some(gpio::GpioSpeed::Speed50MHz));
    gpio::configure_pin(mcu::GPIOA_BASE, mcu::GPIO10, gpio::GpioMode::Input, gpio::GpioConfig::Floating, None);
    usart::start(usart::Usart::Usart1, usart::UsartMode::TxRx, usart::UsartInterrupt::RxInterrupt, usart::UsartBaudRate::B9600, usart::UsartWordLength::Length8Bits, usart::UsartStopBits::Stop1Bit, usart::UsartParity::None);
    gps_neo6m::init(usart::Usart::Usart1, gps_neo6m::GPSProtocol::NMEA, gps_neo6m::GPSBaudRate::B9600, gps_neo6m::GPSUpdateRate::R20Hz, gps_neo6m::GPSOperationMode::Normal, &[gps_neo6m::GPSNmeaSentence::GGA, gps_neo6m::GPSNmeaSentence::RMC], cb_line_from_gps, cb_coords_from_gps);
    irq::enable_irq(mcu::IRQn::USART1 as u32);

    // I2C1 (MPU6050)
    gpio::configure_pin(mcu::GPIOB_BASE, mcu::GPIO06, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfOpenDrain, Some(gpio::GpioSpeed::Speed50MHz));
    gpio::configure_pin(mcu::GPIOB_BASE, mcu::GPIO07, gpio::GpioMode::AlternateFunction, gpio::GpioConfig::AfOpenDrain, Some(gpio::GpioSpeed::Speed50MHz));
    i2c::start(i2c::I2C::I2C1, i2c::I2CClockSpeed::Standard100kHz);
    icm20948::init(&i2c::I2C::I2C1, icm20948::AccelRange::G2, icm20948::GyroRange::D250);

    loop
    {
        // Toggle LED on PC13
        led::led_toggle(mcu::GPIOC_BASE, mcu::GPIO13);
        // Process GPS data
        gps_neo6m::process_gps();
        // Send ICM20948 data
        send_mpu_data();
        // Refresh IWDG
        watchdog::iwdg::refresh();
        // Delay
        utils::delay_ms(50);
    }
}