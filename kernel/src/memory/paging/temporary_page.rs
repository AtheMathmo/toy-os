use super::{ActivePageTable, VirtualAddress};
use super::Page;
use super::table::{Table, Level1};

use memory::{Frame, FrameAllocator};

pub struct TemporaryPage {
    page: Page,
    allocator: TempAllocator,
}

impl TemporaryPage {
    pub fn map(&mut self, frame: Frame, active_table: &mut ActivePageTable) -> VirtualAddress {
        use super::entry::WRITABLE;

        assert!(active_table.translate_page(self.page).is_none(),
                "Temporary page is already mapped.");

        active_table.map_to(self.page, frame, WRITABLE, &mut self.allocator);
        self.page.start_address()
    }

    pub fn unmap(&mut self, active_table: &mut ActivePageTable) {
        active_table.unmap(self.page, &mut self.allocator);
    }

    pub fn map_table_frame(&mut self,
                           frame: Frame,
                           active_table: &mut ActivePageTable)
                           -> &mut Table<Level1> {
        unsafe { &mut *(self.map(frame, active_table) as *mut Table<Level1>) }
    }

    pub fn new<A: FrameAllocator>(page: Page, allocator: &mut A) -> Self {
        TemporaryPage {
            page: page,
            allocator: TempAllocator::new(allocator),
        }
    }
}

struct TempAllocator([Option<Frame>; 3]);

impl FrameAllocator for TempAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        for frame_option in &mut self.0 {
            if frame_option.is_some() {
                return frame_option.take();
            }
        }
        None
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        for frame_option in &mut self.0 {
            if frame_option.is_none() {
                *frame_option = Some(frame);
                return;
            }
        }

        panic!("The TempAllocator can only hold 3 frames.");
    }
}

impl TempAllocator {
    fn new<A: FrameAllocator>(allocator: &mut A) -> Self {
        let mut f = || allocator.allocate_frame();
        let frames = [f(), f(), f()];
        TempAllocator(frames)
    }
}
