use crate::utils;

const PAYLOAD_SIZE_MAX: usize = 256;
const SOF: u8 = 0x7E;
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
    let len = data.len();

    let header = get_header(tp, len as u16);

    let mut frame = FrameTx
    {
        header,
        payload: [0u8; PAYLOAD_SIZE_MAX],
        crc: 0
    };

    // copia payload
    frame.payload[..len].copy_from_slice(data);

    // calcula CRC
    frame.crc = utils::get_crc16(&frame.payload[..len], frame.payload.len() as u16);

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