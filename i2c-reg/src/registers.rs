use crate::hal::blocking::i2c;

pub trait Register {
    fn address(&self) -> u8;
    fn length(&self) -> usize;
}

pub trait I2cReadRegister<'a, Raw>: Register {
    fn i2c_read<I2C, Err>(&self) -> &Fn(&mut I2C, u8) -> Result<Raw, Err>
    where
        I2C: i2c::WriteRead<Error = Err>;
}

pub trait I2cWriteRegister<'a, Raw>: Register {
    fn i2c_write<I2C, Err>(&self) -> &Fn(&mut I2C, u8, Raw) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>;
}
