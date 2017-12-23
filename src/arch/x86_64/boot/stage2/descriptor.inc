DESCRIPTOR_ATTR_4K equ (1 << 15)
DESCRIPTOR_ATTR_DB equ (1 << 14)
DESCRIPTOR_ATTR_L equ (1 << 13)
DESCRIPTOR_ATTR_PRESENT equ (1 << 7)
DESCRIPTOR_ATTR_DPL0 equ (0 << 5)
DESCRIPTOR_ATTR_DPL3 equ (3 << 5)
DESCRIPTOR_ATTR_SEG equ (1 << 4)
DESCRIPTOR_ATTR_EX equ (1 << 3)
DESCRIPTOR_ATTR_DC equ (1 << 2)
DESCRIPTOR_ATTR_RW equ (1 << 1)
DESCRIPTOR_ATTR_AC equ (1 << 0)

SELECTOR_GDT equ (0 << 2)
SELECTOR_LDT equ (1 << 2)
SELECTOR_RPL0 equ (0 << 0)
SELECTOR_RPL3 equ (3 << 0)
SELECTOR_RPL_MASK equ (0b11)

DESCRIPTOR_ATTR_CODE32 equ (DESCRIPTOR_ATTR_PRESENT | DESCRIPTOR_ATTR_DB | \
                            DESCRIPTOR_ATTR_SEG | DESCRIPTOR_ATTR_EX | \
                            DESCRIPTOR_ATTR_RW | DESCRIPTOR_ATTR_4K)
DESCRIPTOR_ATTR_DATA32 equ (DESCRIPTOR_ATTR_PRESENT | DESCRIPTOR_ATTR_DB | \
                            DESCRIPTOR_ATTR_SEG | DESCRIPTOR_ATTR_RW | \
                            DESCRIPTOR_ATTR_4K)

; for bootloader
DESCRIPTOR_ATTR_CODE16 equ (DESCRIPTOR_ATTR_PRESENT | DESCRIPTOR_ATTR_SEG | \
                            DESCRIPTOR_ATTR_EX | DESCRIPTOR_ATTR_RW)
DESCRIPTOR_ATTR_DATA16 equ (DESCRIPTOR_ATTR_PRESENT | DESCRIPTOR_ATTR_SEG | \
                            DESCRIPTOR_ATTR_RW)

DESCRIPTOR_ATTR_TSS equ (DESCRIPTOR_ATTR_PRESENT | 0x9) ; type 0x9 free 386 tss
GATE_ATTR_INTERRUPT equ (DESCRIPTOR_ATTR_PRESENT | 0xE) ; type 0xE 386 interrupt gate

; descriptor base, limit, attr
%macro descriptor 3
  dw %2 & 0xFFFF ; limit 1
  dw %1 & 0xFFFF ; base 1
  db (%1 >> 16) & 0xFF ; base 1
  dw ((%2 >> 8) & 0x0F00) | (%3 & 0xF0FF) ; attr 1, limit 2, attr 2
  db (%1 >> 24) & 0xFF ; base 2
%endmacro

; gate selector, offset, attr
%macro gate 3
  dw %2 & 0xFFFF ; offset 1
  dw %1 ; selector
  db 0 ; zero
  db %3 ; attr
  dw (%2 >> 16) & 0xFFFF ; offset 2
%endmacro