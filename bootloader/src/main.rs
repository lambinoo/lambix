#![no_std]
#![no_main]
#![feature(format_args_nl)]
#![feature(pointer_is_aligned_to)]

use core::mem::size_of;

use arch_amd64::descriptors::CodeDescriptor;
use arch_amd64::descriptors::DataDescriptor;
use arch_amd64::gdt::GlobalDescriptorTable;
use elf::abi::R_X86_64_RELATIVE;
use elf::abi::SHT_RELA;

#[macro_use]
extern crate arch_amd64;

mod panic;

mod bootstrap;
mod kernel_loader;
mod paging;

use bootloader::multiboot2::BootInformation;
use bootloader::KernelInformation;

#[no_mangle]
pub extern "C" fn boot_start(
    multiboot_magic: u32,
    multiboot_header_ptr: *mut BootInformation,
) -> ! {
    println!("Found boot information at {multiboot_header_ptr:?}");

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

    let alignment = segments
        .iter()
        .filter(|s| s.p_type == elf::abi::PT_LOAD)
        .filter_map(|s| usize::try_from(s.p_align).ok())
        .chain(core::iter::once(4096))
        .max()
        .expect("Cannot discovery needed alignment for embedded ELF kernel");
    println!("Need {} bytes to unpack the kernel", needed_memory);

    let allocated_memory = paging::setup_kernel_memory(boot_info, needed_memory, alignment);

    for segment in segments.iter() {
        if segment.p_type == elf::abi::PT_LOAD {
            let segment_data = elf
                .segment_data(&segment)
                .expect("Failed to extract data from ELF payload");

            let vaddr = usize::try_from(segment.p_vaddr).unwrap();
            let filesz = usize::try_from(segment.p_filesz).unwrap();
            let memsz = usize::try_from(segment.p_memsz).unwrap();

            allocated_memory.kernel[vaddr..vaddr + filesz].copy_from_slice(segment_data);
            if memsz >= filesz {
                allocated_memory.kernel[vaddr + filesz..vaddr + memsz].fill(0);
            } else {
                panic!("Invalid ELF header");
            }
        }
    }

    for section in elf.section_headers().unwrap() {
        if section.sh_type == SHT_RELA {
            let load_address = allocated_memory.kernel_virt.start as i64;
            for rela in elf.section_data_as_relas(&section).unwrap() {
                match rela.r_type {
                    R_X86_64_RELATIVE => {
                        let offset: usize = usize::try_from(rela.r_offset).unwrap();
                        let computed_value = load_address + i64::try_from(rela.r_addend).unwrap();

                        let relocated_data =
                            &mut allocated_memory.kernel[offset..offset + size_of::<u64>()];
                        relocated_data.copy_from_slice(&u64::to_ne_bytes(computed_value as u64));
                    }

                    _ => panic!("Unhandled relocation type {:x}, aborting.", rela.r_type),
                }
            }
        }
    }

    println!(
        "Kernel has been extracted at physical address {:?}",
        allocated_memory.kernel.as_ptr()
    );

    let kernel_information = KernelInformation::new(
        boot_info,
        allocated_memory.kernel.as_ptr_range(),
        allocated_memory.stack.as_ptr_range(),
    );

    kernel_loader::exec_long_mode(&kernel_information, &allocated_memory, elf.ehdr.e_entry);
}

pub static EARLY_GDT: GlobalDescriptorTable = GlobalDescriptorTable::new(
    CodeDescriptor::new(0, 0xfffff).readable(),
    DataDescriptor::new(0, 0xfffff).writable(),
);
