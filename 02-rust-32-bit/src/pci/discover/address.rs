use core::fmt::{Display, Formatter};

/// Address of a PCI device, also called **B**us **D**evice **F**unction (BDF).
#[derive(Copy, Clone, Debug)]
pub struct Address {
    pub bus: u8,
    pub slot: u8,
    pub function: u8,
}

impl Address {
    /// Encodes the address to PCI's 32-bit encoding.
    pub fn val(&self) -> u32 {
        ((self.bus as u32) << 16) | ((self.slot as u32) << 11) | ((self.function as u32) << 8)
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:03}:{:02}:{:01}", self.bus, self.slot, self.function)
    }
}

impl From<Address> for u32 {
    fn from(value: Address) -> Self {
        value.val()
    }
}
