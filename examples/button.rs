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
};


#[allow(non_snake_case)]
#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    // ========================== Set-up ==========================
    // Take ownership of device peripherals and split out GPIO group A and B
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();

    // Configure pins
    let mut LD1 = gpioa.pa5.into_push_pull_output();
    LD1.set_low();
    let B1 = gpioc.pc13;
   
   // Debug
   info!("Hello from defmt!");


   // ========================== LOOP ==========================
    loop {
        // Check if button is pressed
        if B1.is_low() {
            LD1.set_high(); // Turn ON LED
        } else {
            LD1.set_low(); // Turn OFF LED
        }
    }
}
// Hello