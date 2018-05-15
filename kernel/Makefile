arch ?= x86_64
target_arch ?= $(arch)-snowflake

LD = ld
AR = ar
STRIP = strip
OBJDUMP = objdump
OBJCOPY = objcopy
CARGO = xargo

BUILD_DIR = ../build/kernel

linker_script = arch/$(arch)/linker.ld
kernel = $(BUILD_DIR)/kernel.bin
kernel_obj = $(BUILD_DIR)/target/$(target_arch)/release/libSnowKernel.a

.PHONY: all

all: $(kernel)

$(kernel): $(kernel_obj)
	@mkdir -p $(shell dirname $@)
	@$(LD) -n --gc-sections -T $(linker_script) $< -o $@ -z max-page-size=0x1000
	@$(OBJDUMP) -d $@ > $@.dsm
	@$(OBJCOPY) --only-keep-debug $@ $@.sym
$(kernel_obj):
	@mkdir -p $(shell dirname $@)
	@RUST_TARGET_PATH=$(abspath .) CARGO_TARGET_DIR=$(BUILD_DIR)/target CFLAGS=-ffreestanding $(CARGO) rustc --target $(target_arch) --release -- -C soft-float -C debuginfo=2 --emit link=$@
