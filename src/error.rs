use core::{ error::Error, fmt::{ Display, Debug } };

#[derive(Debug)]
pub enum ErrorKind {
    CustomError,
    I2cError,
    DeviceNotFound,
    InvalidData,
}

#[derive(Debug)]
pub struct Mpu60x0Error {
    pub kind: ErrorKind,
    pub message: &'static str,
}

impl Mpu60x0Error {
    pub fn new(message: &'static str) -> Self {
        Mpu60x0Error {
            kind: ErrorKind::CustomError,
            message,
        }
    }

    pub fn device_not_found() -> Self {
        Mpu60x0Error {
            kind: ErrorKind::DeviceNotFound,
            message: "Device not found",
        }
    }

    pub fn i2c_error() -> Self {
        Mpu60x0Error {
            kind: ErrorKind::I2cError,
            message: "I2C error",
        }
    }

    pub fn invalid_data() -> Self {
        Mpu60x0Error {
            kind: ErrorKind::InvalidData,
            message: "Invalid data",
        }
    }
}

impl Display for Mpu60x0Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[{:?}] {}", self.kind, self.message)
    }
}

impl Error for Mpu60x0Error {
    fn description(&self) -> &str {
        self.message
    }
}
