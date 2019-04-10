//! Traits for describing I2C registers

use crate::hal::blocking::i2c;

/// Describes generic Register
///
/// # Example
/// ```
/// use i2c_reg::Register;
/// use i2c_reg_derive::Register;
///
/// #[derive(Register)]
/// #[address = 0b0111]
/// #[size = 6]
/// struct BasicRegister;
///
/// let _: <BasicRegister as Register>::Raw = [0; 6];
///
/// assert_eq!(0b0111, BasicRegister.address());
/// assert_eq!(6, BasicRegister.size());
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
///
/// # Example
/// ```
/// use embedded_hal::blocking::i2c;
/// use i2c_reg::{Register, I2cReadRegister, I2cInterface};
/// use i2c_reg_derive::{Register, I2cReadRegister};
///
/// #[derive(Register, I2cReadRegister)]
/// #[address = 0b1_0011]
/// #[size = 4]
/// struct BasicReadRegister;
///
/// # struct MockI2c;
/// #
/// # impl i2c::WriteRead for MockI2c {
/// #     type Error = ();
/// #     fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
/// #         Ok(())
/// #     }
/// # }
/// # let i2c = MockI2c;
/// #
/// let mut interface = I2cInterface { i2c, address: 0b0110 };
/// let _: <BasicReadRegister as Register>::Raw =
///     interface.read_register(BasicReadRegister).unwrap();
/// ```
pub trait I2cReadRegister<Raw>: Register {
    /// Read bytes from register on slave device with `device_address`
    fn i2c_read<I2C, Err>(&self, i2c: &mut I2C, device_address: u8) -> Result<Raw, Err>
    where
        I2C: i2c::WriteRead<Error = Err>;
}

/// Describes readable I2C register
///
/// # Example
/// ```
/// use embedded_hal::blocking::i2c;
/// use i2c_reg::{Register, I2cWriteRegister, I2cInterface};
/// use i2c_reg_derive::{Register, I2cWriteRegister};
///
/// #[derive(Register, I2cWriteRegister)]
/// #[address = 0b0011]
/// #[size = 1]
/// struct BasicWriteRegister;
///
/// # struct MockI2c;
/// #
/// # impl i2c::Write for MockI2c {
/// #     type Error = ();
/// #
/// #     fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
/// #         Ok(())
/// #     }
/// # }
/// # let i2c = MockI2c;
/// #
/// let mut interface = I2cInterface { i2c, address: 0b10_1010 };
/// interface.write_register(BasicWriteRegister, [42]).unwrap();
/// ```
pub trait I2cWriteRegister<Raw>: Register {
    /// Write bytes to register on slave device with `device_address`
    fn i2c_write<I2C, Err>(&self, i2c: &mut I2C, device_address: u8, raw: Raw) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>;
}
