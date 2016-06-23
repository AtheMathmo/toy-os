//! Some useful assembly routines

pub mod tlb;

use super::IDTPointer;

// Return the CS Segment
pub fn get_cs() -> u16 {
    let cs_segment: u16;
    unsafe { asm!("mov %cs, $0" : "=r" (cs_segment)) };
    cs_segment
}

/// Return the cr3 register
pub unsafe fn cr3() -> usize {
    let cr3: usize;
    // Mov cr3 reg out
    asm!("mov %cr3, $0" : "=r" (cr3));
    cr3
}

/// Write the specified value to the cr3 register
pub unsafe fn write_to_cr3(cr3: usize) {
    // Write value to cr3 reg again
    asm!("mov $0, %cr3" :: "r" (cr3) : "memory");
}

/// Loads the IDT into the IDTR.
pub unsafe fn lidt(table_pointer: &IDTPointer) {
    // Unwrap the inner pointer just to be safe
    let ref ptr = table_pointer.0;

    // Load the IDT
    asm!("lidt ($0)" :: "r" (ptr) : "memory");
}
