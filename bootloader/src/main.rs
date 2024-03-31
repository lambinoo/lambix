#![no_std]
#![no_main]
#![feature(format_args_nl)]

use core::{mem::size_of, ptr::addr_of};

use arch_amd64::{
    descriptors::{CodeDescriptor, DataDescriptor},
    gdt::GlobalDescriptorTable,
};
use elf::abi::{DT_RELA, R_X86_64_RELATIVE, SHT_RELA};

use crate::{
    identity_paging::setup_identity_paging,
    multiboot::{BootInformation, MemoryInfo},
};

#[macro_use]
mod serial_print;
mod panic;

mod bootstrap;
mod identity_paging;
mod kernel_loader;
mod multiboot;

#[no_mangle]
pub extern "C" fn boot_start(
    multiboot_magic: u32,
    multiboot_header_ptr: *mut BootInformation,
) -> ! {
    EARLY_GDT.load_gdt();
    EARLY_GDT.set_protected_mode();
    println!("GDT has been applied");

    let boot_info = unsafe { BootInformation::from_ptr(multiboot_header_ptr, multiboot_magic) }
        .expect("Failed to get boot information from the bootloader");

    let elf_header =
        kernel_loader::get_embedded_kernel().expect("No embedded kernel available, aborting.");

    let elf = elf::ElfBytes::<elf::endian::LittleEndian>::minimal_parse(elf_header)
        .expect("Bad ELF payload");

    let segments = elf
        .segments()
        .expect("Invalid ELF payload, no segments found");

    let needed_memory = segments
        .iter()
        .filter(|s| s.p_type == elf::abi::PT_LOAD)
        .map(|s| s.p_vaddr + s.p_memsz)
        .max()
        .map(|size| usize::try_from(size).ok())
        .flatten()
        .expect("Can't fit ELF payload in memory");

    let mem_map = boot_info.memory_map().unwrap();
    let kernel_destination =
        kernel_loader::get_available_memory(mem_map.iter(), needed_memory, &[])
            .expect("Could not find a memory area suitable for unpacking the kernel");

    for segment in segments.iter() {
        if segment.p_type == elf::abi::PT_LOAD {
            let segment_data = elf
                .segment_data(&segment)
                .expect("Failed to extract data from ELF payload");

            let vaddr = usize::try_from(segment.p_vaddr).unwrap();
            let filesz = usize::try_from(segment.p_filesz).unwrap();
            let memsz = usize::try_from(segment.p_memsz).unwrap();

            kernel_destination[vaddr..vaddr + memsz].fill(0);
            kernel_destination[vaddr..vaddr + filesz].copy_from_slice(segment_data);
        }
    }

    for section in elf.section_headers().unwrap() {
        if section.sh_type == SHT_RELA {
            let load_address = kernel_destination.as_ptr() as i64;

            for rela in elf.section_data_as_relas(&section).unwrap() {
                match rela.r_type {
                    R_X86_64_RELATIVE => {
                        let offset: usize = usize::try_from(rela.r_offset).unwrap();
                        let computed_value = load_address + i64::try_from(rela.r_addend).unwrap();

                        let relocated_data =
                            &mut kernel_destination[offset..offset + size_of::<u64>()];
                        relocated_data.copy_from_slice(&u64::to_ne_bytes(computed_value as u64));
                    }

                    _ => panic!("Unhandled relocation type {:x}, aborting.", rela.r_type),
                }
            }
        }
    }

    let stack = kernel_loader::get_available_memory(
        mem_map.iter(),
        4096 * 4,
        &[kernel_destination.as_ptr_range()],
    )
    .expect("Could not allocate a stack for the kernel");

    println!(
        "Kernel has been extracted at {:?}",
        kernel_destination.as_ptr()
    );

    let ventry = usize::try_from(elf.ehdr.e_entry).unwrap();
    let entrypoint = kernel_destination[ventry..ventry].as_ptr() as *const u8;
    println!(
        "Entrypoint at {:?}, stack at {:?}",
        entrypoint,
        stack.as_ptr_range()
    );

    print!("Setting up identity paging.. ");
    setup_identity_paging();
    println!("done.");

    print!("Jumping to extracted kernel.. ");
    kernel_loader::exec_long_mode(entrypoint, stack);
}

pub static EARLY_GDT: GlobalDescriptorTable = GlobalDescriptorTable::new(
    CodeDescriptor::new(0, 0xfffff).readable(),
    DataDescriptor::new(0, 0xfffff).writable(),
);
