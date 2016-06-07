[bits 32]

; Defining constants
VIDEO_MEMORY equ 0xb8000
WHITE_ON_BLACK equ 0x0f

; prints a null-terminated string pointed to by EDX
print_string_pm:
	pusha
	mov edx, VIDEO_MEMORY 				; Set edx to start of video memory

print_string_pm_loop:
	mov al, [ebx]						; Store the char at EBX in AL
	mov ah, WHITE_ON_BLACK 				; Store the attributes in AH

	cmp al, 0							; if al is 0, we reached end of string
	je print_string_pm_done

	mov [edx], ax						; Store char and attributes at current cell

	add ebx, 1							; Increment EX to next char in string
	add edx, 2							; Move to next char cell in video mem

	jmp print_string_pm_loop

print_string_pm_done:
	popa
	ret
