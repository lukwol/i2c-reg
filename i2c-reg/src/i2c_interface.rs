use crate::hal::blocking::i2c;
use crate::registers::{I2cReadRegister, I2cWriteRegister};

#[derive(Debug)]
pub struct I2cInterface<I2C> {
    pub i2c: I2C,
    pub address: u8,
}

impl<I2C> I2cInterface<I2C> {
    pub fn read_register<'a, Raw, Value, Err>(
        &mut self,
        register: &impl I2cReadRegister<'a, Raw>,
    ) -> Result<Value, Err>
    where
        I2C: i2c::WriteRead<Error = Err>,
        Raw: Into<Value>,
    {
        register.i2c_read()(&mut self.i2c, self.address).map(|v| v.into())
    }

    pub fn write_register<'a, Raw, Err>(
        &mut self,
        register: &impl I2cWriteRegister<'a, Raw>,
        value: impl Into<Raw>,
    ) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>,
    {
        register.i2c_write()(&mut self.i2c, self.address, value.into())
    }
}
