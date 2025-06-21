#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    i2c::{ I2c, Mode },
    pac::{ self },
    prelude::*,
    time::Hertz,
};
use rtt_target::{ rtt_init_print, rprintln };

use mpu60x0::Mpu60x0;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let device = pac::Peripherals::take().unwrap();

    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(Hertz::MHz(84)).freeze();
    let _ = device.SYSCFG.constrain();

    let gpiob = device.GPIOB.split();

    let mut delay = device.TIM2.delay::<84_000_000>(&clocks);

    rprintln!("Hello, world!");

    let i2c = I2c::new(
        device.I2C1,
        (gpiob.pb6, gpiob.pb7),
        Mode::Fast {
            frequency: Hertz::Hz(400_000),
            duty_cycle: stm32f4xx_hal::i2c::DutyCycle::Ratio2to1,
        },
        &clocks
    );

    let mpu = Mpu60x0::new(i2c);

    let mut mpu = match mpu.enable(&mut delay) {
        Ok(mpu) => {
            rprintln!("MPU60X0 initialized");
            mpu
        }
        Err(e) => {
            rprintln!("MPU60X0 initialization failed: {:?}", e);
            panic!("MPU60X0 initialization failed");
        }
    };

    loop {
        match mpu.read_fifo() {
            Ok(data) => {
                rprintln!(
                    "Gyro: x: {}, y: {}, z: {}",
                    data.gyro_data.x,
                    data.gyro_data.y,
                    data.gyro_data.z
                );
                rprintln!(
                    "Accel: x: {}, y: {}, z: {}",
                    data.accel_data.x,
                    data.accel_data.y,
                    data.accel_data.z
                );
            }
            Err(_) => (),
        }
    }
}
