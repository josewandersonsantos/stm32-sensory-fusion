#![allow(dead_code)]

use crate::utils;
use crate::mcu;
use crate::i2c;

pub const ICM20948_ADDRESS: u8 = 0x68; // Default I2C address for ICM20948 (AD0 PULLDOWN)
pub const WHO_AM_I_VALUE: u8   = 0xEA; // WHO_AM_I register value

/*
 * BANK SELECT REGISTER (common register of all banks)
 */
pub const REG_BANK_SEL: u8 = 0x7F;

/// Bits to select bank (write in REG_BANK_SEL)
pub const BIT_BANK_SEL_0: u8 = 0b00000000;
pub const BIT_BANK_SEL_1: u8 = 0b00010000;
pub const BIT_BANK_SEL_2: u8 = 0b00100000;
pub const BIT_BANK_SEL_3: u8 = 0b00110000;

/*
 * BANK 0 REGISTERS (main: configuration, data, interrupts, FIFO)
 */
pub const REG0_WHO_AM_I: u8       = 0x00;
pub const REG0_USER_CTRL: u8      = 0x03;
pub const REG0_LP_CONFIG: u8      = 0x05;
pub const REG0_PWR_MGMT_1: u8     = 0x06;
pub const REG0_PWR_MGMT_2: u8     = 0x07;
pub const REG0_INT_PIN_CFG: u8    = 0x0F;
pub const REG0_INT_ENABLE: u8     = 0x10;   // INT_EN
pub const REG0_INT_ENABLE_1: u8   = 0x11;
pub const REG0_INT_ENABLE_2: u8   = 0x12;
pub const REG0_INT_ENABLE_3: u8   = 0x13;
pub const REG0_I2C_MST_STATUS: u8 = 0x17;
pub const REG0_INT_STATUS: u8     = 0x19;
pub const REG0_INT_STATUS_1: u8   = 0x1A;
pub const REG0_INT_STATUS_2: u8   = 0x1B;
pub const REG0_INT_STATUS_3: u8   = 0x1C;

pub const REG0_ACCEL_XOUT_H: u8   = 0x2D;   // Accelerometer data (14 bytes: XH..ZL + Temp + Gyro)
pub const REG0_ACCEL_XOUT_L: u8   = 0x2E;
pub const REG0_ACCEL_YOUT_H: u8   = 0x2F;
pub const REG0_ACCEL_YOUT_L: u8   = 0x30;
pub const REG0_ACCEL_ZOUT_H: u8   = 0x31;
pub const REG0_ACCEL_ZOUT_L: u8   = 0x32;
pub const REG0_GYRO_XOUT_H: u8    = 0x33;
pub const REG0_GYRO_XOUT_L: u8    = 0x34;
pub const REG0_GYRO_YOUT_H: u8    = 0x35;
pub const REG0_GYRO_YOUT_L: u8    = 0x36;
pub const REG0_GYRO_ZOUT_H: u8    = 0x37;
pub const REG0_GYRO_ZOUT_L: u8    = 0x38;
pub const REG0_TEMP_OUT_H: u8     = 0x39;
pub const REG0_TEMP_OUT_L: u8     = 0x3A;

pub const REG0_FIFO_EN_1: u8      = 0x66;
pub const REG0_FIFO_EN_2: u8      = 0x67;
pub const REG0_FIFO_RST: u8       = 0x68;   // or USER_CTRL to reset FIFO
pub const REG0_FIFO_COUNTH: u8    = 0x70;
pub const REG0_FIFO_COUNTL: u8    = 0x71;
pub const REG0_FIFO_R_W: u8       = 0x72;
pub const REG0_DATA_RDY_STTS: u8  = 0x74;
pub const REG0_FIFO_CONFIG: u8    = 0x76;

pub const REG0_TIMEBASE_CORRECTION_PLL: u8 = 0x28;

