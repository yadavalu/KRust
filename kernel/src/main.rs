#![no_std]
#![no_main]

extern crate vga;
use vga::{writeline, error};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	// FIXME: This causes E0433 (could not find `vga` in `$crate`)
	error!("{}", _info);
	loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
	// FIXME: This causes E0433 (could not find `vga` in `$crate`)
    writeline!("Loading KRust kernel version {} ...", 0.1);
	loop {}
}
