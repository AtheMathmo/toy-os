/// Using a bitmap
/// First page will be allocated after the kernel

use super::{kernel_memory_end, Frame, FrameAllocator};

const BIT_MAP_TABLE_ROWS: usize = 32;

pub struct BitMapFrameAllocator {
    paging_start: u64,
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
        let index_in_row = frame.number % 8;
        // Turn off the bit at the index
        self.bit_map[frame.number / 8] &= !(1 << index_in_row);
    }
}

impl BitMapFrameAllocator {
    pub fn new() -> Self {
        BitMapFrameAllocator { paging_start: kernel_memory_end(), bit_map: [0; BIT_MAP_TABLE_ROWS] }
    }

    fn is_frame_free(&self, frame: &Frame) -> bool {
        if frame.number < 8 {
            // If in the first simply shift first row and get bit
            (self.bit_map[0] >> frame.number) == 1
        } else {
            // Get row for that frame and shift the appropriate amount
            (self.bit_map[frame.number / 8] >> (frame.number % 8)) == 1
        }
    }
}
