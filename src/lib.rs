#![no_std]

use core::marker::PhantomData;
use embedded_hal::{ i2c::I2c, delay::DelayNs };
use error::Mpu60x0Error;
use data::{ AccelData, FifoData, GyroData };
use registers::{
    CONFIG,
    FIFO_COUNT_H,
    FIFO_COUNT_L,
    FIFO_DATA,
    FIFO_EN,
    GYRO_CONFIG,
    MPU60X0_ADDRESS,
    PWR_MGMT_1,
    PWR_MGMT_2,
    SMPLRT_DIV,
    USER_CTRL,
    WHO_AM_I,
};

mod registers;
mod data;
pub mod error;

pub struct Mpu60x0<I2C, State: Sealed> {
    i2c: I2C,
    _state: PhantomData<State>,
}

impl<I2C: I2c, State: Sealed> Mpu60x0<I2C, State> {
    fn write_at_address(&mut self, address: u8, value: u8) -> Result<(), Mpu60x0Error> {
        self.i2c.write(0x68, &[address, value]).map_err(|_| { Mpu60x0Error::i2c_error() })
    }

    fn read_address(&mut self, address: u8) -> Result<u8, Mpu60x0Error> {
        let mut buffer = [0; 1];
        self.i2c
            .write_read(0x68, &[address], &mut buffer)
            .map_err(|_| { Mpu60x0Error::i2c_error() })?;
        Ok(buffer[0])
    }

    pub fn ping(&mut self) -> Result<(), Mpu60x0Error> {
        if self.read_address(WHO_AM_I)? != MPU60X0_ADDRESS {
            return Err(Mpu60x0Error::device_not_found());
        }
        Ok(())
    }
}

impl<I2C: I2c> Mpu60x0<I2C, Disabled> {
    pub fn new(i2c: I2C) -> Self {
        Mpu60x0 { i2c, _state: PhantomData }
    }

    pub fn enable<D: DelayNs>(
        mut self,
        delay: &mut D
    ) -> Result<Mpu60x0<I2C, Enabled>, Mpu60x0Error> {
        self.ping()?;

        // 1. Reset device
        self.write_at_address(PWR_MGMT_1, 0x80)?;
        delay.delay_ms(100);

        // 2. Wake up and set clock source
        self.write_at_address(PWR_MGMT_1, 0x01)?;
        delay.delay_ms(10);

        // 3. Enable all sensors (disable standby)
        self.write_at_address(PWR_MGMT_2, 0x00)?;
        delay.delay_ms(10);

        // 4. Reset FIFO
        self.write_at_address(USER_CTRL, 0x04)?;
        delay.delay_ms(10);

        // 5. Enable FIFO logic
        self.write_at_address(USER_CTRL, 0x40)?;
        delay.delay_ms(10);

        // 6. Enable gyro, accel and temp data to FIFO (do this AFTER enabling FIFO logic)
        self.write_at_address(FIFO_EN, 0x78)?;
        delay.delay_ms(10);

        // 7. Configure sample rate, DLPF, gyro config
        self.write_at_address(SMPLRT_DIV, 0x31)?;
        self.write_at_address(CONFIG, 0x04)?;
        self.write_at_address(GYRO_CONFIG, 0x00)?;
        self.write_at_address(ACCEL_CONFIG, 0x00)?;

        Ok(Mpu60x0 { i2c: self.i2c, _state: PhantomData })
    }
}

impl<I2C: I2c> Mpu60x0<I2C, Enabled> {
    pub fn disable(mut self) -> Mpu60x0<I2C, Disabled> {
        // Reset device to disable all sensors and FIFO
        self.write_at_address(PWR_MGMT_1, 0x80).unwrap();
        Mpu60x0 { i2c: self.i2c, _state: PhantomData }
    }

    pub fn read_fifo(&mut self) -> Result<FifoData, Mpu60x0Error> {
        let mut buffer = [0; 12];

        // Read FIFO_COUNT_H and FIFO_COUNT_L
        let fifo_h = self.read_address(FIFO_COUNT_H)?.into();
        let fifo_l = self.read_address(FIFO_COUNT_L)?.into();
        let count = u16::from_be_bytes([fifo_h, fifo_l]);

        if count < 12 {
            return Err(Mpu60x0Error::not_enough_data(count));
        }

        for i in 0..12 {
            buffer[i] = self.read_address(FIFO_DATA)?;
        }

        Ok(FifoData::from_buffer(buffer))
    }

    pub fn read_gyro(&mut self) -> Result<GyroData, Mpu60x0Error> {
        let fifo_data = self.read_fifo()?;

        Ok(fifo_data.gyro_data)
    }

    pub fn read_accel(&mut self) -> Result<AccelData, Mpu60x0Error> {
        let fifo_data = self.read_fifo()?;
        Ok(fifo_data.accel_data)
    }
}

mod sealed {
    pub trait Sealed {}
}

use sealed::Sealed;

use crate::registers::ACCEL_CONFIG;

pub struct Enabled;
pub struct Disabled;

impl Sealed for Enabled {}
impl Sealed for Disabled {}
