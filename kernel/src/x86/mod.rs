pub mod asm;

#[repr(C, packed)]
pub struct DescriptorTablePointer {
	pub limit: u16,
	pub offset: u64,
}

pub type IDTPointer = DescriptorTablePointer;