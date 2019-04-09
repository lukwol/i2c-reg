//! Library for effortless creation of I2C registers with different sizes
//! and typesafe conversion between raw bytes and actual values.

#![no_std]
#![deny(warnings)]

use embedded_hal as hal;

mod i2c_interface;
mod registers;

pub use i2c_interface::I2cInterface;
pub use i2c_reg_derive;
pub use registers::{I2cReadRegister, I2cWriteRegister, Register};
