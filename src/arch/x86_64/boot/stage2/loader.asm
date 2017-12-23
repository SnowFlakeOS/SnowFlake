; https://os.phil-opp.com/entering-longmode/
; http://wiki.osdev.org/Setting_Up_Long_Mode#The_Switch_from_Protected_Mode

org 0x10000

%include "stage2/descriptor.inc"

section .text

KERNEL_ADDRESS equ 0x100000

bits 16

_start:
cli

mov ax, cs
mov ds, ax
mov es, ax
mov fs, ax
mov gs, ax

xor ax, ax
mov ss, ax
mov sp, 0xffff

; Save Boot Device
mov [Boot_Device - $$], dl

; prepare GDTR
xor eax, eax
mov ax, cs
shl eax, 4
add eax, GDT32_PTR - $$
mov [ds:GDT32_reg - $$ + 2], eax ; logical address cs:GDT32_PTR to physical address
lgdt [ds:GDT32_reg - $$]

; Enable A20 Address Line
call    EnableA20

; Get VBE Mode Info
call    GetVBEMode

; Enable VBE Mode
; call    EnableVBE

; Enable Protect Mode
call    EnablePM

;-------------------------------------------------------------------------
; Variables
;-------------------------------------------------------------------------

; Boot Device

Boot_Device db 0

; GDT32

GDT32_PTR: descriptor 0, 0, 0 ; none
GDT32_CODE: descriptor 0, 0xFFFFF, DESCRIPTOR_ATTR_CODE32 | DESCRIPTOR_ATTR_DPL0
GDT32_DATA: descriptor 0, 0xFFFFF, DESCRIPTOR_ATTR_DATA32 | DESCRIPTOR_ATTR_DPL0
GDT32_CODE16: descriptor 0x10000, 0x0FFFF, DESCRIPTOR_ATTR_CODE16 | DESCRIPTOR_ATTR_DPL0
GDT32_DATA16: descriptor 0, 0x0FFFF, DESCRIPTOR_ATTR_DATA16 | DESCRIPTOR_ATTR_DPL0

GDT32_LENGTH equ $ - GDT32_PTR
GDT32_reg:

  dw GDT32_LENGTH - 1 ; GDT limit
  dd 0 ; GDT base, will be filled later

SELECTOR_CODE equ ((GDT32_CODE - GDT32_PTR) | SELECTOR_GDT | SELECTOR_RPL0)
SELECTOR_DATA equ ((GDT32_DATA - GDT32_PTR) | SELECTOR_GDT | SELECTOR_RPL0)
SELECTOR_CODE16 equ ((GDT32_CODE16 - GDT32_PTR) | SELECTOR_GDT | SELECTOR_RPL0)
SELECTOR_DATA16 equ ((GDT32_DATA16 - GDT32_PTR) | SELECTOR_GDT | SELECTOR_RPL0)

; GDT 64

bits 32

GDT64:                           ; Global Descriptor Table (64-bit).
    .Null: equ $ - GDT64         ; The null descriptor.
    dw 0                         ; Limit (low).
    dw 0                         ; Base (low).
    db 0                         ; Base (middle)
    db 0                         ; Access.
    db 0                         ; Granularity.
    db 0                         ; Base (high).
    .Code: equ $ - GDT64         ; The code descriptor.
    dw 0                         ; Limit (low).
    dw 0                         ; Base (low).
    db 0                         ; Base (middle)
    db 10011010b                 ; Access (exec/read).
    db 00100000b                 ; Granularity.
    db 0                         ; Base (high).
    .Data: equ $ - GDT64         ; The data descriptor.
    dw 0                         ; Limit (low).
    dw 0                         ; Base (low).
    db 0                         ; Base (middle)
    db 10010010b                 ; Access (read/write).
    db 00000000b                 ; Granularity.
    db 0                         ; Base (high).
    .Pointer:                    ; The GDT-pointer.
    dw $ - GDT64 - 1             ; Limit.
    dq GDT64                     ; Base.

bits 16

;-------------------------------------------------------------------------
; Functions
;-------------------------------------------------------------------------

