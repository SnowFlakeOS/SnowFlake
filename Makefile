arch ?= x86_64
target ?= $(arch)-snowflake
boot2snow := build/boot2snow/bootx64.efi
kernel := build/kernel/kernel.bin
img := build/snowflake-$(arch).img

CARGO = cargo
NASM = nasm

LD = $(arch)-elf-ld
AS = $(arch)-elf-as
OBJCOPY = $(arch)-elf-objcopy
STRIP = $(arch)-elf-strip

.PHONY: all clean run run-debug img
all: $(img)

clean:
	@rm -r build #target

run: $(img)
	@qemu-system-x86_64 -m 4096 -serial mon:stdio -net none -vga std -bios ovmf.fd $(img)

run-debug: $(img)
	@qemu-system-x86_64 -s -S -m 1024 -serial mon:stdio -net none -vga std -bios ovmf.fd $(img)

img: $(img)

$(img):
	@make -C boot2snow
	@make -C kernel
	@dd if=/dev/zero of=$(img).tmp bs=512 count=98304
	@mkfs.vfat $(img).tmp
	@mmd -i $(img).tmp ::/boot2snow
	@mmd -i $(img).tmp ::/efi
	@mmd -i $(img).tmp ::/efi/boot
	@mcopy -i $(img).tmp $(kernel) ::/boot2snow
	@mcopy -i $(img).tmp res/only_logo.bmp ::/boot2snow
	@mcopy -i $(img).tmp res/full_logo.bmp ::/boot2snow
	@mcopy -i $(img).tmp res/boot2snow.conf ::/boot2snow
	@mcopy -i $(img).tmp $(boot2snow) ::/efi/boot
	@dd if=/dev/zero of=$@ bs=512 count=100352
	@parted $@ -s -a minimal mklabel gpt
	@parted $@ -s -a minimal mkpart EFI FAT32 2048s 93716s
	@parted $@ -s -a minimal toggle 1 boot
	@dd if=$@.tmp of=$@ bs=512 count=98304 seek=2048 conv=notrunc
	@rm -rf $@.tmp