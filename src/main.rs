#![no_std]
#![no_main]
#![allow(arithmetic_overflow)]

use core::panic::PanicInfo;
extern crate rlibc;
mod vga;

// static HELLO: &[u8] = b"hello world";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    /*
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // https://en.wikipedia.org/wiki/VGA_text_mode#Text_buffer
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xF; // white
            *vga_buffer.offset(i as isize * 2 + 2) = 0xF
        }
    }
    */
    let mut writer = vga::Writer {
        column_position: 0,
        color_code: vga::ColorCode::new(vga::Color::Yellow, vga::Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut vga::Buffer) },
    };

    writer.write_string("peepee");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
