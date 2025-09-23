use crate::pci::discover::Address;

const CONFIG_ADDRESS_IO_PORT: u16 = 0xcf8;
const CONFIG_DATA_IO_PORT: u16 = 0xcfc;

/// Reads 32-bit from the PCI config space via x86 I/O ports.
///
/// # Parameters
/// - `base-address`: [`Address`] of PCI device function
/// - `offset`: Offset into the config space. Reduced to the smallest multiple
///   of four smaller than the value:
///   - `0` -> `0`
///   - `1` -> `0`
///   - `4` -> `4`
///   - `7` -> `4`
///   - `8` -> `8`
///
/// # Safety
/// I/O ports must be accessible.
pub unsafe fn io_read_config_space(address: Address, offset: u8) -> u32 {
    // Everything else makes no sense here.
    assert_eq!(
        offset & !0xfc,
        0,
        "Offset(0x{:x}) must point to a whole 32-bit value.",
        offset
    );

    let address = address.val();

    // 0x8000_0000: set configuration space access bit
    let address = 0x8000_0000 | address | offset as u32;

    unsafe {
        x86::io::outl(CONFIG_ADDRESS_IO_PORT, address);
        x86::io::inl(CONFIG_DATA_IO_PORT)
    }
}
