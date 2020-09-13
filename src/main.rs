#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern crate rlibc;
mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    minimalos::init();
    print!(">>> ");
    minimalos::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    minimalos::hlt_loop();
}
