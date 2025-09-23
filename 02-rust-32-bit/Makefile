.PHONY: default
default: build


.PHONY: build
build:
	cargo build --release \
		--target ./x86-unknown-none.json \
		-Z build-std=core,alloc,compiler_builtins \
		-Z build-std-features=compiler-builtins-mem


.PHONY: run
run: | build
	@# QEMU/TCG allows better debuggability over QEMU/KVM
	qemu-system-i386 \
		-debugcon stdio \
		-display none \
		-kernel target/x86-unknown-none/release/kernel \
		-m 64M \
		-machine q35,accel=tcg \
		-monitor none \
		-no-reboot \
		-nodefaults


.PHONY: clean
clean:
	cargo clean
