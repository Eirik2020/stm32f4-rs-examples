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
    adc::{config::AdcConfig, config::SampleTime, Adc},
    i2c::I2c,
};


#[allow(non_snake_case)]
#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    // ========================== Set-up ==========================
    // Take ownership of device peripherals and split out GPIO group A and B
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();

    // Configure clocks
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

   // ========================== Constants ==========================
    const AS5600_ADDR: u8 = 0x36;       // 7-bit I2C address
    const AS5600_RAW_ANGLE_REG: u8 = 0x0C; // MSB of raw angle
    let ms: u32 = 8_000; // clock cycles to millisecond conversion.
    let mut err: f32 = 0.0; // Error value
    let mut duty: u16 = 0; // Error value
    let mut ang_rotor: u16 = 0;
    let mut integral: f32 = 0.0;
    let dt = 0.1; // 100 ms loop
    let mut set: f32 = 0.0;
    let mut set_point_raw: f32 = 0.0;
    let mut set_point_filtered: f32 = 0.0;
    let mut rotor_ang_raw: f32 = 0.0;
    let mut rotor_ang_filtered: f32 = 0.0;
    let mut a1: f32 = 0.09;
    let mut a2: f32 = 0.09;


    // PID
    const P: f32 = 10.0; // PID P-value.
    const I: f32 = 0.0; // PID P-value.

    // ========================= I2C Setup ==========================
    let scl = gpiob.pb8.into_alternate().set_open_drain();
    let sda = gpiob.pb9.into_alternate().set_open_drain();
    let mut i2c = I2c::new(dp.I2C1, (scl, sda), 100.kHz(), &clocks);
    
    // ========================= ADC Setup ==========================
    let potmeter = gpioa.pa0.into_analog();
    let mut adc = Adc::adc1(dp.ADC1, true, AdcConfig::default());

    // =================== DC Motor Driver Setup ====================
    let (_, (IN1_pwm, IN2_pwm, ..)) = dp.TIM1.pwm_hz(2000.Hz(), &clocks);
    let mut IN1_pwm = IN1_pwm.with(gpioa.pa8);
    let mut IN2_pwm = IN2_pwm.with(gpioa.pa9);
    IN1_pwm.disable();
    IN2_pwm.disable();


    // ========================== Main Loop ==========================
    loop {
        // Read Motor Position
        let mut buf = [0u8; 2];
        if i2c.write_read(AS5600_ADDR, &[AS5600_RAW_ANGLE_REG], &mut buf).is_ok() {
            ang_rotor = ((buf[0] as u16) << 8) | (buf[1] as u16);
        }
        else {
            warn!("I2C read failed");
        }

        // Read Potentiometer position
        let set_point: u16 = adc.convert(&potmeter, SampleTime::Cycles_480);


        // Filter position
        set_point_filtered = a1 * (set_point as f32) + (1.0 - a1) * set_point_filtered;
        rotor_ang_filtered = a2 * (ang_rotor as f32) + (1.0 - a2) * rotor_ang_filtered;


        // Calculate error and add P-factor
        err = set_point_filtered - rotor_ang_filtered;
        integral += err * dt;
        set = err*P + integral*I;

        // Apply duty cycle to motor driver
        duty = set.abs() as u16; // Calculate absolute value and convert to unsigned int.
        if duty < 5 { // Deadzone
            duty = 0;
        } else {
            IN1_pwm.set_duty(duty);  
            IN2_pwm.set_duty(duty);  
        }

        // Set motor direction
        if set >= 0.0 { // Forward -->
            IN2_pwm.disable();
            IN1_pwm.enable();

        } else {        // Backwards <--
            IN1_pwm.disable();
            IN2_pwm.enable();  
        }
        
        cortex_m::asm::delay(100 * ms);
        info!("Pot = {}", set_point_filtered);
        info!("Rotor = {}", rotor_ang_filtered);
        info!("Error = {}", set);
    }
}