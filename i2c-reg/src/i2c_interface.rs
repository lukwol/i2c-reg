use crate::hal::blocking::i2c;
use crate::registers::{I2cReadRegister, I2cWriteRegister};

/// I2C interface
///
/// # Usage
///
/// ```
/// # use embedded_hal::blocking::i2c;
/// # use i2c_reg::*;
/// # use i2c_reg_derive::*;
/// #
/// # struct MockI2c;
/// #
/// # impl i2c::WriteRead for MockI2c {
/// #     type Error = ();
/// #     fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
/// #         Ok(())
/// #     }
/// # }
/// #
/// # impl i2c::Write for MockI2c {
/// #     type Error = ();
/// #
/// #     fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
/// #         Ok(())
/// #     }
/// # }
/// # struct Value;
/// #
/// # impl Into<[u8; 2]> for Value {
/// #     fn into(self) -> [u8; 2] {
/// #         [0; 2]
/// #     }
/// # }
/// # impl From<[u8; 6]> for Value {
/// #     fn from(_: [u8; 6]) -> Self {
/// #         Value
/// #     }
/// # }
/// #
/// # let i2c = MockI2c;
/// # let value = Value;
/// #
/// #[derive(Register, I2cReadRegister)]
/// #[addr = 0b1101]
/// #[len = 6]
/// struct ReadOnlyRegister;
///
/// #[derive(Register, I2cReadRegister, I2cWriteRegister)]
/// #[addr = 0b1110]
/// #[len = 2]
/// struct ReadWriteRegister;
///
/// let mut interface = I2cInterface { i2c, address: 0b0110 };
/// let read_result: Value = interface.read_register(&ReadOnlyRegister).unwrap();
/// let write_result = interface.write_register(&ReadWriteRegister, value).unwrap();
/// ```
#[derive(Debug)]
pub struct I2cInterface<I2C> {

    /// SDA and SCL pins
    pub i2c: I2C,

    /// Slave device address
    pub address: u8,
}

impl<I2C> I2cInterface<I2C> {

    /// Read bytes from register and map output to `Value`
    pub fn read_register<Raw, Value, Err>(
        &mut self,
        register: &impl I2cReadRegister<Raw>,
    ) -> Result<Value, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
        Raw: Into<Value>,
    {
        register
            .i2c_read(&mut self.i2c, self.address)
            .map(|v| v.into())
    }

    /// Map `value` to bytes and write to register
    pub fn write_register<Raw, Err>(
        &mut self,
        register: &impl I2cWriteRegister<Raw>,
        value: impl Into<Raw>,
    ) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
    {
        register.i2c_write(&mut self.i2c, self.address, value.into())
    }
}
