#![no_std]
#![no_main]
#![feature(format_args_nl)]

use core::ptr::NonNull;

use elf::{endian::LittleEndian, ElfBytes};

use crate::multiboot::{BootInformation, Tag};

#[macro_use]
mod serial_print;
mod panic;

mod bootstrap;
mod multiboot;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct KernelHeader {
    magic: u32,
    len: u32
}

extern "C" {
    static lambix_kernel_header: KernelHeader;
    static lambix_kernel_start: u8;
}

fn get_embedded_kernel() -> Option<&'static [u8]> {
    let header = unsafe { lambix_kernel_header };
    if header.magic == u32::from_le_bytes(b"lamb".clone()) {
        let kernel_size = usize::try_from(header.len).ok()?;
        let slice = unsafe { core::slice::from_raw_parts(&lambix_kernel_start as *const u8, kernel_size) };
        Some(slice)
    } else {
        None
    }
}


#[no_mangle]
pub extern "C" fn boot_start(
    multiboot_magic: u32,
    multiboot_header_ptr: *mut BootInformation,
) -> ! {
    let boot_info = unsafe { BootInformation::from_ptr(multiboot_header_ptr, multiboot_magic) }
        .expect("Failed to get boot information from the bootloader");

    let elf_header = get_embedded_kernel().expect("No embedded kernel available, aborting.");
    let elf = ElfBytes::<LittleEndian>::minimal_parse(elf_header).expect("Bad ELF payload");

    let headers = elf.section_headers().unwrap();
    let (headers, names) = elf.section_headers_with_strtab().unwrap();

    for header in headers.unwrap().iter() {
        println!("{:?}, {:?}", header, names.unwrap().get(header.sh_name as usize));
    }

    for tag in boot_info.iter() {
        match tag {
            Tag::MemoryMap(mem) => mem
                .iter()
                .map(|range| range)
                .for_each(|m| println!("{:#?}", m)),
            _ => {}
        }
    }

    unreachable!()
}
