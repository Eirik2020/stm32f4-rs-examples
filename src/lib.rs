// Compiler directive
#![deny(unsafe_code)]
#![no_std]


// Imports
use embedded_hal::i2c::I2c;

// Driver struct
pub struct As5600<I2C> {
    i2c: I2C,
    address: u8,
}

// Driver implementation
impl<I2C, E> As5600<I2C>
where
    I2C: I2c<Error = E>,
{
    // Registers
    const DEFAULT_ADDR: u8 = 0x36;
    const ANGLE_REG: u8 = 0x0E; // 12-bit raw angle (2 bytes)

    // Constructor
    pub fn new(i2c: I2C) -> Self {
        Self {
            i2c,
            address: Self::DEFAULT_ADDR,
        }
    }

    /// Reads raw 12-bit angle (0..=4095)
    pub fn read_raw_angle(&mut self) -> Result<u16, E> {
        let mut buf = [0u8; 2];
        self.i2c.write_read(self.address, &[Self::ANGLE_REG], &mut buf)?;
        let angle = ((buf[0] as u16) << 8 | buf[1] as u16) & 0x0FFF;
        Ok(angle)
    }

    /// Converts raw angle to degrees (0.0 - 360.0)
    pub fn read_degrees(&mut self) -> Result<f32, E> {
        let raw = self.read_raw_angle()?;
        Ok((raw as f32) * 360.0 / 4096.0)
    }

    // Release peripheral
    pub fn release(self) -> I2C {
        self.i2c
    }
}