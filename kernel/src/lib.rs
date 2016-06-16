#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![feature(asm)]
#![feature(const_fn)]
#![feature(unique)]

extern crate rlibc;
extern crate spin;
extern crate bit_field;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;
mod interrupts;
mod memory;
mod x86;

use memory::bitmap_frame_allocator::BitMapFrameAllocator;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_info_address: usize) {
    vga_buffer::clear_screen();
    println!("Now running the rust kernel!");
    println!("My stuff:");

    interrupts::init();

    println!("Kernel starts at: 0x{0:x}\nKernel ends at: 0x{1:x}\nTotal size: 0x{2:x}",
             memory::kernel_memory_start(),
             memory::kernel_memory_end(),
             memory::kernel_memory_end() - memory::kernel_memory_start());

    unsafe {
	    let (b_start, b_end) = memory::bootloader_info_memory_limits(multiboot_info_address);
	    println!("Bootloader info starts at: 0x{0:x}\nBootloader info ends at: 0x{1:x}\nTotal size: 0x{2:x}",
	    		b_start,
	    		b_end,
	    		b_end - b_start);
	}
    let boot_info = unsafe { multiboot2::load(multiboot_info_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    let bootloader_info_memory_limits = unsafe { memory::bootloader_info_memory_limits(multiboot_info_address) };

    let mut frame_allocator = memory::area_frame_allocator::AreaFrameAllocator::new(
        memory::kernel_memory_start(), memory::kernel_memory_end(),
        bootloader_info_memory_limits.0, bootloader_info_memory_limits.1,
        memory_map_tag.memory_areas(),
    );

	//let mut frame_allocator = BitMapFrameAllocator::new(memory::kernel_memory_start(), memory::kernel_memory_end());
	memory::test_paging(&mut frame_allocator);

    // Produces a page fault
    unsafe { *(0xdeadbeef as *mut u64) = 42 };

    println!("It did not crash!");

    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    use vga_buffer::Colour;
    vga_buffer::WRITER.lock().set_colour(Colour::Red, Colour::Black);
    println!("\n\nPanic in {} at line {}:", file, line);
    println!("      {}", fmt);
    loop {}
}

#[no_mangle]
pub extern "C" fn fmod() {
    panic!("fmod is currently not implemented.");
}
#[no_mangle]
pub extern "C" fn fmodf() {
    panic!("fmodf is currently not implemented.");
}
