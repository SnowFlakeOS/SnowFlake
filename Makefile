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
	@qemu-system-x86_64 -m 1024 -bios ovmf.fd $(img)

img: $(img)

$(img): #$(kernel)
	@make -C arch/$(arch)/boot2snow
	@make -C kernel
	@dd if=/dev/zero of=$(img).2 bs=512 count=98304
	@mkfs.vfat $(img).2
	@mmd -i $(img).2 ::/boot2snow
	@mmd -i $(img).2 ::/efi
	@mmd -i $(img).2 ::/efi/boot
	@mcopy -i $(img).2 $(kernel) ::/boot2snow
	@mcopy -i $(img).2 splash.bmp ::/boot2snow
	@mcopy -i $(img).2 $(boot2snow) ::/efi/boot
	dd if=/dev/zero of=$@.tmp bs=512 count=100352
	parted $@.tmp -s -a minimal mklabel gpt
	parted $@.tmp -s -a minimal mkpart EFI FAT16 2048s 93716s
	parted $@.tmp -s -a minimal toggle 1 boot
	dd if=$@.2 of=$@.tmp bs=512 count=98304 seek=2048 conv=notrunc
	mv $@.tmp $@