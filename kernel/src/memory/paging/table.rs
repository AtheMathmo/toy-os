use core::ops::{Index, IndexMut};

use memory::FrameAllocator;
use memory::paging::entry::*;
use memory::paging::ENTRY_COUNT;

use core::marker::PhantomData;

/// Table locations in octal
pub const P4: *mut Table<Level4> = 0xffffffff_fffff000 as *mut _;

/// Trait for the table level of each page table.
pub trait TableLevel {}

/// Level 4 used for the P4 table
pub enum Level4 {}
/// Level 3 used for the P3 table
pub enum Level3 {}
/// Level 2 used for the P2 table
pub enum Level2 {}
/// Level 1 used for the P1 table
pub enum Level1 {}

impl TableLevel for Level4 {}
impl TableLevel for Level3 {}
impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

/// A Hierarchical trait used for P4-P2.
///
/// This trait allows us to safely implement a recursive
/// page mapping structure.
pub trait HierarchicalLevel: TableLevel {
	type NextLevel : TableLevel;
}

impl HierarchicalLevel for Level4 {
    type NextLevel = Level3;
}
impl HierarchicalLevel for Level3 {
    type NextLevel = Level2;
}
impl HierarchicalLevel for Level2 {
    type NextLevel = Level1;
}

/// A page table
pub struct Table<L: TableLevel> {
    entries: [Entry; ENTRY_COUNT],
    _level: PhantomData<L>,
}

impl<L: TableLevel> Index<usize> for Table<L> {
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

impl<L: TableLevel> IndexMut<usize> for Table<L> {
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }
}

impl<L: TableLevel> Table<L> {
    /// Set all entries in the table to be unused
    pub fn zero(&mut self) {
        unsafe {
            use vga_buffer::print_error;
            print_error(format_args!("Zeroing 0x{:x}", self as *const _ as usize));
        }

        for entry in self.entries.iter_mut() {
            entry.set_unused();
		}
    }
}

impl<L: HierarchicalLevel> Table<L> {
    /// Gets an Option<ptr> to the next table
    pub fn next_table(&self, index: usize) -> Option<&Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe { &*(address as *const _) })
    }

    /// Gets a mutable Option<ptr> to the next table
    pub fn next_table_mut(&mut self, index: usize) -> Option<&mut Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe { &mut *(address as *mut _) })
    }

    fn next_table_address(&self, index: usize) -> Option<usize> {
        let entry_flags = self[index].flags();

        if entry_flags.contains(PRESENT) && !entry_flags.contains(HUGE_PAGE) {
            let table_address = self as *const _ as usize;
            Some((table_address << 9) | (index << 12))

        } else {
            None
        }
    }

    /// Get the next table or create one if it does not exist
    pub fn next_table_create<A>(&mut self,
                                index: usize,
                                allocator: &mut A)
                                -> &mut Table<L::NextLevel>
        where A: FrameAllocator
    {
        if self.next_table(index).is_none() {
            assert!(!self.entries[index].flags().contains(HUGE_PAGE),
                    "mapping code does not support huge pages");
            let frame = allocator.allocate_frame().expect("no frames available");
            self.entries[index].set(frame, PRESENT | WRITABLE);

            unsafe {
                use vga_buffer::print_error;
                print_error(format_args!("Curr table: 0x{:x}", self as *mut _ as usize));
                print_error(format_args!("Entries[{0}]: {1:?} at 0x{2:x}",
                                         index,
                                         self.entries[index],
                                         &self.entries[index] as *const _ as usize));
                {
                    let a = self.next_table_mut(index).unwrap();
                    print_error(format_args!("next table: 0x{:x}", a as *mut _ as usize));
                    a.zero();
                }
                print_error(format_args!("Entries[{0}]: {1:?} at 0x{2:x}",
                                         index,
                                         self.entries[index],
                                         &self.entries[index] as *const _ as usize));
            }
        }
        let val = self.next_table_mut(index);//.unwrap()
        assert!(!val.is_none());
        val.unwrap()
    }
}
