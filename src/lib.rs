#![no_std]

use core::{ pin, result::Result::{ self, Ok } };
use embedded_hal::blocking::i2c::{ Write, WriteRead };

use error::Mpu60x0Error;
use registers::{
    CONFIG,
    FIFO_DATA,
    FIFO_EN,
    FIFO_COUNT_H,
    FIFO_COUNT_L,
    GYRO_CONFIG,
    I2C_MST_CTRL,
    MPU60X0_ADDRESS,
    PWR_MGMT_1,
    SMPLRT_DIV,
    USER_CTRL,
    WHO_AM_I,
};
use rtt_target::rprintln;

mod registers;
pub mod error;

pub struct Mpu60x0<I2C> {
    i2c: I2C,
}

pub struct GyroData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl<I2C: Write + WriteRead> Mpu60x0<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Mpu60x0 { i2c }
    }

    fn write_at_address(&mut self, address: u8, value: u8) -> Result<(), Mpu60x0Error> {
        rprintln!("write_at_address: reg=0x{:02X} val=0x{:02X}", address, value);
        self.i2c
            .write(MPU60X0_ADDRESS, &[address, value])
            .map_err(|_| { Mpu60x0Error::i2c_error() })
    }

    fn read_address(&mut self, address: u8) -> Result<u8, Mpu60x0Error> {
        let mut buffer = [0; 1];
        self.i2c
            .write_read(MPU60X0_ADDRESS, &[address], &mut buffer)
            .map_err(|_| { Mpu60x0Error::i2c_error() })?;
        Ok(buffer[0])
    }

    fn delay_ms(&mut self, ms: u32) {
        cortex_m::asm::delay(ms);
    }

    pub fn ping(&mut self) -> Result<(), Mpu60x0Error> {
        if self.read_address(WHO_AM_I)? != MPU60X0_ADDRESS {
            return Err(Mpu60x0Error::device_not_found());
        }
        Ok(())
    }

    pub fn init(&mut self) -> Result<(), Mpu60x0Error> {
        self.ping()?;

        // 1. Reset device
        self.write_at_address(PWR_MGMT_1, 0x80)?;
        self.delay_ms(100);

        // Désactive le standby sur tous les axes (active gyro/accel)
        self.write_at_address(0x6c, 0x00)?;

        self.write_at_address(FIFO_EN, 0x70)?;

        self.write_at_address(USER_CTRL, 0x40)?; // Enable FIFO

        self.write_at_address(SMPLRT_DIV, 0x31)?;

        // 3. Disable I2C master mode
        self.write_at_address(I2C_MST_CTRL, 0x00)?;

        // 4. Reset FIFO, puis enable FIFO
        self.write_at_address(USER_CTRL, 0x04)?; // Reset FIFO

        // 7. Configure DLPF (low-pass filter)
        self.write_at_address(CONFIG, 0x04)?;

        Ok(())
    }

    pub fn read_gyro(&mut self) -> Result<GyroData, Mpu60x0Error> {
        let mut buffer = [0; 6];

        // let gyro_h: u16 = self.read_address(FIFO_COUNT_H)?.into();
        // let gyro_l: u16 = self.read_address(FIFO_COUNT_L)?.into();
        // let count = (gyro_h << 8) + gyro_l;
        // if count < 6 {
        //     return Err(Mpu60x0Error::not_enough_data(count));
        // }

        // Lecture burst de 6 octets d'un coup
        for i in 0..6 {
            buffer[i] = self.read_address(FIFO_DATA)?;
        }

        let x = i16::from_be_bytes([buffer[0], buffer[1]]);
        let y = i16::from_be_bytes([buffer[2], buffer[3]]);
        let z = i16::from_be_bytes([buffer[4], buffer[5]]);

        Ok(GyroData { x, y, z })
    }
}