// Bits USER_CTRL
pub const BIT_USER_CTRL_DMP_EN: u8       = 0b10000000;
pub const BIT_USER_CTRL_FIFO_EN: u8      = 0b01000000;
pub const BIT_USER_CTRL_I2C_MST_EN: u8   = 0b00100000;
pub const BIT_USER_CTRL_I2C_IF_DIS: u8   = 0b00010000; // disable I2C, only SPI
pub const BIT_USER_CTRL_DMP_RST: u8      = 0b00001000;
pub const BIT_USER_CTRL_SRAM_RST: u8     = 0b00000100;
pub const BIT_USER_CTRL_I2C_MST_RST: u8  = 0b00000010;
pub const BIT_USER_CTRL_SIG_COND_RST: u8 = 0b00000001; // reset of sensors

// Bits PWR_MGMT_1
pub const BIT_PWR1_DEVICE_RESET: u8 = 0b10000000;
pub const BIT_PWR1_SLEEP: u8        = 0b01000000;
pub const BIT_PWR1_LP_EN: u8        = 0b00100000;
pub const BIT_PWR1_TEMP_DIS: u8     = 0b00001000;
pub const BIT_PWR1_CLKSEL_0: u8     = 0b00000001; // bits [2:0] para clock source

// Bits INT_PIN_CFG
pub const BIT_INT_PIN_CFG_INT_LEVEL: u8      = 0b10000000; // 1 = active low
pub const BIT_INT_PIN_CFG_INT_OPEN: u8       = 0b01000000; // open-drain
pub const BIT_INT_PIN_CFG_LATCH_INT_EN: u8   = 0b00100000;
pub const BIT_INT_PIN_CFG_BYPASS_EN: u8      = 0b00000010; // I2C bypass (para magnetômetro AK09916)

// // Bits GYRO_CONFIG_1 (Bank 0, 0x1A)
// pub const BIT_GYRO_FS_SEL_2000DPS: u8 = 0b00011000; // [4:3] = 11 → ±2000 dps
// pub const BIT_GYRO_FS_SEL_1000DPS: u8 = 0b00010000;
// pub const BIT_GYRO_FS_SEL_500DPS: u8  = 0b00001000;
// pub const BIT_GYRO_FS_SEL_250DPS: u8  = 0b00000000;
// pub const BIT_GYRO_DLPFCFG: u8        = 0b00000111; // [2:0]

// // Bits ACCEL_CONFIG (Bank 0, 0x14)
// pub const BIT_ACCEL_FS_SEL_16G: u8 = 0b00011000; // [4:3]
// pub const BIT_ACCEL_FS_SEL_8G: u8  = 0b00010000;
// pub const BIT_ACCEL_FS_SEL_4G: u8  = 0b00001000;
// pub const BIT_ACCEL_FS_SEL_2G: u8  = 0b00000000;

/*
 * BANK 1 REGISTERS (Self-test, offsets, etc.)
 */
pub const REG1_SELF_TEST_X_GYRO: u8  = 0x02;
pub const REG1_SELF_TEST_Y_GYRO: u8  = 0x03;
pub const REG1_SELF_TEST_Z_GYRO: u8  = 0x04;
pub const REG1_SELF_TEST_X_ACCEL: u8 = 0x0E;
pub const REG1_SELF_TEST_Y_ACCEL: u8 = 0x0F;
pub const REG1_SELF_TEST_Z_ACCEL: u8 = 0x10;

pub const REG1_XA_OFFS_H: u8 = 0x14; // Offset do Accelerometer (high precision, maybe is necessary to calibrate)
pub const REG1_XA_OFFS_L: u8 = 0x15;
pub const REG1_YA_OFFS_H: u8 = 0x17;
pub const REG1_YA_OFFS_L: u8 = 0x18;
pub const REG1_ZA_OFFS_H: u8 = 0x1A;
pub const REG1_ZA_OFFS_L: u8 = 0x1B;

pub const REG1_TIMEBASE_CORR_PLL: u8 = 0x28;

/*
 * BANK 2 REGISTERS (Advanced Configuration of Gyro and Accel)
 */
