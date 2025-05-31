// Compiler directives
#![no_std]
#![no_main]

// Libraries
// Generic
use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

// UART Specific
use core::fmt::Write; // Used for formatted text over UART. 
use stm32f4xx_hal::{
    pac,
    prelude::*,
    serial::{
        config::Config, // Struct for storing the UART configuration.
        Serial // Struct used to initialize UART and pin configuration. 
    },
};



#[entry]
fn main() -> ! {
    // Take ownership of peripherals and configure clocks
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    // Split out GPIO group A and configure rx and tx pins.
    let gpioa = dp.GPIOA.split();
    let tx = gpioa.pa2.into_alternate();
    let rx = gpioa.pa3.into_alternate();

    // Configure UART communication for 115200 baud rate on USART2. 
    let serial_config = Config::default().baudrate(115_200.bps());
    let mut serial = Serial::new(dp.USART2, (tx, rx), serial_config, &clocks).unwrap();

    // Clock cycle to ms conversion factor
    let ms: u32 = 8_000;


    loop {
        // Send UART message
        writeln!(serial, "hello world!\r").ok();

        // Wait for 1000 ms
        cortex_m::asm::delay(1000 * ms); // 1 second delay
    }
}