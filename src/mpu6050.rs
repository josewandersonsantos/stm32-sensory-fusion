
#![allow(dead_code)]
pub const MPU6050_ADDRESS: u8       = 0x68; // Default I2C address for MPU6050
pub const MPU6050_WHO_AM_I: u8      = 0x75; // WHO_AM_I register address
pub const MPU6050_PWR_MGMT_1: u8    = 0x6B; // Power management register
pub const MPU6050_ACCEL_XOUT_H: u8  = 0x3B; // Accelerometer X-axis high byte
pub const MPU6050_ACCEL_XOUT_L: u8  = 0x3C; // Accelerometer X-axis low byte
pub const MPU6050_ACCEL_YOUT_H: u8  = 0x3D; // Accelerometer Y-axis high byte
pub const MPU6050_ACCEL_YOUT_L: u8  = 0x3E; // Accelerometer Y-axis low byte
pub const MPU6050_ACCEL_ZOUT_H: u8  = 0x3F; // Accelerometer Z-axis high byte
pub const MPU6050_ACCEL_ZOUT_L: u8  = 0x40; // Accelerometer Z-axis low byte
pub const MPU6050_GYRO_XOUT_H: u8   = 0x43; // Gyroscope X-axis high byte
pub const MPU6050_GYRO_XOUT_L: u8   = 0x44; // Gyroscope X-axis low byte
pub const MPU6050_GYRO_YOUT_H: u8   = 0x45; // Gyroscope Y-axis high byte
pub const MPU6050_GYRO_YOUT_L: u8   = 0x46; // Gyroscope Y-axis low byte
pub const MPU6050_GYRO_ZOUT_H: u8   = 0x47; // Gyroscope Z-axis high byte
pub const MPU6050_GYRO_ZOUT_L: u8   = 0x48; // Gyroscope Z-axis low byte
pub const MPU6050_TEMP_OUT_H: u8    = 0x41; // Temperature high byte
pub const MPU6050_TEMP_OUT_L: u8    = 0x42; // Temperature low byte
pub const MPU6050_ACCEL_CONFIG: u8  = 0x1C; // Accelerometer configuration register
pub const MPU6050_GYRO_CONFIG: u8   = 0x1B; // Gyroscope configuration register
pub const MPU6050_SMPLRT_DIV: u8    = 0x19; // Sample rate divider register
pub const MPU6050_CONFIG: u8        = 0x1A; // Configuration register
pub const MPU6050_INT_ENABLE: u8    = 0x38; // Interrupt enable register
pub const MPU6050_USER_CTRL: u8     = 0x6A; // User control register
pub const MPU6050_FIFO_EN: u8       = 0x23; // FIFO enable register
pub const MPU6050_I2C_MST_CTRL: u8  = 0x24; // I2C Master control register
pub const MPU6050_I2C_SLV0_ADDR: u8 = 0x25; // I2C Slave 0 address register
pub const MPU6050_I2C_SLV0_REG: u8  = 0x26; // I2C Slave 0 register address
pub const MPU6050_I2C_SLV0_CTRL: u8 = 0x27; // I2C Slave 0 control register
pub const MPU6050_I2C_SLV1_ADDR: u8 = 0x28; // I2C Slave 1 address register
pub const MPU6050_I2C_SLV1_REG: u8  = 0x29; // I2C Slave 1 register address
pub const MPU6050_I2C_SLV1_CTRL: u8 = 0x2A; // I2C Slave 1 control register
pub const MPU6050_I2C_SLV2_ADDR: u8 = 0x2B; // I2C Slave 2 address register
pub const MPU6050_I2C_SLV2_REG: u8  = 0x2C; // I2C Slave 2 register address
pub const MPU6050_I2C_SLV2_CTRL: u8 = 0x2D; // I2C Slave 2 control register
pub const MPU6050_I2C_SLV3_ADDR: u8 = 0x2E; // I2C Slave 3 address register
pub const MPU6050_I2C_SLV3_REG: u8  = 0x2F; // I2C Slave 3 register address
pub const MPU6050_I2C_SLV3_CTRL: u8 = 0x30; // I2C Slave 3 control register

#[derive(Copy, Clone)]
pub enum AccelRange
{
    G2  = 0b00,
    G4  = 0b01,
    G8  = 0b10,
    G16 = 0b11,
}

#[derive(Copy, Clone)]
pub enum GyroRange
{
    D250  = 0b00,
    D500  = 0b01,
    D1000 = 0b10,
    D2000 = 0b11,
}

#[derive(Copy, Clone)]
pub enum Dlpf
{
    Hz260 = 0,
    Hz184 = 1,
    Hz94  = 2,
    Hz44  = 3,
    Hz21  = 4,
    Hz10  = 5,
    Hz5   = 6,
}

use crate::{i2c, utils};

pub struct Mpu6050
{
    i2c: i2c::I2C,
    addr: u8,
}

