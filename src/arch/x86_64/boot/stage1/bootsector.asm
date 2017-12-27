; https://github.com/anchovieshat/cathode/blob/master/fatboot/boot.asm

org 0x7C5A

FATCACHE: equ 0x7E00
LOADLOC: equ 0x8000

%include "fat.inc"

section .text align=1

	jmp 0:start ; fix CS
start:
	xor ax, ax
	mov ds, ax
	mov es, ax
	mov ss, ax
	mov sp, 0x7C00 ; stack grows down from here
	mov bp, sp

	cld ; make no assumptions
	sti ; BIOS may do stuff with interrupts
	push dx ; save the drive letter for later; accessible at [bp-2]

	call ext_read_supported
	test ax, ax
	jz die ; don't bother if we don't support disk extensions

	; compute the first data sector and push it, too
	movzx eax, byte [0x7C00+bpb.num_fats]
	mov ebx, [0x7C00+bpb.sectors_per_fat_32]
	mul ebx
	jo die ; overflowed into dx, we don't handle that, so just give up

	movzx ebx, word [0x7C00+bpb.reserved_sectors]
	add eax, ebx

	; EAX now has first data sector, it will be accessible at [bp-6]
	push eax

	mov edi, LOADLOC
	mov eax, [0x7C00+bpb.root_cluster]
	call fat_read_file

	mov ebx, LOADLOC
.l:
	mov al, [ebx]
	test al, al
	jz .notfound
	cmp al, 0xE5 ; unused
	je .next
	mov cx, 11
	mov si, filename
	mov di, bx ; FIXME inconsistent
	rep cmpsb
	je .found
.next:
	add ebx, fatde_size
	jmp .l
.notfound:
	jmp die
.found:
	mov ax, [ebx+fatde.clust_hi]
	shl eax, 16
	mov ax, [ebx+fatde.clust_lo]
	mov edi, LOADLOC
	call fat_read_file
	test ax, ax
	jz die
	jmp LOADLOC

die:
	cli
	hlt
	jmp die

ext_read_supported:
	push bx
	push cx ; not used here, but interrupt may clobber

	mov ah, 0x41
	mov bx, 0x55aa
	stc ; carry clear on success
	int 0x13
	mov ax, 0 ; 0 = didn't work
	jc .ret ; failed if carry
	cmp bx, 0xaa55 ; failed if this isn't rotated
	jne .ret
	mov ax, 1 ; 1 = worked
.ret:
	pop cx
	pop bx
	ret

ext_read_sector:
	push dx
	push si

	mov [dap.lba], esi
	mov [dap.dest], edi
	mov ah, 0x42
	mov dx, [bp-2] ; grab drive letter
	mov si, dap
	stc
	int 0x13
	mov ax, 0 ; assume failed
	jc .ret ; failed if carry
	mov ax, 1

.ret:
	pop si
	pop dx
	ret
dap:
.size: db 0x10
._reserved: db 0
.blocks: dw 1
.dest: dd 0
.lba: dq 0

fat_read_cluster:
	push ebx
	push esi
	push eax

	sub eax, 2 ; cluster number - 2
	movzx ebx, byte [0x7C00+bpb.sectors_per_cluster]
	mul ebx ; * sectors_per_cluster
	jo die ; again, die on overflow
	add eax, [bp-6] ; + first_data_sector
	mov esi, eax
	mov [dap.blocks], bx
	call ext_read_sector

	pop eax
	pop esi
	pop ebx
	ret

fat_next_cluster:
	push ebx
	push edx
	push esi
	push edi

	mov ebx, 4
	mul ebx ; multiply by 4, as each 32-bit entry is 4 bytes
	jo die ; again, die on overflow

	cdq ; prepare for division
	movzx ebx, word [0x7C00+bpb.sector_size]
	div ebx ; edx has remainder

	movzx ebx, word [0x7C00+bpb.reserved_sectors] ; first FAT sector
	add eax, ebx

	; read sector eax, then index it by edx
	mov edi, FATCACHE
	mov esi, eax
	mov word [dap.blocks], 1
	call ext_read_sector
	test ax, ax
	jz .ret ; early return on failure

	mov eax, [FATCACHE+edx]
	and eax, 0x0FFFFFFF ; ignore "reserved" bits

.ret:
	pop edi
	pop esi
	pop edx
	pop ebx
	ret

fat_read_file:
	push ebx
.l:
	cmp eax, 0xFFFFFF8
	jae .ret
	call fat_read_cluster
	movzx ebx, word [0x7C00+bpb.sector_size]
	add edi, ebx
	call fat_next_cluster
	jmp .l

.ret:
	pop ebx
	ret

filename: db "LOADER  BIN"