#![no_main]
#![no_std]

core::arch::global_asm!(include_str!("start.S"), options(att_syntax));

extern crate alloc;

#[macro_use]
mod macros;

mod debugcon;
mod heap;
mod pci;

use core::panic::PanicInfo;
use log::{debug, error, info};

/// Entry into the Rust code.
#[unsafe(no_mangle)]
extern "C" fn rust_entry(multiboot_magic: u32, multiboot_hdr: u32) -> ! {
    main(multiboot_magic, multiboot_hdr).expect("Should run kernel");
    unreachable!();
}

/// Exits QEMU via the shutdown device on the i440fx board.
fn exit_qemu() -> ! {
    unsafe { x86::io::outw(0x604, 0x2000) };
    loop {
        // It may take a few cycles until the machine powers off.
        core::hint::spin_loop();
    }
}

/// Executes the kernel's main logic.
fn main(multiboot_magic: u32, multiboot_hdr: u32) -> anyhow::Result<()> {
    init_environment()?;
    debug!("multiboot_hdr=0x{multiboot_hdr:x?}, multiboot_magic=0x{multiboot_magic:x?}");

    println!(
        "println works too, wow! {ansi_begin}Even with colors!{ansi_end}",
        // ANSI: red + bold
        ansi_begin = "\u{1b}[31;1m",
        ansi_end = "\x1b[0m"
    );

    let devices = pci::discover();
    println!("PCI devices:");
    for dev in devices {
        println!("  {}", dev);
    }

    exit_qemu();
}

/// Initializes the environment.
fn init_environment() -> anyhow::Result<()> {
    debugcon::DebugconLogger::init();
    info!("Logger initialized!");
    Ok(())
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    error!("PANIC! {}", info);
    exit_qemu();
}
