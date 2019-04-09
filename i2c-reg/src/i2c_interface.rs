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
/// # static mut REGISTER_CACHE: [u8; 2] = [0, 0];
/// #
/// # struct MockI2c;
/// #
/// # impl i2c::WriteRead for MockI2c {
/// #     type Error = ();
/// #     fn write_read(&mut self, address: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
/// #         for (i, item) in unsafe { REGISTER_CACHE }.iter().enumerate() {
/// #             buffer[i] = *item;
/// #         }
/// #         Ok(())
/// #     }
/// # }
/// #
/// # impl i2c::Write for MockI2c {
/// #     type Error = ();
/// #
/// #     fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
/// #         for (i, item) in bytes.iter().skip(1).enumerate() {
/// #             unsafe { REGISTER_CACHE [i] = *item; }
/// #         }
/// #         Ok(())
/// #     }
/// # }
/// #[derive(Register, I2cReadRegister, I2cWriteRegister)]
/// #[addr = 0b1110]
/// #[len = 2]
/// struct TemperatureRegister;
///
/// type Raw = <TemperatureRegister as Register>::Raw;
///
/// #[derive(Debug, PartialEq)]
/// struct Temperature(u16);
///
/// impl Into<Raw> for Temperature {
///     fn into(self) -> Raw {
///         [(self.0 >> 8) as u8, self.0 as u8]
///     }
/// }
///
/// impl From<Raw> for Temperature {
///     fn from(raw: Raw) -> Self {
///         Temperature(((raw[0] as u16) << 8) + raw[1] as u16)
///     }
/// }
///
/// # let i2c = MockI2c;
/// # let value = Temperature;
/// #
/// let mut interface = I2cInterface { i2c, address: 0b0110 };
/// interface.write_register(TemperatureRegister, Temperature(42)).unwrap();
/// let temperature: Temperature = interface.read_register(TemperatureRegister).unwrap();
/// assert_eq!(Temperature(42), temperature);
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
        register: impl I2cReadRegister<Raw>,
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
        register: impl I2cWriteRegister<Raw>,
        value: impl Into<Raw>,
    ) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
    {
        register.i2c_write(&mut self.i2c, self.address, value.into())
    }
}
