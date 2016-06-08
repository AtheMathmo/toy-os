; Load DH sectors to ES:BX from drive DL

disk_load:
	push dx				; Store DX on stack so that later we can
						; recall how many sectors were requested 
						; to be read. Even if altered later.
	mov ah, 0x02		; BIOS read sector function
	mov al, dh 			; Read DH sectors
	mov ch, 0x00		; Select cylinder 0
	mov dh, 0x00		; Select head 0
	mov cl, 0x02		; Start reading from the second sector
	
	int 0x13			; BIOs interrupt to read from disk
	
	jc disk_error		; Jump if error (i.e. carry flag)

	pop dx 				; Restore DX from the stack
	cmp dh, al 			; If Sectors read != sectors expected
	jne disk_error		; Then display error message
	ret

disk_error:
	mov bx, DISK_ERROR_MSG
	call print_string
	jmp $

; Variables
DISK_ERROR_MSG:
	db "Disk read error!", 0
