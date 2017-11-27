arch ?= x86_64
target ?= $(arch)-snowflake
boot := build/boot-$(arch).bin
loader := build/loader-$(arch).elf
kernel := build/kernel-$(arch).bin
kernel_elf := build/kernel-$(arch).elf
img := build/os-$(arch).img
iso := build/os-$(arch).iso

rust_os := target/$(target)/debug/libsnowflake.a

linker_script := src/arch/$(arch)/linker.ld

boot_source_file := src/arch/$(arch)/boot.asm
loader_source_file := src/arch/$(arch)/loader.asm

libc_source_files := $(shell find src/libc -name "*.c")
libc_object_files := $(patsubst src/libc/%.c, \
    build/libc/%.o, $(libc_source_files))

CARGO = cargo

CC = clang
CFLAGS = -target x86_64-pc-linux-gnu -ffreestanding -mcmodel=large -mno-red-zone -mno-mmx -mno-sse -mno-sse2 -nostdlib -Isrc/include/libc

NASM = nasm

LD = $(arch)-elf-ld
AS = $(arch)-elf-as
OBJCOPY = $(arch)-elf-objcopy

.PHONY: all clean run img

all: $(boot) $(kernel)

clean:
	@rm -r build target

run: $(img)
	@qemu-system-x86_64 $(img)

img: $(img)

iso: $(iso)

$(img): $(boot) $(kernel)
	@mkdir -p $(shell dirname $@)
	@dd if=/dev/zero of=$(img) bs=512 count=2880
	@dd if=$(boot) of=$(img) conv=notrunc
	@dd if=$(kernel) of=$(img) conv=notrunc bs=512 seek=1

$(iso): $(img)
	@mkdir -p build/cdcontents
	@cp $(img) build/cdcontents
	@mkisofs -o $(iso) -V SnowWhiteOS -b $(img) build/cdcontents

$(boot):
	@mkdir -p $(shell dirname $@)
	@$(NASM) -f bin $(boot_source_file) -o $(boot)

$(loader):
	@mkdir -p $(shell dirname $@)
	@$(NASM) -f elf64 $(loader_source_file) -o $(loader)

$(kernel): $(loader) cargo $(rust_os) $(libc_object_files) $(linker_script)
	@$(LD) -n --gc-sections -T $(linker_script) -o $(kernel_elf) $(loader) $(libc_object_files) $(rust_os)
	@$(OBJCOPY) $(kernel_elf) -O binary $(kernel) # debug

# compile kernel files
cargo:
	@xargo build --target $(target)

# compile libc files
build/libc/%.o: src/libc/%.c
	@mkdir -p $(shell dirname $@)
	@$(CC) $(CFLAGS) -c $< -o $@
