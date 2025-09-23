

.PHONY: default
default: build


.PHONY: build
build:
	gcc -m32 -nostdlib -nostartfiles -ffreestanding -static -T link.ld -o kernel kernel.S


.PHONY: run
run: | build
	@# QEMU/TCG allows better debuggability over QEMU/KVM
	qemu-system-i386 \
		-debugcon stdio \
		-display none \
		-kernel ./kernel \
		-m 64M \
		-machine q35,accel=tcg \
		-monitor none \
		-no-reboot \
		-nodefaults


.PHONY: clean
clean:
	rm -f ./kernel



