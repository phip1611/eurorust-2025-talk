use crate::pci::discover::Device;
use heapless::Vec;

pub(self) mod discover;
mod io;

/// Discovers all available [`Device`]s via the I/O port-based PCI config space
/// interface. See [`io`].
pub fn discover() -> Vec<Device, 32> {
    discover::discover()
}
