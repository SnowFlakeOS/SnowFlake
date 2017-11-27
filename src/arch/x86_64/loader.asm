[bits 64]

extern init
global _start

_start:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    mov rsp, 0xffffff00001ffff8
    xor rbp, rbp
    
    call init     ; Call our kernel's main() function
    hlt