# GNU Assembler Syntax (GAS) with AT&T Assembly Flavor

.code32

.section .multiboot_header, "a", @progbits

/*
 * Multiboot v1 Header.
 * Required so that we can be booted by QEMU via the "-kernel" parameter.
 */
.align 8
.global multiboot_header
multiboot_header:
    .long   0x1badb002
    .long   0x0
    .long  -0x1badb002

.section .text

# When we start here via Multiboot 1, the machine state is 32-bit protected
# mode without paging.
.global start
start:
    # QEMU debugcon: port 0xe9
    mov  $0xe9, %dx

    mov  $'h', %al
    out  %al, %dx

    mov  $'i', %al
    out  %al, %dx

    mov  $'\n', %al
    out  %al,  %dx

    # QEMU shutdown device (i440fx board)
    mov  $0x604 , %dx
    mov  $0x2000, %ax
    out  %ax,     %dx

    ud2
