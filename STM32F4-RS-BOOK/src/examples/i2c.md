# Using I2C
Plain and simple, I2C is useful. This is not a guide on how I2C works, just show to read and write messages. 

To use i2C, we need to import the module `I2c`:
```rust
use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
    i2c::I2c,
};
```

Configuring up I2C is quite simple. We simply need to find two valid i2C pins, like `PB9` and `PB8`, which is connected to the peripheral `I2C1`. We configure this pins into alternate mode, with a open drain, letting our i2C peripheral set their mode:
```rust
let scl = gpiob.pb8.into_alternate().set_open_drain();
    let sda = gpiob.pb9.into_alternate().set_open_drain();
    let mut i2c = I2c::new(dp.I2C1, (scl, sda), 100.kHz(), &clocks);
```

To read from a sensor, we can use `.write_read`, which sends a message to the address specified (`AS5600_ADDR`), and reads the register `AS5600_RAW_ANGLE_REG`.
```rust
i2c.write_read(AS5600_ADDR, &[AS5600_RAW_ANGLE_REG], &mut buf).is_ok()
```




## Complete Example
Here is a complete code example, it is the default example, and can be run with:
```sh
$ cargo embed --example i2c_as5600
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
    i2c::I2c,
};


#[allow(non_snake_case)]
#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    // ========================== Set-up ==========================
    // Take ownership of device peripherals and split out GPIO group A and B
    let dp = pac::Peripherals::take().unwrap();
    let gpiob = dp.GPIOB.split();

    // Configure clocks
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

   // ========================== Constants ==========================
    const AS5600_ADDR: u8 = 0x36;       // 7-bit I2C address
    const AS5600_RAW_ANGLE_REG: u8 = 0x0C; // MSB of raw angle
    let ms: u32 = 8_000; // clock cycles to millisecond conversion.
    let raw2deg: f32 = 360.0 / 4096.0;
    let mut ang_rotor_deg: f32 = 0.0;
    let mut ang_rotor_raw: u16 = 0;

    // ========================= I2C Setup ==========================
    let scl = gpiob.pb8.into_alternate().set_open_drain();
    let sda = gpiob.pb9.into_alternate().set_open_drain();
    let mut i2c = I2c::new(dp.I2C1, (scl, sda), 100.kHz(), &clocks);


    // ========================== Main Loop ==========================
    loop {
        // Read Motor Position
        let mut buf = [0u8; 2];
        if i2c.write_read(AS5600_ADDR, &[AS5600_RAW_ANGLE_REG], &mut buf).is_ok() {
            ang_rotor_raw = ((buf[0] as u16) << 8) | (buf[1] as u16);
        }
        else {
            warn!("I2C read failed");
        }

        // Convert to degrees
        ang_rotor_deg = (ang_rotor_raw as f32) * raw2deg;
        
        // Send Position over defmt
        info!("Rotor position = {}", ang_rotor_deg);

        // Wait 500ms
        cortex_m::asm::delay(200 * ms);
    }
}
```