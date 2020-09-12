#![no_std]
#![no_main]
use core::fmt::Write;
use core::panic::PanicInfo;
extern crate rlibc;
mod vga;

#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = vga::Writer {
        column_position: 0,
        color_code: vga::ColorCode::new(vga::Color::Yellow, vga::Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut vga::Buffer) },
    };

    panic!("error message here");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut writer = vga::Writer {
        column_position: 0,
        color_code: vga::ColorCode::new(vga::Color::Yellow, vga::Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut vga::Buffer) },
    };
    write!(writer, "{}", info).unwrap();

    loop {}
}
