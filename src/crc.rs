use crate::{mcu, rcc, utils};

pub fn init()
{
    rcc::ahb::enable(rcc::ahb::AHBPeripheral::Crc);
}

pub fn reset()
{
    let crc_cr = mcu::CRC_CR as *mut u32;
    unsafe
    {
        utils::write_register32(crc_cr, 1); // RESET bit
    }
}

pub fn calc(data: &[u32]) -> u32
{
    let crc_dr = mcu::CRC_DR as *mut u32;

    for &word in data
    {
        unsafe
        {
            utils::write_register32(crc_dr, word);
        }
    }

    unsafe
    {
        utils::read_register32(crc_dr)
    }
}