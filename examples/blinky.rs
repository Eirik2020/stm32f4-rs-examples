// ========================== Embedded Rust Set-up ==========================
#![deny(unsafe_code)]
#![no_main]
#![no_std]


// Imports
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

    // Configure pins
    let mut LD1 = gpioa.pa5.into_push_pull_output();
    LD1.set_low();

    // Calculate conversion factor from clock cycles to ms, assuming 8 MHz
    let ms: u32 = 8_000;
    
   
  
   // ========================== LOOP ==========================
    loop {
        // Wait 500ms
        cortex_m::asm::delay(500 * ms);

        // Turn on LED LD1
        LD1.set_high();

        // Wait 500ms
        cortex_m::asm::delay(500 * ms);

        // Turn off LED LD1
        LD1.set_low();

        
    }
}
// Hello