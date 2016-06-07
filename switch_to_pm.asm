[bits 16]
; Switch to protected mode
switch_to_pm:
    cli                     ; disable interrupts
    lgdt [gdt_descriptor]   ; Load the GDT register

    mov eax, cr0            ; Set the protection enabled bit
    or eax, 0x1
    mov cr0, eax

    jmp CODE_SEG:PModeInit

[bits 32]
PModeInit:
    mov ax, DATA_SEG
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov ebp, 0x90000
    mov esp, ebp

    call BEGIN_PM