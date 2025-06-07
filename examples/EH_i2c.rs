// ========================== Embedded Rust Set-up ==========================
#![deny(unsafe_code)]
#![no_main]
#![no_std]


// Imports
use defmt::*;
use defmt_rtt as _; // Global logger
use panic_probe as _; // Panic handler with defmt support
use cortex_m_rt::entry;
use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
    i2c::I2c,
};

// This library
use library::As5600; // Import your library


#[allow(non_snake_case)]
#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    // ========================== Set-up ==========================
    // Take ownership of device peripherals and split out GPIO group A and B
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    // ========================= I2C Setup ==========================
    let gpiob = dp.GPIOB.split();

    let scl = gpiob.pb8.into_alternate().set_open_drain();
    let sda = gpiob.pb9.into_alternate().set_open_drain();

    let i2c = I2c::new(dp.I2C1, (scl, sda), 100.kHz(), &clocks);
    let mut encoder = As5600::new(i2c);

    // Variables
    let ms: u32 = 8_000; // clock cycles to millisecond conversion.


    // ========================== Main Loop ==========================
    loop {
        if let Ok(angle) = encoder.read_degrees() {
            defmt::info!("Rotor position = {}°", angle);
        } 

        // Wait 500ms
        cortex_m::asm::delay(200 * ms);
    }
}