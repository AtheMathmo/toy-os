pub mod bitmap_frame_allocator;
pub mod area_frame_allocator;
mod paging;

pub use self::paging::remap_the_kernel;

pub const PAGE_SIZE: usize = 4096;

enum Void {}

extern "C" {
    static KERNEL_START: Void;
    static KERNEL_END: Void;
}

/// Returns the address of the start of kernel allocated memory.
pub fn kernel_memory_start() -> usize {
    &KERNEL_START as *const _ as usize
}

/// Returns the address of the end of kernel allocated memory.
pub fn kernel_memory_end() -> usize {
    &KERNEL_END as *const _ as usize
}

/// Reads the size of the bootloader
/// from the bootloader information section
///
/// # Safety
///
/// The address given to the function must be a valid pointer
/// to the start of the bootloader information header. As specified
/// in section 3.4. of [the multiboot specification 1.6](http://nongnu.askapache.com/grub/phcoder/multiboot.pdf).
pub unsafe fn bootloader_info_memory_limits(bootloader_info_address: usize) -> (usize, usize) {
    let info_size = *(bootloader_info_address as *const u32);
    (bootloader_info_address,
     (bootloader_info_address + info_size as usize))
}

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
        // kernel_memory_end() + PAGE_SIZE - (kernel_memory_end() % PAGE_SIZE) +
        self.number * PAGE_SIZE
    }

    /// Clone the frame.
    fn clone(&self) -> Frame {
        Frame { number: self.number }
    }

    fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end,
        }
    }
}

struct FrameIter {
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        } else {
            None
        }
    }
}

/// Trait for allocating and deallocating page frames
pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}
