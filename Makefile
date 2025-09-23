.PHONY: default
default: build

##########################
# BIN TARGETS

.PHONY: build
build:
	$(MAKE) -C 01-just-assembly build
	$(MAKE) -C 02-rust-32-bit build

.PHONY: clean
clean:
	$(MAKE) -C 01-just-assembly clean
	$(MAKE) -C 02-rust-32-bit clean



