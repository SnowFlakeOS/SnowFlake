; https:;github.com/anchovieshat/cathode/blob/master/stage2/load.asm

bits 16

extern s2main
global intcall

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

intcall_bounce16:
	mov ecx, cr0
	and ecx, ~1
	mov cr0, ecx
	mov [0x7C00], esp
	mov sp, di

	jmp 0:intcall_16
.return:
	mov ecx, cr0
	or ecx, 1
	mov cr0, ecx
	mov esp, [0x7C00]
	jmp 0x08:prot_start

intcall_16:
  mov ax, 0
  mov ss, ax
	sti

	;int 0x10

	pop ds
	pop es
	pop ax
	pop bx
	pop cx
	pop dx
	pop si
	pop di
	pop bp
	popf

	;int 0x10

	retf

; {{{
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
; }}}

; {{{
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
; }}}

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

intcall:
	push ebp
	mov ebp, esp
	pushfd
	pushad

	mov al, [ebp+8] ; interrupt number
	lea esi, [ebp+12] ; RM structure

	mov edi, 0x7C00 ; RM stack

  ; pushing RM return address
	o16 pushf ; FLAGS
	pop cx
	sub di, 2
	mov [di], cx
	sub di, 2
	mov word [di], 0 ; CS
	sub di, 2
	mov word [di], intcall_bounce16.return

  ; pushing interrupt address
	mov dl, 4 ; 4 bytes per interrupt in IVT (cs+off)
  mul dl ; *4
	mov bx, ax
	mov cx, [bx+2] ; push cs
	sub di, 2
	mov [di], cx
	mov cx, [bx] ; push off
	sub di, 2
	mov [di], cx

	sub di, 0x14 ; size of structure

	mov ecx, 0x14
	rep movsb

	sub di, 0x14

	call 0x18:intcall_bounce16

	popad
	popfd
	pop ebp
	ret


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