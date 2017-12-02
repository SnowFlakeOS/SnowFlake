arch ?= x86_64
target ?= $(arch)-snowflake
entry := build/entry-$(arch).elf
kernel := build/kernel-$(arch).elf
iso := build/os-$(arch).iso

rust_os := target/$(target)/debug/libsnowflake.a

linker_script := src/arch/$(arch)/linker.ld

entry_source_file := src/arch/$(arch)/entry.asm

libc_source_files := $(shell find src/libc -name "*.c")
libc_object_files := $(patsubst src/libc/%.c, \
    build/libc/%.o, $(c_source_files))

boot_asm_source_files := $(shell find src/arch/$(arch)/boot -name "*.asm")
boot_asm_object_files := $(patsubst src/arch/$(arch)/boot/%.asm, \
    build/arch/$(arch)/boot/%.bin, $(boot_asm_source_files))

CARGO = cargo

CC = clang
CFLAGS = -target x86_64-pc-linux-gnu -ffreestanding -mcmodel=large -mno-red-zone -mno-mmx -mno-sse -mno-sse2 -nostdlib -Isrc/include/libc

NASM = nasm

LD = $(arch)-elf-ld
AS = $(arch)-elf-as
OBJCOPY = $(arch)-elf-objcopy
STRIP = $(arch)-elf-strip

.PHONY: all clean run iso

all: $(boot_asm_object_files) $(kernel)

clean:
	@rm -r build target

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(boot_asm_object_files) $(kernel)
	@mkdir -p build/iso
	@mkdir -p build/iso/boot
	@cp build/arch/$(arch)/boot/boot.bin build/iso/boot/boot.bin # Stage 1
	@cp build/arch/$(arch)/boot/loader.bin build/iso/boot/loader.bin # Stage 2
	@cp $(kernel) build/iso/kernel.bin
	@$(STRIP) build/iso/kernel.bin
	@genisoimage -R -J -c boot/bootcat -b boot/boot.bin -no-emul-boot -boot-load-size 4 -o $(iso) ./build/iso

$(entry):
	@mkdir -p $(shell dirname $@)
	@$(NASM) -f elf64 $(entry_source_file) -o $(entry)

$(kernel): $(entry) cargo $(rust_os) $(libc_object_files) $(linker_script)
	@$(LD) -n --gc-sections -T $(linker_script) -o $(kernel) $(entry) $(libc_object_files) $(rust_os)

# compile kernel files
cargo:
	@xargo build --target $(target)

# compile libc files
build/arch/$(arch)/boot/%.bin: src/arch/$(arch)/boot/%.asm
	@mkdir -p $(shell dirname $@)
	@$(NASM) -f bin $< -o $@

# compile libc files
build/libc/%.o: src/libc/%.c
	@mkdir -p $(shell dirname $@)
	@$(CC) $(CFLAGS) -c $< -o $@
