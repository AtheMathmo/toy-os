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
mod x86;

#[no_mangle]
pub extern fn rust_main() {
	vga_buffer::clear_screen();
	println!("Now running the rust kernel!");

	interrupts::init();

	println!("It did not crash!");

	loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
	use vga_buffer::Colour;
	vga_buffer::WRITER.lock().set_colour(Colour::Red, Colour::Black);
	println!("\n\nPanic in {} at line {}:", file, line);
	println!("      {}", fmt);
	loop {}
}

#[no_mangle]
pub extern fn fmod() {
	panic!("fmod is currently not implemented.");
}
#[no_mangle]
pub extern fn fmodf() { 
	panic!("fmodf is currently not implemented.");
}
