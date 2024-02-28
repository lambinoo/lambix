#![no_std]
#![no_main]
#![feature(format_args_nl)]

use crate::multiboot::{BootInformation, MemoryInfo, Tag};

#[macro_use]
mod serial_print;
mod panic;

mod bootstrap;
mod multiboot;

extern "C" {
    static kernel_start_addr: u8;
}

fn get_kernel_elf_start() -> *const u8 {
    unsafe { &kernel_start_addr as *const _ }
}

#[no_mangle]
pub extern "C" fn boot_start(
    multiboot_magic: u32,
    multiboot_header_ptr: *mut BootInformation,
) -> ! {
    let boot_info = unsafe { BootInformation::from_ptr(multiboot_header_ptr, multiboot_magic) }
        .expect("Failed to get boot information from the bootloader");

    let start = get_kernel_elf_start();
    let elf_header = unsafe { core::slice::from_raw_parts(start, 4) };

    for tag in boot_info.iter() {
        match tag {
            Tag::MemoryMap(mem) => mem
                .iter()
                .map(|range| range)
                .for_each(|m| println!("{:?}", m)),
            _ => {}
        }
    }

    unreachable!()
}
