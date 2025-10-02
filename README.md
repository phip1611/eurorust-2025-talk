# A Minimal Rust Kernel: Printing to QEMU with core::fmt

This repository shows practical code examples for
[my EuroRust talk 2025](https://eurorust.eu/talks/a-minimal-rust-kernel/) in
Paris. The [Slides](https://eurorust-2025.slides.phip1611.dev/) are at another location.

The talk and the examples focuses on x86 space only.

## Structure

### `01-just-assembly`

This is a truly minimal binary that boots in QEMU and prints `hi` to the screen.
It demonstrates how to access (virtual) hardware.

**DEMO:** Type `make run` to start the demo in QEMU.

### `02-rust-32-bit`

This is a truly minimal Rust kernel binary that boots in QEMU and can print
various log messages as well as the PCI space of the VM to the terminal.

It consists of an assembly routine to set up the stack and to jump into the Rust
code. Note that this is a 32-bit kernel for x86 32-bit protected mode without
paging, i.e., no virtual memory.

**DEMO:** Type `make run` to start the demo in QEMU.

### `03-PhipsOS`

A more sophisticated project, but not included in this repository, is the
[kernel of PhipsOS](https://github.com/phip1611/phips-os/tree/6efe6e5aee6dd7203a65a1b6e1fff78ed49e4ad8).
Although typical kernel features are not yet implemented, technically, the
kernel and its EFI loader bring everything you expect from a "real" kernel:

- 64-bit long mode with kernel being in higher half of virtual address space
- kernel has proper LOAD segments with proper access rights
- 2 MiB huge-page mappings for the kernels LOAD segments
- and a project setup with `no_std` binaries and unit tests

A few selected highlights:
- [loading the kernel from disk](https://github.com/phip1611/phips-os/blob/6efe6e5aee6dd7203a65a1b6e1fff78ed49e4ad8/ws/bins/uefi-loader/src/main.rs#L63)
- [loading the kernel's LOAD segments into RAM](https://github.com/phip1611/phips-os/blob/main/ws/libs/loader-lib/src/lib.rs#L48)
- [jumping from the loader into the kernel](https://github.com/phip1611/phips-os/blob/6efe6e5aee6dd7203a65a1b6e1fff78ed49e4ad8/ws/bins/uefi-loader/src/main.rs#L92)
