#![feature(lang_items)]
#![feature(start)]
#![no_std]

#[no_mangle] // ensure that this symbol is called `rust_main` in the output
pub extern fn rust_main() {
	let x : *mut u8 = 0xb8000 as *mut u8;

	unsafe {
		// Set the value to 'X' (hex 0x58)
		let y = &mut *(x);
		*y = 0x58;
	}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }