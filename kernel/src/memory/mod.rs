pub mod bitmap_frame_allocator;
pub mod area_frame_allocator;
mod paging;

enum Void {}

extern "C" {
    static KERNEL_START: Void;
    static KERNEL_END: Void;
}

pub fn kernel_memory_start() -> usize {
    &KERNEL_START as *const _ as usize
}

pub fn kernel_memory_end() -> usize {
    &KERNEL_END as *const _ as usize
}

/// Reads the size of the bootloader
/// from the bootloader information section
pub unsafe fn bootloader_info_memory_limits(bootloader_info_address: usize) -> (usize, usize) {
	let info_size = *(bootloader_info_address as *const u32);
	(bootloader_info_address, (bootloader_info_address + info_size as usize))
}

pub const PAGE_SIZE: usize = 4096;

/// A page frame
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
	fn containing_address(address: usize) -> Frame {	
		Frame { number: address / PAGE_SIZE }
	}

	fn start_address(&self) -> self::paging::PhysicalAddress {
		// Start address is first page start after kernel end
		//kernel_memory_end() + PAGE_SIZE - (kernel_memory_end() % PAGE_SIZE) +
		self.number * PAGE_SIZE
	}
}

/// Trait for allocating and deallocating page frames
pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}


pub use self::paging::test_paging;