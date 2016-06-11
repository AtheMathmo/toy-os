pub mod bitmap_frame_allocator;

enum Void {}

extern "C" {
    static KERNEL_START: Void;
    static KERNEL_END: Void;
}

pub fn kernel_memory_start() -> u64 {
    &KERNEL_START as *const _ as u64
}

pub fn kernel_memory_end() -> u64 {
    &KERNEL_END as *const _ as u64
}

/// Reads the size of the bootloader
/// from the bootloader information section
pub unsafe fn bootloader_info_memory_limits(bootloader_info_address: usize) -> (u64, u64) {
	let raw_ptr = bootloader_info_address as *const _;
	let info_size : u32;

	info_size = *raw_ptr;
	(raw_ptr as u64, raw_ptr.offset(info_size as isize) as u64)
}

pub const PAGE_SIZE: usize = 4096;

/// A page frame
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

/// Trait for allocating and deallocating page frames
pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}
