arch ?= x86_64
target ?= $(arch)-snowflake
boot2snow := build/arch/$(arch)/boot2snow/bootx64.efi
kernel := build/kernel/kernel.bin
img := build/snowflake-$(arch).img

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
	@qemu-system-x86_64 -bios ovmf.fd $(img)

img: $(img)

$(img): #$(kernel)
	@make -C arch/$(arch)/boot2snow
	@make -C kernel
	@dd if=/dev/zero of=$(img).tmp bs=512 count=98304
	@/usr/sbin/mkfs.vfat $(img).tmp
	@mmd -i $(img).tmp ::/boot2snow
	@mmd -i $(img).tmp ::/efi
	@mmd -i $(img).tmp ::/efi/boot
	@mcopy -i $(img).tmp $(kernel) ::/boot2snow
	@mcopy -i $(img).tmp res/only_logo.bmp ::/boot2snow
	@mcopy -i $(img).tmp res/full_logo.bmp ::/boot2snow
	@mcopy -i $(img).tmp res/boot2snow.conf ::/boot2snow
	@mcopy -i $(img).tmp $(boot2snow) ::/efi/boot
	@dd if=/dev/zero of=$@ bs=512 count=100352
	@/usr/sbin/parted $@ -s -a minimal mklabel gpt
	@/usr/sbin/parted $@ -s -a minimal mkpart EFI FAT32 2048s 93716s
	@/usr/sbin/parted $@ -s -a minimal toggle 1 boot
	@dd if=$@.tmp of=$@ bs=512 count=98304 seek=2048 conv=notrunc
	@rm -rf $@.tmp