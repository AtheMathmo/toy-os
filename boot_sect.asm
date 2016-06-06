[ORG 0x7c00]

    mov bx, HELLO_MSG
    call print_string

    mov bx, GOODBYE_MSG
    call print_string

    jmp $

%include "print_string.asm"

; DATA
HELLO_MSG:
    db 'Hello, world!', 0

GOODBYE_MSG:
    db 'Goodbye!', 0

    times 510-($-$$) db 0
    dw 0xaa55