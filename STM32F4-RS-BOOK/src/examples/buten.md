# Buten Controlled Light
Being able to have some inputs would be nice right? 
So lets try controlling the LED with the on-board button. Just like in the blinky example, we need to split out of peripherals. We also split out GPIO group C, since the on-board button B1, is connected to pin 13 in group C, PC13. 
```rust
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();
```


We then assign PC13 to B1. Notice here we don't need to declare B1 a mutable variable, since we only read data from it. (If we wanted to write to it, we had to make it mutable).  
We then check if B1 is true (high) using `.is_low()`, and set our LED LD1 high if it is.
```rust
let B1 = gpioc.pc13;

loop {
    // Check if button is pressed
    if B1.is_low() {
        LD1.set_high(); // Turn ON LED
    } else {
        LD1.set_low(); // Turn OFF LED
    }
}
```





## Complete Example
Here is a complete code example and can be run with:
```sh
$ cargo embed --example buten
```
```rust
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
    let gpioc = dp.GPIOC.split();

    // Configure pins
    let mut LD1 = gpioa.pa5.into_push_pull_output();
    LD1.set_low();
    let B1 = gpioc.pc13;

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
```
(btw, buten is a inside joke, I am not illiterate)