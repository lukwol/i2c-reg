#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Address(pub u8);

impl From<Address> for u8 {
    fn from(address: Address) -> Self {
        address.0
    }
}
