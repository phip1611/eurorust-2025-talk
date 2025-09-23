use core::fmt::{Display, Formatter};
use pci_types::HeaderType;

/// Common PCI device information obtained from the configuration space. Not ABI
/// compatible with the config space and enriched with additional info.
#[derive(Debug)]
#[allow(unused)]
pub struct CommonConfigSpaceInfo {
    pub vendor_id: u16,
    // Vendor name, if the vendor is known.
    pub vendor_name: Option<&'static str>,
    pub device_id: u16,
    // Device name, if the device is known.
    pub device_name: Option<&'static str>,
    pub command_register: u16,
    pub status_register: u16,
    pub revision_id: u8,
    // 24 bit long
    pub class_code: u32,
    pub cache_line_size: u8,
    pub latency_timer: u8,
    pub header_type: HeaderType,
    pub bist: u8,
    pub address_registers: [u32; 6],
}

impl Display for CommonConfigSpaceInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}(0x{:x})/{}(0x{:x})",
            self.vendor_name.unwrap_or("<unknown>"),
            self.vendor_id,
            self.device_name.unwrap_or("<unknown>"),
            self.device_id
        )
    }
}
