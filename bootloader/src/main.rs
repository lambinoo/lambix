#![no_std]
#![no_main]

#![feature(format_args_nl)]

extern crate builtins_shared;

mod bootstrap;
mod multiboot;

#[macro_use]
mod serial_print;

extern "C" {
    static kernel_start_addr: u8;
    static kernel_end_addr: u8;
}

fn get_kernel_address() -> (*const u8, *const u8) {
    unsafe {
        (&kernel_start_addr as *const _, &kernel_end_addr as *const _)
    }
}

#[no_mangle]
pub extern "C" fn boot_start(multiboot_magic: u32, multiboot_header: u32) -> ! {
    println!("multiboot magic: 0x{:X}, header: 0x{:X}", multiboot_magic, multiboot_header);

    let (start, _) = get_kernel_address();
    let elf_header = unsafe {
        core::slice::from_raw_parts(start, 4)
    };

    println!("elf_header: {:?}", core::str::from_utf8(elf_header));

    loop {
        
    }
}
