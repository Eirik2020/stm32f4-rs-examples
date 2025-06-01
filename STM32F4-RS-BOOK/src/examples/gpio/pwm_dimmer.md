# Dimming a LED using PWM
Now lets try ourselvs at something a bit more advanced. A LED that is dimmed with the help of a button.
We will dim it using PWM, but before we can do this, we need to configure the clocks and timers, which in turn control our PWM. 


We give ownership of the reset and clock control to `rcc`, from `dp`.  
Then configure(`.cfgr`) it from a high speed external(`use_hse()`) clock running at 8 MHz(`8.MHz()`) and lock the configuration(`.freeze()`).
```rust
// Configure clocks
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
```


Now to the juicy part, we actually configure one of our timers (Timer 2, TIM2) for PWM.
To do this, we do this:
```rust
// Configure PWM
let (_, (LD1_pwm, ..)) = dp.TIM2.pwm_hz(2000.Hz(), &clocks);
```
So what is going on here?  
Here we take timer 2(`TIM2`) from the device peripheral handler(`dp`), and configure it for PWM with hertz as a input (`.pwm_hz()`). And set the PWM frequency to 2000 Hz (`.Hz()`), and use our on-board clock as a reference (`&clocks`).  
Now why do we assign it to `(_, (LD1_pwm, ..))`?  
This is because it returns an array, where the first spot is the PWM controller, which we ignore with (`_`), then the second spot is all the PWM channels
`(LD1_pwm, ..))`. The first channel is assigned to `LD1_pwm`, while we ignore the rest with `, ..)`, it would be the same as putting `_` for all the other channels. 
Can I do PWM now? No, have patience my child. We have our PWM channel, we now need to assign it to a suitable pin,
so we assign `LD1_pwm` to pin 5 in GPIO group A (`pa5`):
```rust
let mut LD1_pwm = LD1_pwm.with(gpioa.pa5);
let max_duty = LD1_pwm.get_max_duty();
LD1_pwm.enable();
```
We also use `.get_max_duty()` to get the maximum PWM value, and `.enable()` to enable the channel.  

Finally, we can set the PWM duty cycle for PA5, we do this with `.set_duty()`, and give it a division of `max_duty`.  
In this case divided by 4 or 25% duty cycle.
```rust
LD1_pwm.set_duty(max_duty / 4); // 25% duty for LED LD1. 
```

Now lets make it even cooler. Lets have 10 levels (0-100%), controlled by a button!
```rust
// Initialize counter variable
    let mut counter = 0;

    // Calculate conversion factor from clock cycles to ms, assuming 8 MHz
    let ms: u32 = 8_000;
    
   loop {
    if B1.is_low() {
        cortex_m::asm::delay(50 * ms); // debounce delay
        if B1.is_low() {
            counter = (counter + 1) % 11; // increment and roll over after 10 (0-10 total 11 states)

            // Wait until button released
            while B1.is_high() {}
        }
    }

    // Set PWM for LD1
    let duty = (max_duty * counter) / 10; // 10 steps (0%, 10%, ..., 100%)
    LD1_pwm.set_duty(duty);
}
```
Here we initialize the counter and calculate the clock-cycle to ms factor.  
In the main loop we check if our button is pressed(`if B1.is_low()`), if it is, we wait for 50ms(`cortex_m::asm:delay(50*ms)`), then if it still is pressed, we increment the counter. The reason for the delay is a quick-and-dirty debounce of the button. To increment to counter from 0-10, we use this `counter = (counter + 1) % 11`. Then wait for the button to be unpressed (`while B1.is_high()`), so we don't increment more than once.  

Outside the loop, we calculate the desired duty cycle from the counter and apply it to our LED, LD1.


## Complete Example
Here is a complete code example and can be run with:
```sh
$ cargo embed --example pwm_dimmer
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

    // Configure clocks
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    // Configure pins
    let B1 = gpioc.pc13;

    // Configure PWM
    let (_, (LD1_pwm, ..)) = dp.TIM2.pwm_hz(2000.Hz(), &clocks);
    let mut LD1_pwm = LD1_pwm.with(gpioa.pa5);
    let max_duty = LD1_pwm.get_max_duty();
    LD1_pwm.enable();

    // Initialize counter variable
    let mut counter = 0;

    // Calculate conversion factor from clock cycles to ms, assuming 8 MHz
    let ms: u32 = 8_000;
    

   // ========================== LOOP ==========================
   loop {
    if B1.is_low() {
        cortex_m::asm::delay(50 * ms); // debounce delay
        if B1.is_low() {
            counter = (counter + 1) % 11; // increment and roll over after 10 (0-10 total 11 states)

            // Wait until button released
            while B1.is_high() {}
        }
    }

    // Set PWM for LD1
    let duty = (max_duty * counter) / 10; // 10 steps (0%, 10%, ..., 100%)
    LD1_pwm.set_duty(duty);
}
}
```