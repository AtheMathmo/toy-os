#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![feature(asm)]

//extern crate rlibc;

#[no_mangle] // ensure that this symbol is called `rust_main` in the output
pub extern fn rust_main() {
	let x : *mut u8 = 0xb8000 as *mut u8;

	//let chars = [ 0x58, 0x59, 0x60 ];
	unsafe {
		// Set the value to 'X' (hex 0x58)
		let y = &mut *(x);

		// for c in chars.iter() {
		// 	let z = 0x40;
		// }

		*y = 0x58;

	}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }

// If these are commented it will work
// Otherwise triple seg
#[no_mangle]
pub extern fn fmod() -> ! { 
	unsafe {
		asm!("int3");
	}
	loop {}
}
#[no_mangle]
pub extern fn fmodf() -> ! { 
	unsafe {
		asm!("int3");
	}
	loop {}
}