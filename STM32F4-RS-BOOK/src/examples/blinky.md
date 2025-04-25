# Blinky
Blinky will be our first attempt at using the STM32F4 HAL crate. We use this crate to access the peripherals of the STM32, safely wrapped in rust. 

From the STM32F4 HAL we import the Peripheral Access Crate (PAC) and the preludes. The preludes import some generics and traits for embedded HAL that saves us some typing. 
The PAC is how we access the peripherals of the microcontroller unit (MCU). 
```rust
use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
};
```

## What is a PAC?
Peripheral Access Crate (PAC) is how we access the peripherals of the microcontroller unit (MCU). 
To use the peripherals, we need to assign ownership of the device peripherals. This is required in rust to manage memory. 
```rust
    let dp = pac::Peripherals::take().unwrap();
```

From the device peripherals, we split out the struct gpioa, which is the GPIO group A on the microcontroller. We can see in the datasheet that this group contains pins 0-15 (PA0-15).
```rust
    let gpioa = dp.GPIOA.split();
```

We can assign a single pin to a variable and configure it, in this case we want to configure it into a push/pull output, to either connect our LED to power or ground. 
```rust
    let mut LD1 = gpioa.pa5.into_push_pull_output();
```

Now that we have configured PA5 into a push/pull output with the name LD1, which correspondes with the LD1 LED on the ST NucleoF401RE board. We probably want to do something with it right? 
We can turn it off, with the following commands:
```rust
    LD1.set_low();
    LD1.set_high();
```

We also want it to blink, we can use `cortex_m::asm::delay(8_000_000);`, this simply pauses the program for 8 000 000 million clock cycles, and since our clock runs at 8MHz, that equates to 1 second.  
By adding a conversion factor, we can use milliseconds instead, giving us a simple blinky program:
```rust
// Clock-cycles to millisecond conversion factor
let ms: u32 = 8_000;
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
```


## Complete Example
Here is a complete code example, it is the default example, and can be run with:
```sh
$ cargo build --example blinky
$ cargo embed
```

```rust
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
```