//! Assembly routines for the translation lokaside buffer.

/// Invalidates the translation lookaside buffer entry for the
/// page containing the specified address.
/// [More documentation here](http://x86.renejeschke.de/html/file_module_x86_id_144.html).
///
/// # Safety
///
/// Will produce a General Protection fault if the current
/// priviledge level is not 0.
pub unsafe fn invalidate_entry(address: usize) {
    asm!("invlpg ($0)" :: "r" (address) : "memory");
}

pub unsafe fn flush_tlb() {
    let cr3: usize;
    // Mov cr3 reg out
    asm!("mov %cr3, $0" : "=r" (cr3));

    // Write value to cr3 reg again
    asm!("mov $0, %cr3" :: "r" (cr3) : "memory");
}
