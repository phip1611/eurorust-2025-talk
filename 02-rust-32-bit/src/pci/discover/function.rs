use super::address::Address;
use super::config_space::CommonConfigSpaceInfo;
use crate::pci::io::io_read_config_space;
use core::fmt::{Display, Formatter};
use pci_ids::FromId;
use pci_types::HeaderType;

/// A single function belonging to a device.
#[derive(Clone, Copy, Debug)]
pub struct Function {
    address: Address,
}

impl Function {
    /// Constructor returning a [`Function`] at the given [`Address`] by
    /// discovering it on the PCI bus.
    pub fn discover(address: Address) -> Option<Self> {
        let config_space_buf = unsafe { io_read_config_space(address, 0) };

        // PCI returns just ones, if something is undefined.
        if config_space_buf == 0xffff_ffff {
            return None;
        }

        Some(Self { address })
    }

    /// Queries the configuration space of the function via I/O.
    pub fn io_info(&self) -> CommonConfigSpaceInfo {
        let mut config_space_buf = [0_32; 10];

        for i in 0..10 {
            config_space_buf[i] = unsafe {
                // * 4: read only even 32-bit registers
                io_read_config_space(self.address, i as u8 * 4)
            };
        }

        // PCI returns just ones, if something is undefined.
        if config_space_buf[0] == 0xffff_ffff {
            panic!("Device only returns 0xff..?!");
        }

        let vendor_id = config_space_buf[0] as u16;
        let device_id = (config_space_buf[0] >> 16) as u16;
        let vendor_name = pci_ids::Vendor::from_id(vendor_id).map(|x| x.name());
        let device_name = pci_ids::Device::from_vid_pid(vendor_id, device_id).map(|x| x.name());

        let command_register = config_space_buf[1] as u16;
        let status_register = (config_space_buf[1] >> 16) as u16;
        let revision_id = config_space_buf[2] as u8;
        let class_code = config_space_buf[2] >> 8 as u32;
        let cache_line_size = config_space_buf[3] as u8;
        let latency_timer = (config_space_buf[3] >> 8) as u8;
        let header_type = match config_space_buf[3] >> 16 as u8 {
            0x00 => HeaderType::Endpoint,
            0x01 => HeaderType::PciPciBridge,
            0x02 => HeaderType::CardBusBridge,
            t => HeaderType::Unknown(t as u8),
        };
        let bist = (config_space_buf[3] >> 24) as u8;

        let address_registers = [
            config_space_buf[4],
            config_space_buf[5],
            config_space_buf[6],
            config_space_buf[7],
            config_space_buf[8],
            config_space_buf[9],
        ];

        CommonConfigSpaceInfo {
            vendor_id,
            vendor_name,
            device_id,
            device_name,
            command_register,
            status_register,
            revision_id,
            class_code,
            cache_line_size,
            latency_timer,
            header_type,
            bist,
            address_registers,
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {}", self.address, self.io_info())?;
        Ok(())
    }
}
