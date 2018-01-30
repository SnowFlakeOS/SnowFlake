arch ?= x86_64
target_arch ?= $(arch)-snowflake

LD = $(arch)-efi-pe-ld
AS = $(arch)-efi-pe-as
AR = $(arch)-efi-pe-ar
STRIP = $(arch)-efi-pe-strip
OBJDUMP = $(arch)-efi-pe-objdump
CARGO = xargo

BUILD_DIR = ../build/kernel

linker_script = ../arch/$(arch)/linker.ld
kernel = $(BUILD_DIR)/kernel.bin
kernel_obj = $(BUILD_DIR)/target/$(target_arch)/release/libSnowKernel.a

.PHONY: all

all: $(kernel)

$(kernel): $(kernel_obj)
	@mkdir -p $(shell dirname $@)
	@$(LD) \
		-T $(linker_script) \
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
	@$(OBJDUMP) -d $@ > $@.dsm
	@$(STRIP) $@

$(kernel_obj):
	@mkdir -p $(shell dirname $@)
	@RUST_TARGET_PATH=$(abspath .) CARGO_TARGET_DIR=$(BUILD_DIR)/target $(CARGO) build --target $(target_arch) --release
