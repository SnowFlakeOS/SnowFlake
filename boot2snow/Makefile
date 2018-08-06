arch ?= x86_64
target_arch ?= $(arch)-boot2snow

BUILD_DIR = ../build/boot2snow
PREFIX = $(shell readlink -f ..//prefix)

LD = $(PREFIX)/bin/$(arch)-efi-pe-ld

CARGO = xargo
CARGOFLAGS = --target $(target_arch) --release -- -C soft-float

BINUTILS = 2.31

boot2snow = $(BUILD_DIR)/bootx64.efi
boot2snow_obj = $(BUILD_DIR)/target/$(target_arch)/release/libBoot2Snow.a

objs = $(boot2snow_obj).o

.PHONY: all

all: $(boot2snow)

$(boot2snow): $(objs) $(LD)
	@mkdir -p $(shell dirname $@)
	@$(LD) \
		--oformat pei-x86-64 \
		--dll \
		--image-base 0 \
		--section-alignment 32 \
		--file-alignment 32 \
		--major-os-version 0 \
		--minor-os-version 0 \
		--major-image-version 0 \
		--minor-image-version 0 \
		--major-subsystem-version 0 \
		--minor-subsystem-version 0 \
		--subsystem 10 \
		--heap 0,0 \
		--stack 0,0 \
		--pic-executable \
		--entry _start \
		--no-insert-timestamp \
		$< -o $@
 
$(boot2snow_obj).o: $(boot2snow_obj)
	@cd $(shell dirname $<) && $(AR) x libBoot2Snow.a
	@ld -r $(shell dirname $<)/*.o -o $@

$(boot2snow_obj):
	@mkdir -p $(shell dirname $@)
	@RUST_TARGET_PATH=$(abspath .) CARGO_TARGET_DIR=$(BUILD_DIR)/target $(CARGO) rustc --lib $(CARGOFLAGS) -C lto --emit link=$@

$(PREFIX)/binutils-$(BINUTILS).tar.gz:
	@mkdir -p $(shell dirname $@)
	@curl -o $@ https://mirror.jre655.com/GNU/binutils/binutils-$(BINUTILS).tar.gz

$(LD): $(PREFIX)/binutils-$(BINUTILS).tar.gz
	@tar -zxvf $< -C $(PREFIX)
	@mkdir -p $(PREFIX)/build
	@cd $(PREFIX)/build && \
	../binutils-$(BINUTILS)/configure --target="$(arch)-efi-pe" --disable-werror --prefix="$(PREFIX)" && \
	make all-ld -j `nproc` && \
	make install-ld -j `nproc`
