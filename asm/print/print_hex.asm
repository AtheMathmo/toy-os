; Prints a hex value using BIOS.
; The dx register should point to the address of the hex value
; to be printed.

print_hex:
	pusha				; Push all register values on stack
	
	mov ax, dx			; Move the hex value into ax

	mov bx, HEX_OUT		; Move address of HEX_OUT into bx
	mov al, ah 			; Make both halves equal to ah to get
						; only one byte

	add bx, 0x2			; Skip the starting '0x' on HEX_OUT
	
	call modify_hex_out ; Modifies HEX_OUT from bx with al and ah

	;; Repeat for mov al, ah (the upper byte)
	mov ax, dx			; Move the hex value into ax
	mov ah, al 			; Make the halves equal to al

	call modify_hex_out ; Modifies HEX_OUT from bx with al and ah

	mov bx, HEX_OUT		; print the string pointed to by BX
	call print_string

	popa				; Pop all register values from stack
	ret

; Functions

; Modifies the hex out value from the given index
; 
; Requires ah, al to both be set to the same byte
; Requires bx to be set to the index at which
; we begin modifying HEX_OUT
modify_hex_out:
	shr ah, 4			; Give ah the high nibble
	and al, 0x0f		; Give al the low nibble

	cmp ah, 0xa			; Check if this is an alpha hex value
	jl skip_1
	add ah, byte 0x7		; If so then add 7 for the ascii encoding
	skip_1:
		add ah, 0x30 	; Now add the char value
		mov [bx], ah

	add bx, 0x1			; Move the string pointer across one

	cmp al, 0xa
	jl skip_2
	add al, 0x7		; If so then add 7 for the ascii encoding
	skip_2:
		add al, 0x30 	; Now add the char value
		mov [bx], al

	add bx, 0x1

	ret

; Global Variables
HEX_OUT:
	db '0x0000',0	; This is 12 hex values + 1 bit