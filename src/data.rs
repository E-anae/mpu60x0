pub struct FifoData {
    pub gyro_data: GyroData,
    pub accel_data: AccelData,
}

pub struct GyroData {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

pub struct AccelData {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

impl FifoData {
    pub fn from_buffer(buffer: [u8; 12]) -> Self {
        FifoData {
            gyro_data: GyroData {
                x: u16::from_be_bytes([buffer[6], buffer[7]]),
                y: u16::from_be_bytes([buffer[8], buffer[9]]),
                z: u16::from_be_bytes([buffer[10], buffer[11]]),
            },
            accel_data: AccelData {
                x: u16::from_be_bytes([buffer[0], buffer[1]]),
                y: u16::from_be_bytes([buffer[2], buffer[3]]),
                z: u16::from_be_bytes([buffer[4], buffer[5]]),
            },
        }
    }
}
