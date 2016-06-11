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
mod vga_buffer;
mod interrupts;
mod memory;
mod x86;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_info_address: usize) {
    vga_buffer::clear_screen();
    println!("Now running the rust kernel!");
    println!("This will work...");

    interrupts::init();

    println!("Kernel starts at: {0}\nKernel ends at: {1}\nTotal size: {2}",
             memory::kernel_memory_start(),
             memory::kernel_memory_end(),
             memory::kernel_memory_end() - memory::kernel_memory_start());

    unsafe {
	    let (b_start, b_end) = memory::bootloader_info_memory_limits(multiboot_info_address);
	    println!("Bootloader info starts at: {0}\nBootloader info ends at: {1}\nTotal size: {2}",
	    		b_start,
	    		b_end,
	    		b_end - b_start);
	}

	use memory::FrameAllocator;
	let mut allocator = memory::bitmap_frame_allocator::BitMapFrameAllocator::new();

	for _ in 0..20 {
		allocator.allocate_frame();
	}


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
