#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _; // Global logger
use panic_probe as _;
use core::fmt::Write;
use stm32f4xx_hal::{pac, prelude::*, 
    serial::config::Config,
    serial::Serial,
};


#[entry]
fn main() -> ! {
    // ========================== Set-up ==========================
    // Take ownership of device peripherals and split out GPIO group A and B
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();

    // Configure clocks
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    // Configure UART
    let tx = gpioa.pa2.into_alternate();
    let rx = gpioa.pa3.into_alternate();
    let serial_config = Config::default().baudrate(115_200.bps());
    let mut serial = Serial::new(dp.USART2, (tx, rx), serial_config, &clocks).unwrap();

    // Create message
    let x = 2;
    let y = 2;
    let z = x + y;
    
    // Calculate delay conversion constant
    let ms: u32 = 8_000;


    loop {
        // Wait 500ms
        cortex_m::asm::delay(500 * ms);

        // Send message
        writeln!(serial, "Obiously {} + {} = {}\r\n right?", x, y, z).ok();
    }
}