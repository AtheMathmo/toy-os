#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![feature(asm)]

extern crate rlibc;

#[start]
#[export_name ="_start"]
pub extern fn rust_main() {
	let buffer = (0xb8000 + 1988) as *mut _;

	let hello_world = b"Hello World!";
	let colour_byte = 0x1f; // White foreground, blue background

	let mut hello_coloured = [colour_byte; 24];
	// Fill with colour
	for (i, char_byte) in hello_world.into_iter().enumerate() {
		hello_coloured[i * 2] = *char_byte
	}

	// Write the message
	unsafe {
		*buffer = hello_coloured
	};

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
