arch ?= x86_64
target_arch ?= $(arch)-snowflake

LD = $(arch)-elf-ld
AS = $(arch)-elf-as
AR = $(arch)-elf-ar
STRIP = $(arch)-elf-strip
OBJDUMP = $(arch)-elf-objdump
OBJCOPY = $(arch)-elf-objcopy
CARGO = xargo

BUILD_DIR = ../build/kernel

linker_script = ../arch/$(arch)/linker.ld
kernel = $(BUILD_DIR)/kernel.bin
kernel_obj = $(BUILD_DIR)/target/$(target_arch)/release/libSnowKernel.a

.PHONY: all

all: $(kernel)

$(kernel): $(kernel_obj)
	@mkdir -p $(shell dirname $@)
	@$(LD) -n --gc-sections -T $(linker_script) -o $@ $(BUILD_DIR)/start.o $< -z max-page-size=0x1000
	@$(OBJDUMP) -d $@ > $@.dsm
	@$(STRIP) -g $@

$(kernel_obj):
	@mkdir -p $(shell dirname $@)
	@make -C ../arch/x86_64/init
	@RUST_TARGET_PATH=$(abspath .) CARGO_TARGET_DIR=$(BUILD_DIR)/target $(CARGO) build --target $(target_arch) --release
