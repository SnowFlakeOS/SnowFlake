use64

global _start

extern init

_start:
    call init     ; Call our kernel's main() function
    hlt