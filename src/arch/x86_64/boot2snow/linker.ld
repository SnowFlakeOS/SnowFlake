ENTRY(start)

SECTIONS {
	. = 0x8000;
	.text16 : { *(.text16) }
	.text : { *(.text) }
}