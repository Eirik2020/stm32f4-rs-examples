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

            // Optional: wait until button released
            while B1.is_high() {}
        }
    }

    // Set PWM for LD1
    let duty = (max_duty * counter) / 10; // 10 steps (0%, 10%, ..., 100%)
    LD1_pwm.set_duty(duty);
}
}