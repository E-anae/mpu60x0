#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32f4xx_hal::{
    delay::Delay,
    i2c::{ I2c, Instance, Mode },
    pac::{ self, sai::ch::im, GPIOG },
    prelude::*,
};
use rtt_target::{ rtt_init_print, rprintln };

use mpu60x0::{ Mpu60x0, error::Mpu60x0Error, MPU60X0_ADDRESS };

struct Mpu60x0Impl<I2C: Instance, Pins> {
    i2c: I2c<I2C, Pins>,
}

impl<I2C: Instance, Pins> Mpu60x0Impl<I2C, Pins> {
    fn new(i2c: I2c<I2C, Pins>) -> Self {
        Mpu60x0Impl { i2c }
    }
}

impl<I2C: Instance, Pins> Mpu60x0 for Mpu60x0Impl<I2C, Pins> {
    fn write_at_address(&mut self, address: u8, value: u8) -> Result<(), Mpu60x0Error> {
        self.i2c.write(address, &[value]).map_err(|_| Mpu60x0Error::i2c_error())
    }

    fn read_address(&mut self, address: u8) -> Result<u8, Mpu60x0Error> {
        let mut buffer = [0; 1];
        self.i2c.read(0x68, &mut buffer).map_err(|_| Mpu60x0Error::i2c_error())?;
        rprintln!("0x{:02X}", buffer[0]);
        Ok(buffer[0])
    }

    fn delay_ms(&mut self, ms: u32) {
        cortex_m::asm::delay(ms);
    }
}

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

    let mut mpu = Mpu60x0Impl::new(i2c);

    loop {
        led.toggle();
        delay.delay_ms(1000u16);
        match mpu.ping() {
            Ok(_) => rprintln!("MPU60X0 is responding"),
            Err(e) => rprintln!("MPU60X0 not responding: {:?}", e),
        }
    }
}
