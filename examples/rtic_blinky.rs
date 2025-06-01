// ####  SET-UP  ####
// Compiler directives
#![deny(unsafe_code)]
#![no_main]
#![no_std]


// Imports
// Debugger output for RTT
use rtt_target::{rprintln, rtt_init_print};

// Panic handler for RTT
use panic_rtt_target as _;

// RTIC
use rtic::app;
use rtic_monotonics::systick::prelude::*;

// STM32F4 HAL
use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
    gpio::{Output, PushPull, PA5},
};


// Set monotonic time to 1000 Hz, 1 ms resolution.
systick_monotonic!(Mono, 1000);




#[app(
    device = stm32f4xx_hal::pac,  // This device uses the stm32f4xx_hal Peripheral Access Crate (PAC).
    peripherals = true,           // Auto-initializes the Peripherals struct (dp).
    dispatchers = [SPI1],         // Unused interrupts that RTIC can use internally for software tasks. 
)]
mod app {
    // Import everything (*) from the parent module (rtic_blinky.rs)
    use super::*;

    // Resources
    #[shared] // Shared between different tasks
    struct Shared {}

    #[local] // Task local data only
    struct Local {
        led: PA5<Output<PushPull>>, // LED pin
        state: bool,                // LED state (ON/OFF)
    }


    #[init] // Start-up function that initializes the program.
    fn init(cx: init::Context) -> (Shared, Local) {
        // Assign context device peripherals to dp.
        let dp = cx.device;

        // Initialize the systick interrupt & obtain the token to prove that we did
        Mono::start(cx.core.SYST, 8_000_000); // default STM32F401 clock-rate is 8MHz

        // Report that the program successfully started.
        rtt_init_print!();
        rprintln!("init");

        // Setup LED
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();
        led.set_low();

        // Schedule the blinking task
        blink::spawn().ok();

        // Initialize resources
        (Shared {}, Local { led, state: false })
    }



    // ####  TASKS  ####
    #[task(local = [led, state])] // This task uses the local resources "led" and "state".
    async fn blink(cx: blink::Context) { // Use context cx to access local and shared resources.
        loop {
            rprintln!("blink");
            // Access local resources from context (cx.local)
            if *cx.local.state {         // If LED is on.
                cx.local.led.set_high();
                *cx.local.state = false;
            } else {                     // If LED is off
                cx.local.led.set_low();
                *cx.local.state = true;
            }
            // At the end of the task, wait 1000 ms (none-blocking).
            Mono::delay(1000.millis()).await;
        }
    }
}