;-------------------------------------------------------------------------
; - Get VBE Mode Info
;-------------------------------------------------------------------------

GetVBEMode:
mov ax, 0x4F01

; 1024x768 16bit(R(5):G(6):B(5)) Color     
mov cx, 0x117
mov bx, 0x07E0   
mov es, bx         
mov di, 0x00

 int 0x10 

;-------------------------------------------------------------------------
; - Enable VBE Mode
;-------------------------------------------------------------------------

EnableVBE:
    ;mov ax, 0x4F02

     ; 1024x768 16bit(R(5):G(6):B(5)) Color 
    ;mov bx, 0x4117
    ;int 0x10       

;-------------------------------------------------------------------------
; - Enable A20 Address Line
;-------------------------------------------------------------------------

EnableA20:

    ; Preserve ax register.
    push    ax

    ; Check if the A20 line is already enabled.
    call    TestA20
    jc      .done

    .attempt1:

        ; Attempt enabling with the BIOS.
        mov     ax,     0x2401
        int     0x15

        ; Check if A20 line is now enabled.
        call    TestA20
        jc      .done

    .attempt2:

        ; Attempt enabling with the keyboard controller.
        call    .attempt2.wait1

        ; Disable keyboard
        mov     al,     0xad
        out     0x64,   al
        call    .attempt2.wait1

        ; Read from input
        mov     al,     0xd0
        out     0x64,   al
        call    .attempt2.wait2

        ; Get keyboard data
        in      al,     0x60
        push    eax
        call    .attempt2.wait1

        ; Write to output
        mov     al,     0xd1
        out     0x64,   al
        call    .attempt2.wait1

        ; Send data
        pop     eax
        or      al,     2
        out     0x60,   al
        call    .attempt2.wait1

        ; Enable keyboard
        mov     al,     0xae
        out     0x64,   al
        call    .attempt2.wait1

        ; Check if the A20 line is now enabled.
        call    TestA20
        jc      .done

        jmp     .attempt3

        .attempt2.wait1:

            in      al,     0x64
            test    al,     2
            jnz     .attempt2.wait1
            ret

        .attempt2.wait2:

            in      al,     0x64
            test    al,     1
            jz      .attempt2.wait2
            ret

    .attempt3:

        ; Attempt enabling with the FAST A20 feature.
        in      al,     0x92
        or      al,     2
        out     0x92,   al
        xor     ax,     ax

        ; Check if A20 line is now enabled.
        call    TestA20

    .done:

        ; Restore register.
        pop     ax

        ret

;-------------------------------------------------------------------------
; - Test A20 Address Line
;-------------------------------------------------------------------------

TestA20:

    ; Preserve registers.
    push    ds
    push    es
    pusha

    ; Initialize return result to "not enabled".
    clc

    ; Set es segment register to 0x0000.
    xor     ax,     ax
    mov     es,     ax

    ; Set ds segment register to 0xffff.
    not     ax
    mov     ds,     ax

    ; If the A20 line is disabled, then es:di and ds:si will point to the same
    ; physical memory location due to wrap-around at 1 MiB.
    ;
    ; es:di = 0000:0500 = 0x0000 * 16 + 0x0500 = 0x00500 = 0x0500
    ; ds:si = ffff:0510 = 0xffff * 16 + 0x0510 = 0x10500 = 0x0500
    mov     di,     0x0500
    mov     si,     0x0510

    ; Preserve the original values stored at es:di and ds:si.
    mov     ax,     [es:di]
    push    ax
    mov     ax,     [ds:si]
    push    ax

    ; Write different values to each logical address.
    mov     byte [es:di],   0x00
    mov     byte [ds:si],   0xff

    ; If a store to ds:si changes the value at es:di, then memory wrapped and
    ; A20 is not enabled.
    cmp     byte [es:di],   0xff

    ; Restore the original values stored at es:di and ds:si.
    pop     ax
    mov     [ds:si],    ax
    pop     ax
    mov     [es:di],    ax

    je      .done

    .enabled:

        ; Set the carry flag to indicate the A20 line is enabled.
        stc

    .done:

        ; Restore registers.
        popa
        pop     es
        pop     ds

        ret

