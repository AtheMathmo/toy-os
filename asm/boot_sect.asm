[org 0x7c00]
KERNEL_OFFSET equ 0x1000    ; The memory offset to the KERNEL_OFFSET

    mov [BOOT_DRIVE], dl    ; Storing location of boot drive


    mov bp, 0x9000          ; Set the stack.
    mov sp, bp

    mov bx, MSG_REAL_MODE   ; Announce that we're booting in real mode
    call print_string

    call load_kernel        ; Load the kernel

    call switch_to_pm       ; Switch into protected mode

    jmp $

; Include our various routines
%include "./print/print_string.asm"
%include "./print/print_string_pm.asm"
%include "./disk/disk_load.asm"
%include "gdt.asm"
%include "switch_to_pm.asm"

[bits 16]
; Load kernel
load_kernel:
    mov bx, MSG_LOAD_KERNEL ; Print that we are loading kernel
    call print_string

    mov bx, KERNEL_OFFSET   ; Set up the disk load parameters
    mov dh, 15              ; We load the first 15 sectors
    mov dl, [BOOT_DRIVE]    ; excluding the boot sector
    call disk_load

    ret

[bits 32]
; This is our protected section
BEGIN_PM:

    mov ebx, MSG_PMODE
    call print_string_pm    ; Our 32 bit print routine

    call KERNEL_OFFSET      ; Jump to the address of the loaded
                            ; kernel code

    jmp $                   ; Hang


; Global Variables
BOOT_DRIVE      db 0
MSG_REAL_MODE   db "Started in 16-bit Real Mode", 0
MSG_PMODE       db "Successfully landed in 32-bit Protected Mode", 0
MSG_LOAD_KERNEL db "Loading kernel into memory.", 0

; Bootsector padding
times 510-($-$$) db 0
dw 0xaa55