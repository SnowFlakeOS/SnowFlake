OUTPUT_FORMAT("elf64-x86-64")
ENTRY(_start)

KERNEL_BASE = 0xffffff0000000000;

SECTIONS
{
	. = KERNEL_BASE;

		.text : AT(ADDR(.text) - KERNEL_BASE)
	{
		*(.text)
	}

		/* Read-write data (uninitialized) and stack */
	.bss : AT(ADDR(.bss) - KERNEL_BASE)
	{
		*(.bss)
	}

	
    .rodata : AT(ADDR(.rodata) - KERNEL_BASE) {
        *(.rodata)
	}
}
