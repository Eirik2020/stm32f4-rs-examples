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
use heapless::String; // fixed-capacity string
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

    // Split serial object into receiver and transmitter.
    let (mut tx, mut rx) = serial.split();

    // Message buffer
    let mut buffer: String<64> = String::new();

    // First loop check
    let mut first_loop: bool = true;



    loop {
        loop { // Wait for a full line of input
            if let Ok(byte) = rx.read() { // If a UART byte is received:
                let c = byte as char;   // Convert byte to char.
                if c == '\r' || c == '\n' {   // If it is a carrige return or line break, end loop.
                    break;
                } else {
                    buffer.push(c).ok();     // Else, add char to buffer.
                }
            }
        }

        // Run CLI application
        // Greet user on first loop.
        if first_loop {
            writeln!(tx, "\r\nWelcome to the STM32 UART Menu! \r\n").ok();
            first_loop = false;
            }

        // Display Menu
        print_menu(&mut tx); // Print menu

        // Respond to user
        match buffer.trim() {
            "1" => writeln!(tx, "You selected option 1: LED ON"),
            "2" => writeln!(tx, "You selected option 2: LED OFF"),
            "3" => writeln!(tx, "You selected option 3: STATUS OK"),
            "4" => writeln!(tx, "You selected option 4: Resetting..."),
            _   => writeln!(tx, "ERROR! Invalid command: {}", buffer),
        }.ok();

        // Clear buffer
        buffer.clear();

        // Clear space for next response
        writeln!(tx, "\r\n\n\n").ok();
    }
}


// Functions
fn print_menu<W: Write>(tx: &mut W) {
    writeln!(tx, "== Menu == \r\n").ok();
    writeln!(tx, "1. Turn LED ON \r\n").ok();
    writeln!(tx, "2. Turn LED OFF \r\n").ok();
    writeln!(tx, "3. Print STATUS \r\n").ok();
    writeln!(tx, "4. Reset \r\n").ok();
}