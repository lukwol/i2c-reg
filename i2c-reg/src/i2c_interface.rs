use crate::hal::blocking::i2c;
use crate::registers::{I2cReadRegister, I2cWriteRegister};

/// I2C interface
/// # Usage
///
/// ```
/// // TODO: write usage
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
