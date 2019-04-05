use crate::hal::blocking::i2c;

pub trait Register {
    type Raw;

    fn address(&self) -> u8;
    fn length(&self) -> usize;
}

pub trait I2cReadRegister<Raw>: Register {
    fn i2c_read<I2C, Err>(&self, i2c: &mut I2C, device_address: u8) -> Result<Raw, Err>
    where
        I2C: i2c::WriteRead<Error = Err>;
}

pub trait I2cWriteRegister<Raw>: Register {
    fn i2c_write<I2C, Err>(&self, i2c: &mut I2C, device_address: u8, raw: Raw) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>;
}
