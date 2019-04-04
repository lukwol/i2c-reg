#![no_std]
#![deny(warnings)]

use embedded_hal as hal;

mod i2c_interface;
mod registers;

pub use i2c_interface::I2cInterface;
pub use registers::{I2cReadRegister, I2cWriteRegister, Register};
