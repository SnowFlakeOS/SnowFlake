; https://github.com/anchovieshat/cathode/blob/master/stage2/load.asm

org 0x8000
bits 16

%include "fat.inc"

; TODO - Load kernel from disk

start:
	mov dx, [bp-2] ; keep drive letter around

	call check_a20
	test ax, ax
	jnz .s
	call .die

    .s:
	    cli

    .goto_prot
	    lgdt [gdtr32]
	    mov eax, cr0
	    or eax, 1
	    mov cr0, eax
	    push dword 0x08
	    push dword prot_main
	    jmp 0x08:prot_start

    .die:
	    cli
	    hlt
	    jmp .die

check_a20:
    push ds
	push es
	push ax
	push di
	push si

	cli ; so we don't damage something
	xor ax, ax
	mov es, ax ; normal es
	not ax
	mov ds, ax ; 0xFFFF ds

	mov di, 0x0500
	mov si, 0x0510
	mov al, [es:di] ; save originals
	push ax
	mov al, [ds:si]
	push ax

	mov byte [es:di], 0x00 ; write junk
	mov byte [ds:si], 0xFF ; here too
	cmp byte [es:di], 0xFF ; did we see it here?

	pop ax ; restore
	mov [ds:si], al
	pop ax
	mov [es:di], al

	mov ax, 0
	je .ret
	mov ax, 1

.ret:
	pop si
	pop di
	pop ax
	pop es
	pop ds
	sti
	ret

;-------------------------------------------------------------------------
; - Protected Mode Init
;-------------------------------------------------------------------------

bits 32

    prot_start:
	    mov eax, 0x10
	    mov ds, eax
	    mov es, eax
	    mov fs, eax
	    mov gs, eax
	    mov ss, eax
	    retf

    prot_main:
	    mov esp, 0x8000
	    mov ebp, esp
	    mov eax, edx
	    movzx eax, dl
	    push eax ; push drive letter, at [bp-4]
	    cld ; just checking
	    call EnableLM

;-------------------------------------------------------------------------
; - Eanble Long Mode
;-------------------------------------------------------------------------

EnableLM:

    call build_page_tables

    ; Enable PAE
    mov eax, cr4                 
    or eax, 1 << 5               
    mov cr4, eax

    ; # Optional : Enable global-page mechanism by setting CR0.PGE bit to 1
    mov eax, cr4                 
    or eax, 1 << 7               
    mov cr4, eax

    ; Load CR3 with PML4 base address
    ; NB: in some examples online, the address is not offseted as it seems to
    ; be in the proc datasheet (if you were wondering about this strange thing).
    mov eax, 0x1000
    mov cr3, eax

    ; Check PML5 & Enable PML5
    mov eax, 0x7                 ; You might want to check for page 7 first!
    xor ecx, ecx
    cpuid
    test ecx, (1<<16)
    jnz .5_level_paging

    ; Set LME bit in EFER register (address 0xC0000080)
    mov ecx, 0xC0000080     ; operand of 'rdmsr' and 'wrmsr'
    rdmsr                   ; read before pr ne pas écraser le contenu
    or eax, 1 << 8          ; eax : operand de wrmsr
    wrmsr

    ; Enable paging by setting CR0.PG bit to 1
    mov eax, cr0
    or eax, (1 << 31)
    mov cr0, eax

    ; Load 64-bit GDT
    lgdt [GDT64.Pointer]

    ; Jump to code segment in 64-bit GDT
    jmp GDT64.Code:(_start64)

    ; Enable PML5
    .5_level_paging:
        mov eax, cr4
        or eax, (1<<12) ;CR4.LA57
        mov cr4, eax

build_page_tables:
    ; PML4 starts at 0x1000
    ; il faut laisser la place pour tte la page PML4/PDP/PD ie. 0x1000

    ; PML4 @ 0x1000
    mov eax, 0x2000         ;PDP base address            
    or eax, 0b11            ;P and R/W bits
    mov ebx, 0x1000         ;MPL4 base address
    mov [ebx], eax

    ; PDP @ 0x2000; maps 64Go
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

    mov eax, 0x10
	mov ds, eax
	mov es, eax
	mov fs, eax
	mov gs, eax
	mov ss, eax

    mov esp, 0x8000
	mov ebp, esp
	mov eax, edx
	movzx eax, dl

    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax
    hlt

.die:
    cli
    hlt
    jmp .die

bits 16

gdt32:
.null: dd 0
	   db 0
	   db 00010000b
	   dw 0
.code32: dw 0xffff
	     dw 0
	     db 0
	     db 10011010b
	     db 11001111b
	     db 0
.data32: dw 0xffff
	     dw 0
	     db 0
	     db 10010010b
	     db 11001111b
	     db 0
.code16: dw 0xffff
		 dw 0
		 db 0
		 db 10011010b
		 db 00000000b
		 db 0
.data16: dw 0xffff
		 dw 0
		 db 0
		 db 10010010b
		 db 00000000b
		 db 0
.end:
gdtr32: dw (gdt32.end-gdt32)-1
	  dd gdt32

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
