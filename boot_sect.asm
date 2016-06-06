[ORG 0x7c00]
    mov [BOOT_DRIVE], dl    ; BIOS stores our boot drive in DL.

    mov bp, 0x8000          ; Set the stack far out of the way
    mov sp, bp

    mov bx, 0x9000          ; Load 5 sectors to 0x0000(ES):ox9000(BX)
    mov dh, 5               ; from the boot disk
    mov dl, [BOOT_DRIVE]
    call disk_load

    mov dx, [0x9000]        ; Print the first loaded word (0xdada)
    call print_hex

    mov dx, [0x9000 + 512]  ; Print the first word from second sector (0xface)
    call print_hex
    
    jmp $

%include "./print/print_string.asm"
%include "./print/print_hex.asm"
%include "disk_load.asm"

; Global Variables
BOOT_DRIVE: db 0

; Bootsector padding
times 510-($-$$) db 0
dw 0xaa55

; Add some data after the boot sector to test reading
times 256 dw 0xdada
times 256 dw 0xface