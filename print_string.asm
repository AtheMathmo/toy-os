; Prints a string using BIOS.
; The bx register should point to the address of the string ; to be printed.

print_string:
	pusha				; Push all register values on stack
	mov ah, 0x0e		; BIOS tele-type output

	continue:
		mov al, [bx]	; Move char at bx address into al
		add bx, 0x1		; Move the pointer in bx across 16 bits.
		cmp al, 0		; Compare character in al with 0
		je done			; If 0 then we're done

		int 0x10		; Call BIOS routine to print char in al
		jmp continue	; Continue printing string

	done:
		popa			; Pop all register values from stack
		ret