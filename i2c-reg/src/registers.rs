//! Traits for describing I2C registers

use crate::hal::blocking::i2c;

// TODO: Add example and compare ImplementedRegister vs DerivedRegister, it should be the same
/// Describes generic Register
pub trait Register {
    /// Raw type (bytes) of value read or written to register
    type Raw;

    /// Register address
    fn address(&self) -> u8;

    /// Register number of bytes
    fn size(&self) -> usize;
}

/// Describes writable I2C register
pub trait I2cReadRegister<Raw>: Register {
    /// Read bytes from register on slave device with `device_address`
    fn i2c_read<I2C, Err>(&self, i2c: &mut I2C, device_address: u8) -> Result<Raw, Err>
    where
        I2C: i2c::WriteRead<Error = Err>;
}

/// Describes readable I2C register
pub trait I2cWriteRegister<Raw>: Register {
    /// Write bytes to register on slave device with `device_address`
    fn i2c_write<I2C, Err>(&self, i2c: &mut I2C, device_address: u8, raw: Raw) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>;
}
