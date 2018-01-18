arch ?= x86_64
target ?= $(arch)-snowflake
entry := build/entry-$(arch).o
kernel := build/kernel-$(arch).elf
img := build/snowflake-$(arch).img
rust_os := target/$(target)/debug/libSnowFlake.a

linker_script := src/arch/$(arch)/linker.ld

entry_source_file := src/arch/$(arch)/entry.asm

CARGO = cargo
NASM = nasm

LD = $(arch)-elf-ld
AS = $(arch)-elf-as
OBJCOPY = $(arch)-elf-objcopy
STRIP = $(arch)-elf-strip

.PHONY: all clean run img
all: $(img)

clean:
	@rm -r build #target

run: $(img)
	@qemu-system-x86_64 -enable-kvm -cpu host -serial file:virtual.log -vga std -hda $(img)

img: $(img)

$(img): #$(kernel)
	@make -C src/arch/x86_64/boot2snow
	@dd if=/dev/zero of=$(img) bs=1M count=10
	@mkfs.vfat -F32 $(img)
	@dd if=build/arch/$(arch)/boot/stage1.bin of=$(img) conv=notrunc bs=1 count=420 seek=90
	@mcopy -D o -D O -ni $(img) build/arch/$(arch)/boot/stage2.bin ::/stage2.bin
	#@mkisofs -R -J -c boot/bootcat -b boot/boot.bin -no-emul-boot -boot-load-size 4 -o $(iso) ./build/iso

$(entry):
	@mkdir -p $(shell dirname $@)
	@$(NASM) -f elf64 $(entry_source_file) -o $(entry)

$(kernel): $(entry) cargo $(rust_os) $(linker_script)
	@$(LD) -n --gc-sections -T $(linker_script) -o $(kernel) $(entry) $(rust_os) -z max-page-size=0x1000

# compile kernel files
cargo:
	xargo build --target $(target)