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

#[start]
#[export_name ="_start"]
pub extern fn rust_main() {
	use core::fmt::Write;

	vga_buffer::WRITER.lock().clear_screen();
	vga_buffer::WRITER.lock().write_str("Now running the rust kernel!");
	// vga_buffer::WRITER.lock().write_byte(b'H');
	// vga_buffer::WRITER.lock().write_byte(b'e');
	// vga_buffer::WRITER.lock().write_byte(b'l');
	// vga_buffer::WRITER.lock().write_byte(b'l');
	// vga_buffer::WRITER.lock().write_byte(b'o');
	// vga_buffer::WRITER.lock().write_byte(b'\n');
	// vga_buffer::WRITER.lock().write_byte(b'W');

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