pub const REG2_GYRO_SMPLRT_DIV: u8    = 0x00;
pub const REG2_GYRO_CONFIG_1: u8      = 0x01;
pub const REG2_GYRO_CONFIG_2: u8      = 0x02;
pub const REG2_ACCEL_SMPLRT_DIV_1: u8 = 0x10;
pub const REG2_ACCEL_SMPLRT_DIV_2: u8 = 0x11;
pub const REG2_ACCEL_CONFIG: u8       = 0x14;
pub const REG2_ACCEL_CONFIG_2: u8     = 0x15;

/*
 * BANK 3 REGISTERS (I2C Master to Magnetômetro AK09916)
 */
pub const REG3_I2C_MST_CTRL: u8       = 0x01;
pub const REG3_I2C_MST_DELAY_CTRL: u8 = 0x02;
pub const REG3_I2C_SLV0_ADDR: u8      = 0x03;
pub const REG3_I2C_SLV0_REG: u8       = 0x04;
pub const REG3_I2C_SLV0_CTRL: u8      = 0x05;
pub const REG3_I2C_SLV0_DO: u8        = 0x06;
// ... until SLV4

pub const REG3_I2C_SLV4_CTRL: u8      = 0x15;
pub const REG3_I2C_MST_STATUS: u8     = 0x17; // also Bank 0

// Bits I2C_MST_CTRL
pub const BIT_I2C_MST_CLK_400KHZ: u8 = 0b00001101; // commom example

static mut LAST_BANK: u8 = 0xFF;

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

#[derive(Copy, Clone)]
pub enum Bank
{
    Bank0 = 0,
    Bank1 = 1,
    Bank2 = 2,
    Bank3 = 3,
}

fn check(i2c: &i2c::I2C) -> bool
{
    set_bank(i2c, Bank::Bank0);
    i2c::master::read_register8(i2c, ICM20948_ADDRESS, REG0_WHO_AM_I) == WHO_AM_I_VALUE
}

fn set_bank(i2c: &i2c::I2C, bank: Bank) -> ()
{
    unsafe
    {
        if LAST_BANK == 0xFF
        {
            LAST_BANK = i2c::master::read_register8(i2c, ICM20948_ADDRESS, REG_BANK_SEL);
        }

        let bank_bits = match bank
        {
            Bank::Bank0 => BIT_BANK_SEL_0,
            Bank::Bank1 => BIT_BANK_SEL_1,
            Bank::Bank2 => BIT_BANK_SEL_2,
            Bank::Bank3 => BIT_BANK_SEL_3,
            _ => return, // invalid bank
        };

        if LAST_BANK != bank_bits
        {
            LAST_BANK = bank_bits;
            i2c::master::write_register8(i2c, ICM20948_ADDRESS, REG_BANK_SEL, bank_bits);
        }
    }
}

pub fn init(i2c: &i2c::I2C, accel: AccelRange, gyro: GyroRange, dlpf: Dlpf) ->u8
{
    if ! check(i2c) {return 0;}

    /* DLPF */
    // i2c::master::write_register8(i2c, ICM20948_ADDRESS, REG2_GYRO_CONFIG_1, dlpf as u8);

    // Set bank2
    set_bank(i2c, Bank::Bank2);
    // Gyro range
    i2c::master::write_register8(i2c, ICM20948_ADDRESS, REG2_GYRO_CONFIG_1, (gyro as u8) << 1);
    // Accel range
    i2c::master::write_register8(i2c, ICM20948_ADDRESS, REG2_ACCEL_CONFIG, (accel as u8) << 1);
    
    // Disable I2C master | Disable FIFO | Reset sensors
    let user_ctrl:u8 = !BIT_USER_CTRL_I2C_MST_EN | !BIT_USER_CTRL_FIFO_EN | !BIT_USER_CTRL_SIG_COND_RST;
    set_bank(i2c, Bank::Bank0);
    i2c::master::write_register8(i2c, ICM20948_ADDRESS, REG0_USER_CTRL, user_ctrl);
    
    // enable magnetometer bypass
    let int_pin_cfg:u8 = BIT_INT_PIN_CFG_BYPASS_EN;
    i2c::master::write_register8(i2c, ICM20948_ADDRESS, REG0_INT_PIN_CFG, int_pin_cfg);

    return 1;
}