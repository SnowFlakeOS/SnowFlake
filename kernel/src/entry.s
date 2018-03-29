.global _start
.global _info
.global _magic
.intel_syntax noprefix
.code32
.section .text

_start:
    push ecx
    push edx 
    call _start_uefi
    hlt

.code64
_start_uefi:
    mov _magic, ecx
    mov _info, edx
    cmp ecx, 0x71FF0EF1
    jz start_uefi
    hlt

.section .data

_info:
    .quad 0
    .quad 0xFFFFFFFF
_magic:
    .quad 0