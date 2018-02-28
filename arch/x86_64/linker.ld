ENTRY(_start)

KERNEL_VMA = 0xFFFF800000000000;
KERNEL_LMA = 0x100000;

SECTIONS {
  . = KERNEL_LMA;
	KERNEL_BASE = .;
  . += SIZEOF_HEADERS;
  LOAD_ADDR = .;

  .init : AT(ADDR(.init)) {
		*(.inittext)
		. = ALIGN(8);
		*(.initdata)
  }

  . += KERNEL_VMA;

  .rodata ALIGN(0x1000): AT(ADDR(.rodata) - KERNEL_VMA) {
    *(.rodata .rodata.*)
  }

  .text ALIGN(0x1000): AT(ADDR(.text) - KERNEL_VMA) {
    *(.text .text.*)
  }

  .data ALIGN(0x1000): AT(ADDR(.data) - KERNEL_VMA)  {
    *(.data .data.*)
  }

  .bss ALIGN(0x1000): AT(ADDR(.bss) - KERNEL_VMA)  {
    *(.bss .bss.*)
  }

  .got ALIGN(0x1000): AT(ADDR(.got) - KERNEL_VMA)  {
    *(.got)
  }

  .got.plt ALIGN(0x1000): AT(ADDR(.got.plt) - KERNEL_VMA)  {
    *(.got.plt)
  }

  KERNEL_END = (. + 0xFFF)&0xFFFFFFFFFFFFF000;
}