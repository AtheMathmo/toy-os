#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![feature(asm)]
#![feature(const_fn)]
#![feature(unique)]

extern crate rlibc;
extern crate spin;

mod vga_buffer;

#[start]
#[export_name ="_start"]
pub extern fn rust_main() {
	vga_buffer::print_something();

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
