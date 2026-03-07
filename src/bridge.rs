use core::u16;

use crate::{checksum, utils};

const PAYLOAD_SIZE_MAX: usize = 256;
const SOF: u8 = 0x7E;
const HEADER_SIZE:usize = core::mem::size_of::<FrameHeader>();
static mut FRAME_ID: u8 = 1;

#[repr(u8)]
pub enum FrameType
{
    FR_TYPE_VERSION = 0,
    FR_TYPE_CFG = 1,
    FR_TYPE_GPS_DATA = 2,
    FR_TYPE_COORDS = 3,
    FR_TYPE_ACC_DATA = 4,
    FR_TYPE_GYR_DATA = 5,
}

#[derive(Clone, Copy)]
pub struct FrameHeader
{
    pub size: u16,
    pub package_type: u8,
    pub frame_id: u8
}

pub struct FrameTx
{
    pub header: FrameHeader,
    pub payload: [u8; PAYLOAD_SIZE_MAX],
    pub crc: u16
}

pub fn get_header(tp: FrameType, len: u16) -> FrameHeader
{
    unsafe 
    {
        FRAME_ID = FRAME_ID.wrapping_add(1);

        FrameHeader
        {
            size: len,
            frame_id: FRAME_ID,
            package_type: tp as u8
        }
    }
}

pub fn build_frame(tp: FrameType, data: &[u8]) -> FrameTx
{
    let len = 
    if data.len() > PAYLOAD_SIZE_MAX
    {
        data.len()
    }
    else
    {
        PAYLOAD_SIZE_MAX
    };
    
    let header = get_header(tp, len as u16);
    let mut frame = FrameTx
    {
        header,
        payload: [0u8; PAYLOAD_SIZE_MAX],
        crc: 0
    };

    // copy payload
    frame.payload[..len].copy_from_slice(data);
    // get CRC16
    let bytes = utils::as_bytes(&frame);
    frame.crc = checksum::get_crc16(bytes, (data.len() + HEADER_SIZE) as u16);

    frame
}

pub fn get_gps_data(data: &[u8]) -> FrameTx
{
    build_frame(FrameType::FR_TYPE_GPS_DATA, data)
}

pub fn get_package_cfg(data: &[u8]) -> FrameTx
{
    build_frame(FrameType::FR_TYPE_CFG, data)
}

pub fn get_package_acc_date(x: f32, y: f32, z: f32) -> FrameTx
{
    let mut data:[u8; 12] = [0u8; 12];
    
    data[0..4].copy_from_slice(&x.to_le_bytes());
    data[4..8].copy_from_slice(&y.to_le_bytes());
    data[8..12].copy_from_slice(&z.to_le_bytes());

    build_frame(FrameType::FR_TYPE_ACC_DATA, &data)
}

pub fn get_package_gyr_date(x: f32, y: f32, z: f32) -> FrameTx
{
    let mut data:[u8; 12] = [0u8; 12];
    
    data[0..4].copy_from_slice(&x.to_le_bytes());
    data[4..8].copy_from_slice(&y.to_le_bytes());
    data[8..12].copy_from_slice(&z.to_le_bytes());

    build_frame(FrameType::FR_TYPE_GYR_DATA, &data)
}

pub fn get_package_coords(lat: f32, lon: f32) -> FrameTx
{
    let mut data:[u8; 8] = [0u8; 8];
    
    data[0..4].copy_from_slice(&lat.to_le_bytes());
    data[4..8].copy_from_slice(&lon.to_le_bytes());

    build_frame(FrameType::FR_TYPE_COORDS, &data)
}

pub fn verify_package(mut frame: FrameTx) -> u8
{
    let crc_rx = frame.crc;
    let bytes = utils::as_bytes(&frame);
    let crc_cl = checksum::get_crc16(&bytes, (frame.header.size + HEADER_SIZE as u16));
    
    (crc_cl == crc_rx) as u8
}