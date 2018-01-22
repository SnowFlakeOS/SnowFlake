; https://github.com/anchovieshat/cathode/blob/master/stage2/load.asm

bits 16

extern s2main

section .text16
start:
	mov dx, [bp-2] ; keep drive letter around
	mov bp, msg
	mov cx, msg.size
	call print

	mov bp, disab
	mov cx, disab.size
	call check_a20
	test ax, ax
	jnz .s
	call print
	call die
.s:
	cli
.goto_prot
	lgdt [gdtr]
	mov eax, cr0
	or eax, 1
	mov cr0, eax
	push dword 0x08
	push dword prot_main
	jmp 0x08:prot_start

die:
	cli
	hlt
	jmp die

print:
	push ax
	push bx
.l:
	mov al, [bp]
	inc bp
	mov ah, 0x0E
	xor bx, bx
	int 0x10
	loop .l
	pop bx
	pop ax
	ret
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

section .text
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
	call s2main
	hlt
	jmp $

section .rodata

msg: db "Booting stage2...",0xa,0xd
.size: equ $-msg

disab: db "A20 disabled...",0xa,0xd
.size: equ $-disab

gdt:
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
gdtr: dw (gdt.end-gdt)-1
	  dd gdt