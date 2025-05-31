#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;
use core::fmt::Write;
use stm32f4xx_hal::{
    pac,
    prelude::*,
    serial::{config::Config, Serial},
};
use nb::block;
use heapless::String;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let tx = gpioa.pa2.into_alternate();
    let rx = gpioa.pa3.into_alternate();
    let serial_config = Config::default().baudrate(115_200.bps());
    let serial = Serial::new(dp.USART2, (tx, rx), serial_config, &clocks).unwrap();

    let (mut tx, mut rx) = serial.split();

    let ms: u32 = 8_000;
    let mut buf: String<64> = String::new();

    loop {
        if let Ok(byte) = block!(rx.read()) {
            if byte == b'\n' || byte == b'\r' {
                let trimmed = buf.trim();
                if let Some(arg) = trimmed.strip_prefix("thunder ") {
                    if let Ok(val) = arg.parse::<u32>() {
                        writeln!(tx, "LIGHTNING {}\r", val).ok();
                    } else {
                        writeln!(tx, "Invalid number\r").ok();
                    }
                }
                buf.clear();
            } else {
                if buf.push(byte as char).is_err() {
                    buf.clear(); // reset buffer if overflow
                }
            }
        }

        cortex_m::asm::delay(500 * ms);
    }
}
