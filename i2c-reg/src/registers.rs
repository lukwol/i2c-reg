use crate::address::Address;
use crate::hal::blocking::i2c;

pub trait Register {
    fn address(&self) -> Address;
}

pub trait I2cReadRegister<'a, Raw>: Register {
    fn i2c_read<I2C, Err>(&self) -> &Fn(&mut I2C, Address, Address) -> Result<Raw, Err>
    where
        I2C: i2c::WriteRead<Error = Err>;
}

pub trait I2cWriteRegister<'a, Raw>: Register {
    fn i2c_write<I2C, Err>(&self) -> &Fn(&mut I2C, Address, Address, Raw) -> Result<(), Err>
    where
        I2C: i2c::Write<Error = Err>;
}

#[macro_export]
macro_rules! reg {
    ($name: ident, $addr: expr) => {
        #[derive(Debug)]
        pub(crate) struct $name;

        impl Register for $name {
            fn address(&self) -> Address {
                Address($addr)
            }
        }
    };
}

#[macro_export]
macro_rules! i2c_read {
    ($name: ident, $len: tt) => {
        impl<'a> I2cReadRegister<'a, [u8; $len]> for $name {
            fn i2c_read<I2C, Err>(
                &self,
            ) -> &Fn(&mut I2C, Address, Address) -> Result<[u8; $len], Err>
            where
                I2C: i2c::WriteRead<Error = Err>,
            {
                &|i2c, device_address, reg_address| {
                    let mut buff = [0; $len];
                    i2c.write_read(device_address.into(), &[reg_address.into()], &mut buff)?;
                    Ok(buff)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! i2c_write {
    ($name: ident, $len: tt) => {
        impl<'a> I2cWriteRegister<'a, [u8; $len]> for $name {
            fn i2c_write<I2C, Err>(
                &self,
            ) -> &Fn(&mut I2C, Address, Address, [u8; $len]) -> Result<(), Err>
            where
                I2C: i2c::Write<Error = Err>,
            {
                &|i2c, device_address, reg_address, value| {
                    let mut payload = [0; $len + 1];
                    payload[0] = reg_address.into();
                    for (i, item) in value.iter().enumerate() {
                        payload[i + 1] = *item;
                    }
                    i2c.write(device_address.into(), &payload)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! i2c_rw_reg {
    ($name: ident, addr: $addr: expr, len: $len: tt) => {
        reg!($name, $addr);
        i2c_read!($name, $len);
        i2c_write!($name, $len);
    };
}

#[macro_export]
macro_rules! i2c_ro_reg {
    ($name: ident, addr: $addr: expr, len: $len: tt) => {
        reg!($name, $addr);
        i2c_read!($name, $len);
    };
}

#[macro_export]
macro_rules! i2c_wo_reg {
    ($name: ident, addr: $addr: expr, len: $len: tt) => {
        reg!($name, $addr);
        i2c_write!($name, $len);
    };
}
