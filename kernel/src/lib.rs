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

	let x = 0xb8000 as *mut u8;

	unsafe {
		let y = &mut *x;
		*y = 0x58;
	}
	loop {}
	// use core::fmt::Write;

	// vga_buffer::WRITER.lock().clear_screen();
	// vga_buffer::WRITER.lock().write_str("Now running the rust kernel!");
	// // If this is commented it will work.. Otherwise broken
	// // println!("test");

	//loop {}
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
