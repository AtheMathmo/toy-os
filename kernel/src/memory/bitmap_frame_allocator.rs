/// Using a bitmap
/// First page will be allocated after the kernel

use super::{Frame, FrameAllocator};
use core::u8;

// 512bytes * 8 = 4096 frames
const BIT_MAP_TABLE_ROWS: usize = 512;

pub struct BitMapFrameAllocator {
    kernel_start_frame: Frame,
    kernel_end_frame: Frame,
    bit_map: [u8; BIT_MAP_TABLE_ROWS],
}

impl FrameAllocator for BitMapFrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        // Right now we just do an O(N) check
        for i in 0..BIT_MAP_TABLE_ROWS * 8 {
            let frame = Frame { number: i };
            if self.is_frame_free(&frame) {
                // Allocate the frame and return it
                let index_in_row = frame.number % 8;
                self.bit_map[frame.number / 8] |= !(1 << index_in_row);
                return Some(frame);
            }
        }
        None
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        if frame >= self.kernel_start_frame || frame <= self.kernel_end_frame {
            panic!("Frame cannot be deallocated as it is used by the kernel.");
        }

        let index_in_row = frame.number % 8;
        // Turn off the bit at the index
        self.bit_map[frame.number / 8] &= !(1 << index_in_row);
    }
}

impl BitMapFrameAllocator {
    /// Creates a new BitMapFrameAllocator
    ///
    /// The allocator will reserve the memory within the kernel
    pub fn new(kernel_start: usize, kernel_end: usize) -> Self {
        let kernel_start_frame = Frame::containing_address(kernel_start);
        let kernel_end_frame = Frame::containing_address(kernel_end);

        let mut bit_map = [0; BIT_MAP_TABLE_ROWS];
        let reserved_frame_start = kernel_start_frame.number / 8;
        let reserved_frame_end = kernel_end_frame.number / 8;
        let reserved_start_index = kernel_start_frame.number % 8;
        let reserved_end_index = kernel_end_frame.number % 8;

        // Reserve all bits on the left of the kernel start frame
        bit_map[reserved_frame_start] |=  u8::MAX << reserved_start_index;

        // Reserve all bits on the right of the kernel end frame
        bit_map[reserved_frame_start] |=  !(u8::MAX << reserved_end_index);
        
        // Set all frames inside kernel memory to used
        for i in reserved_frame_start + 1 .. reserved_frame_end - 1 {
            bit_map[i] = u8::MAX;
        }


        BitMapFrameAllocator {
            kernel_start_frame: kernel_start_frame,
            kernel_end_frame: kernel_end_frame,
            bit_map: [0; BIT_MAP_TABLE_ROWS],
        }
    }

    pub fn is_frame_free(&self, frame: &Frame) -> bool {
        if frame.number < 8 {
            // If in the first simply shift first row and get bit
            ((self.bit_map[0] >> frame.number) & 1) == 0
        } else {
            // Get row for that frame and shift the appropriate amount
            ((self.bit_map[frame.number / 8] >> (frame.number % 8)) & 1) == 0
        }
    }
}