pub fn init(i2c: &i2c::I2C, accel: AccelRange, gyro: GyroRange, dlpf: Dlpf)
{
    /* Wake up */
    i2c::master::write_register8(i2c, MPU6050_ADDRESS, MPU6050_PWR_MGMT_1, 0x00);
    utils::delay_ms(100);

    /* Sample rate = Gyro / (1 + div) */
    i2c::master::write_register8(i2c, MPU6050_ADDRESS, MPU6050_SMPLRT_DIV, 0x07); // ~125 Hz com DLPF ativo

    /* DLPF */
    i2c::master::write_register8(i2c, MPU6050_ADDRESS, MPU6050_CONFIG, dlpf as u8);

    /* Gyro range */
    i2c::master::write_register8(i2c, MPU6050_ADDRESS, MPU6050_GYRO_CONFIG, (gyro as u8) << 3);

    /* Accel range */
    i2c::master::write_register8(i2c, MPU6050_ADDRESS, MPU6050_ACCEL_CONFIG, (accel as u8) << 3);

    /* Disable FIFO */
    i2c::master::write_register8(i2c, MPU6050_ADDRESS, MPU6050_FIFO_EN, 0x00);

    /* Disable interrupts */
    i2c::master::write_register8(i2c, MPU6050_ADDRESS, MPU6050_INT_ENABLE, 0x00);

    /* Disable I2C master */
    i2c::master::write_register8(i2c, MPU6050_ADDRESS, MPU6050_USER_CTRL, 0x00);
}

pub fn check(i2c: &i2c::I2C) -> bool
{
    i2c::master::read_register8(i2c, MPU6050_ADDRESS, MPU6050_WHO_AM_I) == 0x68
}

fn read_i16(i2c: &i2c::I2C, reg_h: u8, reg_l: u8) -> i16
{
    let h = i2c::master::read_register8(i2c, MPU6050_ADDRESS, reg_h);
    let l = i2c::master::read_register8(i2c, MPU6050_ADDRESS, reg_l);
    ((h as i16) << 8) | (l as i16)
}

pub fn accel_raw(i2c: &i2c::I2C) -> (i16, i16, i16)
{
    (
        read_i16(i2c, MPU6050_ACCEL_XOUT_H, MPU6050_ACCEL_XOUT_L),
        read_i16(i2c, MPU6050_ACCEL_YOUT_H, MPU6050_ACCEL_YOUT_L),
        read_i16(i2c, MPU6050_ACCEL_ZOUT_H, MPU6050_ACCEL_ZOUT_L),
    )
}

pub fn gyro_raw(i2c: &i2c::I2C) -> (i16, i16, i16)
{
    (
        read_i16(i2c, MPU6050_GYRO_XOUT_H, MPU6050_GYRO_XOUT_L),
        read_i16(i2c, MPU6050_GYRO_YOUT_H, MPU6050_GYRO_YOUT_L),
        read_i16(i2c, MPU6050_GYRO_ZOUT_H, MPU6050_GYRO_ZOUT_L),
    )
}

pub fn temperature_raw(i2c: &i2c::I2C) -> i16
{
    read_i16(i2c, MPU6050_TEMP_OUT_H, MPU6050_TEMP_OUT_L)
}

pub fn temperature_c(i2c: &i2c::I2C) -> f32
{
    let raw = temperature_raw(i2c);
    (raw as f32) / 340.0 + 36.53
}

pub fn accel_g(i2c: &i2c::I2C, range: AccelRange) -> (f32, f32, f32)
{
    let (x, y, z) = accel_raw(i2c);
    let scale = match range
    {
        AccelRange::G2  => 16384.0,
        AccelRange::G4  => 8192.0,
        AccelRange::G8  => 4096.0,
        AccelRange::G16 => 2048.0,
    };
    (
        x as f32 / scale,
        y as f32 / scale,
        z as f32 / scale,
    )
}

pub fn gyro_dps(i2c: &i2c::I2C, range: GyroRange) -> (f32, f32, f32)
{
    let (x, y, z) = gyro_raw(i2c);
    let scale = match range
    {
        GyroRange::D250  => 131.0,
        GyroRange::D500  => 65.5,
        GyroRange::D1000 => 32.8,
        GyroRange::D2000 => 16.4,
    };
    (
        x as f32 / scale,
        y as f32 / scale,
        z as f32 / scale,
    )
}

pub fn who_am_i(i2c: &i2c::I2C) -> u8
{
    i2c::master::read_register8(i2c, MPU6050_ADDRESS, MPU6050_WHO_AM_I)
}

pub fn reset(i2c: &i2c::I2C)
{
    i2c::master::write_register8(i2c, MPU6050_ADDRESS, MPU6050_PWR_MGMT_1, 1 << 7);
    utils::delay_ms(100);
}

pub fn sleep(i2c: &i2c::I2C, enable: bool)
{
    let mut v = i2c::master::read_register8(i2c, MPU6050_ADDRESS, MPU6050_PWR_MGMT_1);
    if enable
    {
        v |= 1 << 6;
    }
    else
    {
        v &= !(1 << 6);
    }
    i2c::master::write_register8(i2c, MPU6050_ADDRESS, MPU6050_PWR_MGMT_1, v);
}
// pub fn 