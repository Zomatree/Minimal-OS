#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use minimalos::memory;
use x86_64::{structures::paging::Page, VirtAddr};
extern crate rlibc;
mod vga;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    minimalos::hlt_loop();
}

entry_point!(kernal_main);

fn kernal_main(info: &'static BootInfo) -> ! {
    minimalos::init();

    let phys_mem_offset = VirtAddr::new(info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&info.memory_map) };

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    minimalos::hlt_loop();
}
