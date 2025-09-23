use super::address::Address;
use super::function::Function;
use core::fmt::{Display, Formatter};
use heapless::Vec;

/// A single device referenced by a bus id and the slot id.
#[derive(Debug)]
pub struct Device {
    /// All device functions. The Vector has always at least one element, as
    /// the main function is at index 0.
    functions: Vec<Function, 8>,
}

impl Device {
    /// Discovers if a PCI device exists on the [`Address`] by reading
    /// the configuration space.
    pub fn discover(address: Address) -> Option<Self> {
        let main_function = Function::discover(address)?;

        let mut additional_functions = Vec::<_, 7>::new();
        for function in 1..8 {
            let address = Address {
                bus: address.bus,
                slot: address.slot,
                function,
            };
            let function = Function::discover(address);
            if let Some(function) = function {
                additional_functions.push(function).unwrap();
            }
        }

        let mut functions = Vec::new();
        functions.push(main_function).unwrap();
        functions.extend(additional_functions);

        Some(Self { functions })
    }

    /// Returns the main [`Function`] of the device.
    pub fn main_function(&self) -> &Function {
        &self.functions[0]
    }

    /// Returns the additional [`Function`] of the device (if any).
    pub fn additional_functions(&self) -> &[Function] {
        &self.functions[1..]
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.main_function())?;

        if !self.additional_functions().is_empty() {
            writeln!(f, "")?;
        }

        for (i, function) in self.additional_functions().iter().enumerate() {
            write!(f, "  {}", function)?;
            if i < self.additional_functions().len() - 1 {
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}
