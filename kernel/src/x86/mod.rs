pub mod asm;

#[repr(C, packed)]
pub struct DescriptorTablePointer {
    pub limit: u16,
    pub offset: u64,
}

/// Tuple wrapper for the IDT Pointer.
pub struct IDTPointer(pub DescriptorTablePointer);
