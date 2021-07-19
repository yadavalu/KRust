#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	errorln!("{}", _info);
	loop {}
}

mod vga;
use core::fmt::Write;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Loading KRust kernel version {}", 0.1);
	loop {}
}
