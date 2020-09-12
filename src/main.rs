#![no_std]
#![no_main]
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

    writer.writeln("peepee\npeepee2");
    writer.writeln("peepee\npeepee2");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
