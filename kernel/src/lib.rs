#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![feature(asm)]
#![feature(const_fn)]
#![feature(unique)]

extern crate rlibc;
extern crate spin;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
	vga_buffer::clear_screen();
	println!("Now running the rust kernel!");

	use vga_buffer::Colour;
	vga_buffer::WRITER.lock().set_colour(Colour::Red, Colour::Black);
	print!("Nothing else to do...");

	loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }

// If these are commented it will work
// Otherwise triple seg
#[no_mangle]
pub extern fn fmod() -> ! { 
	loop {}
}
#[no_mangle]
pub extern fn fmodf() -> ! { 
	loop {}
}
