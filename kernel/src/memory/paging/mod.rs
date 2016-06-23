pub use self::entry::*;
pub use self::mapper::Mapper;
use self::temporary_page::TemporaryPage;

use memory::{PAGE_SIZE, Frame, FrameAllocator};
use core::ops::{Deref, DerefMut};
use x86::asm;
use x86::asm::tlb;

mod entry;
mod table;
mod temporary_page;
mod mapper;

const ENTRY_COUNT: usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

#[derive(Debug, Clone, Copy)]
pub struct Page {
    number: usize,
}

impl Page {
    /// Return the `Page` which contains the virtual address.
    pub fn containing_address(address: VirtualAddress) -> Page {
        assert!(address < 0x0000_8000_0000_0000 || address >= 0xffff_8000_0000_0000,
                "invalid address: 0x{:x}",
                address);
        Page { number: address / PAGE_SIZE }
    }

    fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }

    fn p4_index(&self) -> usize {
        (self.number >> 27) & 0o777
    }

    fn p3_index(&self) -> usize {
        (self.number >> 18) & 0o777
    }

    fn p2_index(&self) -> usize {
        (self.number >> 9) & 0o777
    }

    fn p1_index(&self) -> usize {
        (self.number >> 0) & 0o777
    }
}

pub struct ActivePageTable {
    mapper: Mapper,
}

impl Deref for ActivePageTable {
    type Target = Mapper;

    fn deref(&self) -> &Mapper {
        &self.mapper
    }
}

impl DerefMut for ActivePageTable {
    fn deref_mut(&mut self) -> &mut Mapper {
        &mut self.mapper
    }
}

impl ActivePageTable {
    /// Creates a new ActivePageTable
    ///
    /// # Safety
    ///
    /// The function is marked unsafe as only a single ActivePageTable
    /// may exist at a time.
    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable { mapper: Mapper::new() }
    }

    /// Calls the function with the temporary page and then reinstates
    /// the active page table.
    pub fn with<F>(&mut self,
                   table: &mut InactivePageTable,
                   temporary_page: &mut temporary_page::TemporaryPage,
                   f: F)
        where F: FnOnce(&mut Mapper)
    {
        {
            let backup = Frame::containing_address(unsafe { asm::cr3() });

            // Map temporary page to current p4 table
            let p4_table = temporary_page.map_table_frame(backup.clone(), self);

            // Overwrite recursive mapping
            self.p4_mut()[511].set(table.p4_frame.clone(), PRESENT | WRITABLE);
            unsafe { tlb::flush_tlb() };

            // Execute f in the new context
            f(self);

            // Restore the recursive mapping to original p4 table
            p4_table[511].set(backup, PRESENT | WRITABLE);
            unsafe { tlb::flush_tlb() };
        }

        temporary_page.unmap(self);

    }

    pub fn switch(&mut self, new_table: InactivePageTable) -> InactivePageTable {
        let old_table = InactivePageTable {
            p4_frame: Frame::containing_address(unsafe { asm::cr3() }),
        };

        unsafe {
            asm::write_to_cr3(new_table.p4_frame.start_address());
        }

        old_table
    }
}

pub struct InactivePageTable {
    p4_frame: Frame,
}

impl InactivePageTable {
    pub fn new(frame: Frame,
               active_table: &mut ActivePageTable,
               temporary_page: &mut TemporaryPage)
               -> Self {
        {
            let table = temporary_page.map_table_frame(frame.clone(), active_table);

            // Zero the table
            table.zero();

            // Set recursive mapping for the table
            table[511].set(frame.clone(), PRESENT | WRITABLE);
        }
        temporary_page.unmap(active_table);

        InactivePageTable { p4_frame: frame }
    }
}

/// Remap the kernel using the allocator.
pub fn remap_the_kernel<A>(allocator: &mut A)
    where A: FrameAllocator
{
    let mut temporary_page = TemporaryPage::new(Page { number: 0xcafebabe }, allocator);

    let mut active_table = unsafe { ActivePageTable::new() };
    let mut new_table = {
        let frame = allocator.allocate_frame().expect("no more frames");
        InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
    };

    active_table.with(&mut new_table, &mut temporary_page, |mapper| {
        // Identity map the kernel
        let start_frame = Frame::containing_address(super::kernel_memory_start());
        let end_frame = Frame::containing_address(super::kernel_memory_end());

        for frame in Frame::range_inclusive(start_frame, end_frame) {
            mapper.identity_map(frame, WRITABLE, allocator);
        }

        // Identity map the VGA buffer
        let vga_bugger_frame = Frame::containing_address(0xb8000);
        mapper.identity_map(vga_bugger_frame, WRITABLE, allocator);
    });

    // Switch to using the newly mapped table
    active_table.switch(new_table);
    println!("Using the new table!");
}
