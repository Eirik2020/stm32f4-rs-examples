# Dimming a LED using a potentiometer
Lets try controlling the brightness of a LED using a potentiometer. To do things we need to measure the analog voltage from a potentiometer.  
To do this, we need to import the adc peripheral from the HAL library:
```rust
use stm32f4xx_hal::{
    pac::{self},
    adc::{config::AdcConfig, config::SampleTime, Adc},
    prelude::*,
};
```

Then we configure our pin PA0 into a analog pin, then configure the adc connected to it (ADC1), as ADC. 
```rust
// Configure ADC
let dimmer = gpioa.pa0.into_analog();
let mut adc = Adc::adc1(dp.ADC1, true, AdcConfig::default());
```

Now, we can read the raw analog value using adc.convert.
```rust
// Read Dimmer 
let duty = adc.convert(&dimmer, SampleTime::Cycles_480);
```




## Complete Example
Here is a complete code example and can be run with:
```sh
$ cargo embed --example potmeter_dimmer
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
use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f4xx_hal::{
    pac::{self},
    adc::{config::AdcConfig, config::SampleTime, Adc},
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

    // Configure clocks
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    // Configure ADC
    let dimmer = gpioa.pa0.into_analog();
    let mut adc = Adc::adc1(dp.ADC1, true, AdcConfig::default());

    // Configure PWM
    let (_, (LD1_pwm, ..)) = dp.TIM2.pwm_hz(2000.Hz(), &clocks);
    let mut LD1_pwm = LD1_pwm.with(gpioa.pa5);
    LD1_pwm.enable();

    // Calculate conversion factor from clock cycles to ms, assuming 8 MHz
    let ms: u32 = 8_000;


   // ========================== LOOP ==========================
    loop {
        // Read Dimmer 
        let duty = adc.convert(&dimmer, SampleTime::Cycles_480);

        // Set LED duty
        LD1_pwm.set_duty(duty);  // Turn OFF LED

        // Print Duty
        info!("Dimmer = {}", duty);

        // Delay until next cycle
        asm::delay(100 * ms); 

    }
}
```