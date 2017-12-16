arch ?= x86_64
target ?= $(arch)-snowflake
entry := build/entry-$(arch).elf
kernel := build/kernel-$(arch).elf
iso := build/snowflake-$(arch).iso
img := build/os-$(arch).img

rust_os := target/$(target)/debug/libSnowFlake.a

linker_script := src/arch/$(arch)/linker.ld

entry_source_file := src/arch/$(arch)/entry.asm

libc_source_files := $(shell find src/libc -name "*.c")
libc_object_files := $(patsubst src/libc/%.c, \
    build/libc/%.o, $(libc_source_files))

CARGO = cargo

CC = clang
CFLAGS = -target $(arch)-pc-linux-gnu -ffreestanding -mcmodel=large -mno-red-zone -mno-mmx -mno-sse -mno-sse2 -nostdlib -Isrc/include/libc

NASM = nasm

LD = $(arch)-elf-ld
AS = $(arch)-elf-as
OBJCOPY = $(arch)-elf-objcopy
STRIP = $(arch)-elf-strip

.PHONY: all clean run iso

all: $(iso) $(boot_asm_object_files) $(kernel)

clean:
	@rm -r build target

run: $(iso)
	@qemu-system-x86_64 -m 256M -cdrom $(iso)

iso: $(iso)

$(iso): $(img)
	@mkdir -p build/iso
	@mkdir -p build/iso/boot
	@mkdir -p build/iso/efi/boot
	@mkdir -p build/iso/snow
	@cp $(img) build/iso/boot/boot.img
	#@cp build/arch/$(arch)/boot/boot.igloo build/iso/boot/boot.igloo # Stage 1
	#@cp $(kernel) build/iso/snow/kernel.igloo
	#@$(STRIP) build/iso/snow/kernel.igloo
	@mkisofs -R -J -c boot/bootcat -b boot/boot.img -no-emul-boot -boot-load-size 4 -o $(iso) ./build/iso

$(img): $(kernel)
	@make -C src/arch/$(arch)/boot
	@dd if=/dev/zero of=$(img) bs=512 count=1440
	@dd if=build/arch/$(arch)/boot/boot.igloo of=$(img) conv=notrunc

$(entry):
	@mkdir -p $(shell dirname $@)
	@$(NASM) -f elf64 $(entry_source_file) -o $(entry)

$(kernel): $(entry) cargo $(rust_os) $(libc_object_files) $(linker_script)
	@$(LD) -n --gc-sections -T $(linker_script) -o $(kernel) $(entry) $(libc_object_files) $(rust_os) -z max-page-size=0x1000

# compile kernel files
cargo:
	xargo build --target $(target)

# compile libc files
build/libc/%.o: src/libc/%.c
	@mkdir -p $(shell dirname $@)
	@$(CC) $(CFLAGS) -c $< -o $@
