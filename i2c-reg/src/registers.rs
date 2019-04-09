//! Traits for describing I2C registers

use crate::hal::blocking::i2c;

/// Describes generic Register
///
/// # Example
/// ```
/// use i2c_reg::Register;
/// use i2c_reg_derive::Register;
///
/// struct BasicRegister;
///
/// impl Register for BasicRegister {
///     type Raw = [u8; 6];
///
///     fn address(&self) -> u8 {
///         0b0111
///     }
///
///     fn size(&self) -> usize {
///         6
///     }
/// }
///
/// #[derive(Register)]
/// #[address = 0b0111]
/// #[size = 6]
/// struct DerivedBasicRegister;
///
/// let raw: <BasicRegister as Register>::Raw = [0; 6];
/// let derived_raw: <DerivedBasicRegister as Register>::Raw = [0; 6];
///
/// assert_eq!(BasicRegister.address(), DerivedBasicRegister.address());
/// assert_eq!(BasicRegister.size(), DerivedBasicRegister.size());
/// assert_eq!(raw, derived_raw);
/// ```
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
