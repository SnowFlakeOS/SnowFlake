;=============================================================================
; @file     start.asm
; @brief    The kernel launcher.
; @details  The boot loader launches the kernel by jumping to _start.
;
; Copyright 2016 Brett Vickers.
; Use of this source code is governed by a BSD-style license that can
; be found in the MonkOS LICENSE file.
;=============================================================================

; The boot loader should have put us in 64-bit long mode.
bits 64

; Include boot loader's memory layout.
; Use a special section .start, which comes first in the linker.ld .text
; section. This way, the _start label will be given the lowest possible code
; address (0x00301000 in our case).
section .start
    global _start

    extern init        ; Exported by main.c

;-----------------------------------------------------------------------------
; @function     _start
; @brief        Kernel entry point, called by the boot loader.
;-----------------------------------------------------------------------------
_start:
    ; Call the kernel's main entry point. This function should never return.
    call    init

    ; If the function does return for some reason, hang the computer.
    .hang:
        cli
        hlt
        jmp     .hang