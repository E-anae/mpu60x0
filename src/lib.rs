#![no_std]

use core::{ panic::PanicInfo, result::Result::{ self, Ok } };

use error::Mpu60x0Error;
use registers::{ PWR_MGMT_1, WHO_AM_I };

mod registers;
pub mod error;

pub const MPU60X0_ADDRESS: u8 = 0x68;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}

pub trait Mpu60x0 {
    fn write_at_address(&mut self, address: u8, value: u8) -> Result<(), Mpu60x0Error>;
    fn read_address(&mut self, address: u8) -> Result<u8, Mpu60x0Error>;
    fn delay_ms(&mut self, ms: u32);

    fn ping(&mut self) -> Result<(), Mpu60x0Error> {
        if self.read_address(WHO_AM_I)? != MPU60X0_ADDRESS {
            return Err(Mpu60x0Error::device_not_found());
        }
        Ok(())
    }

    fn init(&mut self) -> Result<(), Mpu60x0Error> {
        if self.read_address(WHO_AM_I)? != MPU60X0_ADDRESS {
            return Err(Mpu60x0Error::device_not_found());
        }

        self.write_at_address(PWR_MGMT_1, 0x80)?;

        // Wait for 100ms
        self.delay_ms(100);

        self.write_at_address(PWR_MGMT_1, 0x01)?;

        Ok(())
    }

    fn enable_gyro(&mut self) -> Result<(), Mpu60x0Error> {
        // Enable gyro
        self.write_at_address(0x1b, 0x00)?;

        Ok(())
    }

    fn enable_accel(&mut self) -> Result<(), Mpu60x0Error> {
        // Enable accel
        self.write_at_address(0x1c, 0x00)?;

        Ok(())
    }
}