;-------------------------------------------------------------------------
; - Eanble Protected Mode
;-------------------------------------------------------------------------

EnablePM:
    mov eax, cr0
    or eax, 1
    mov cr0, eax

    ; go to protect mode
    xor eax, eax
    mov ax, cs
    shl eax, 4
    add eax, _start32 - $$
    mov [ds:ljmp_address - $$], eax ; logical address cs:_start32 to physical address

    ; ljmp dword SELECTOR_CODE:_start32
    db 0x66 ; operand-size override
    db 0xea ; ljmp
    ljmp_address dd 0xdeadbeef ; will be changed
    dw SELECTOR_CODE 

;-------------------------------------------------------------------------
; - Protected Mode Init
;-------------------------------------------------------------------------

bits 32
_start32:
    cli

    mov ax, SELECTOR_DATA
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    call    EnableLM

;-------------------------------------------------------------------------
; - Eanble Long Mode
;-------------------------------------------------------------------------

EnableLM:

    call build_page_tables


;Enable PAE
mov eax, cr4                 
or eax, 1 << 5               
mov cr4, eax

;# Optional : Enable global-page mechanism by setting CR0.PGE bit to 1
mov eax, cr4                 
or eax, 1 << 7               
mov cr4, eax

;Load CR3 with PML4 base address
;NB: in some examples online, the address is not offseted as it seems to
;be in the proc datasheet (if you were wondering about this strange thing).
mov eax, 0x1000
mov cr3, eax

; Check PML5 & Enable PML5
mov eax, 0x7                 ; You might want to check for page 7 first!
xor ecx, ecx
cpuid
test ecx, (1<<16)
jnz .5_level_paging

;Set LME bit in EFER register (address 0xC0000080)
mov ecx, 0xC0000080     ;operand of 'rdmsr' and 'wrmsr'
rdmsr                   ;read before pr ne pas écraser le contenu
or eax, 1 << 8          ;eax : operand de wrmsr
wrmsr

;Enable paging by setting CR0.PG bit to 1
mov eax, cr0
or eax, (1 << 31)
mov cr0, eax

;Load 64-bit GDT
lgdt [GDT64.Pointer]

;Jump to code segment in 64-bit GDT
jmp GDT64.Code:_start64

; Enable PML5
.5_level_paging:
    mov eax, cr4
    or eax, (1<<12) ;CR4.LA57
    mov cr4, eax

build_page_tables:
    ;PML4 starts at 0x1000
    ;il faut laisser la place pour tte la page PML4/PDP/PD ie. 0x1000

    ;PML4 @ 0x1000
    mov eax, 0x2000         ;PDP base address            
    or eax, 0b11            ;P and R/W bits
    mov ebx, 0x1000         ;MPL4 base address
    mov [ebx], eax

    ;PDP @ 0x2000; maps 64Go
    mov eax, 0x3000         ;PD base address
    mov ebx, 0x2000         ;PDP physical address   
    mov ecx, 64             ;64 PDP

    build_PDP:
        or eax, 0b11    
        mov [ebx], eax
        add ebx, 0x8
        add eax, 0x1000     ;next PD page base address
        loop build_PDP

    ;PD @ 0x3000 (ends at 0x4000, fits below 0x7c00)
    ; 1 entry maps a 2MB page, the 1st starts at 0x0
    mov eax, 0x0            ;1st page physical base address     
    mov ebx, 0x3000         ;PD physical base address
    mov ecx, 512                        

    build_PD:
        or eax, 0b10000011      ;P + R/W + PS (bit for 2MB page)
        mov [ebx], eax
        add ebx, 0x8
        add eax, 0x200000       ;next 2MB physical page
        loop build_PD

    ;(tables end at 0x4000 => fits before Bios boot sector at 0x7c00)
    ret

;-------------------------------------------------------------------------
; - Long Mode Init
;-------------------------------------------------------------------------

bits 64

_start64:
    mov ax, SELECTOR_DATA
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax
    hlt

die:
    cli
    hlt
    jmp die