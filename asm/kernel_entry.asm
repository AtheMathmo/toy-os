[bits 32]
[extern rust_main]

global _start
_start:
	call rust_main		; Enter the kernel via the main function
	jmp $				; Hang when we return from the kernel