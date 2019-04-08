use crate::hal::blocking::i2c;

/// Describes generic Register
pub trait Register {

    /// Raw type (bytes) of value read or written to register
    type Raw;

    /// Register address
    fn address(&self) -> u8;

    /// Lenght of bytes that can be read or written to register
    fn length(&self) -> usize;
}

/// Describes writable I2C register
pub trait I2cReadRegister<Raw>: Register {

    /// Read bytes from reagister on slave device with `device_address` via I2C
    fn i2c_read<I2C, Err>(&self, i2c: &mut I2C, device_address: u8) -> Result<Raw, Err>
    where
        I2C: i2c::WriteRead<Error = Err>;
}

/// Describes readable I2C register
pub trait I2cWriteRegister<Raw>: Register {

    /// Write bytes to reagister on slave device with `device_address` via I2C
    fn i2c_write<I2C, Err>(&self, i2c: &mut I2C, device_address: u8, raw: Raw) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>;
}
