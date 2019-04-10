# i2c-reg

## Features

Effortless creation of I2C read only/write only/read write registers with various sizes.

```rust
#[derive(Register, I2cReadRegister, I2cWriteRegister)]
#[address = 0b1110]
#[size = 2]
struct TemperatureRegister;
```

Typesafe conversion between raw bytes and actual values when reading from/writing to registers via interface.

```rust
type Raw = <TemperatureRegister as Register>::Raw;

struct Temperature(u16);

impl Into<Raw> for Temperature {
    fn into(self) -> Raw {
        [(self.0 >> 8) as u8, self.0 as u8]
    }
}

impl From<Raw> for Temperature {
    fn from(raw: Raw) -> Self {
        Temperature(((raw[0] as u16) << 8) + raw[1] as u16)
    }
}

let mut interface = I2cInterface { i2c, address: 0b0110 };
interface.write_register(TemperatureRegister, Temperature(42)).unwrap();
let temperature: Temperature = interface.read_register(TemperatureRegister).unwrap();
assert_eq!(Temperature(42), temperature);
```
