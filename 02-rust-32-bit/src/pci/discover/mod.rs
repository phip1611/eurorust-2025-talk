//! Anything required for discovering PCI devices from the PCI bus.

use heapless::Vec;

mod address;
mod config_space;
mod device;
mod function;

pub use address::*;
pub use device::*;

/// Discovers all available [`Device`]s via the I/O port-based PCI config space
/// interface.
pub fn discover() -> Vec<Device, 32> {
    let mut devices = Vec::new();
    for bus in 0..=255 {
        for slot in 0..32 {
            let pci_addr = Address {
                bus,
                slot,
                function: 0,
            };
            let dev = Device::discover(pci_addr);
            if let Some(dev) = dev {
                devices.push(dev).unwrap();
            }
        }
    }
    devices
}
