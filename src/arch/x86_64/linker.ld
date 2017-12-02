ENTRY(_start)
OUTPUT_FORMAT("elf64-x86-64")

kernel_base = 0xffffff0000000000;

SECTIONS {
    . = kernel_base;

    .text : AT(ADDR(.text) - kernel_base) {
        * (.text)
    }

    .rodata : AT(ADDR(.rodata) - kernel_base) {
        * (.rodata)
    }

    .data : AT(ADDR(.data) - kernel_base) {
        * (.data)
    }
}