#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    delay::Delay,
    i2c::{ I2c, Instance, Mode },
    pac::{ self, sai::ch::im, GPIOG },
    prelude::*,
};
use core::panic::PanicInfo;
use rtt_target::{ rtt_init_print, rprintln };

use mpu60x0::{ Mpu60x0, error::Mpu60x0Error };

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk((84).mhz()).freeze();
    let _ = device.SYSCFG.constrain();

    let gpiog = device.GPIOG.split();
    let gpiob = device.GPIOB.split();

    let mut led = gpiog.pg13.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &clocks);

    rprintln!("Hello, world!");

    let i2c = I2c::new(
        device.I2C1,
        (gpiob.pb6, gpiob.pb7),
        Mode::Fast {
            frequency: (400_000).hz(),
            duty_cycle: stm32f4xx_hal::i2c::DutyCycle::Ratio2to1,
        },
        clocks
    );

    let mut mpu = Mpu60x0::new(i2c);

    match mpu.init() {
        Ok(_) => rprintln!("MPU60X0 initialized"),
        Err(e) => rprintln!("MPU60X0 initialization failed: {:?}", e),
    }

    loop {
        match mpu.read_gyro() {
            Ok(data) => {
                rprintln!("Gyro data: x: {}, y: {}, z: {}", data.x, data.y, data.z);
            }
            Err(e) => {
                rprintln!("Error reading gyro data: {:?}", e);
            }
        }

        delay.delay_ms(1000_u32);
    }
}
