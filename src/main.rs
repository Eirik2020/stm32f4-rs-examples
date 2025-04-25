#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Crates
use panic_probe as _;   // Panic handler with defmt support
use cortex_m_rt::entry; // ARM dependencies for cortex-m architecture
use stm32f4xx_hal as _; // STM32F4 series HAL crate

// Debugger
use defmt::*;
use defmt_rtt as _;         // Global logger


// MAIN
#[allow(clippy::empty_loop)] 
#[entry] 
fn main() -> ! {
    info!("Use cargo embed --example <example name>, to build examples!");

    loop {}
}