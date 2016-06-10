//! Some useful assembly routines
use super::IDTPointer;

pub fn get_cs() -> u16 {
	let cs_segment : u16;
	unsafe {
		asm!("mov %cs, $0" : "=r" (cs_segment))
	};
	cs_segment
}

/// Loads the IDT into the IDTR.
pub unsafe fn lidt(table_pointer: &IDTPointer) {
	asm!("lidt ($0)" :: "r" (table_pointer) : "memory");
}