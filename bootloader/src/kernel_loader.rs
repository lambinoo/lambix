use core::ops::Range;
use core::ptr::addr_of;

use bootloader::multiboot2::MemoryInfo;
use bootloader::multiboot2::MemoryInfoIter;
use bootloader::KernelInformation;

use crate::paging::KernelMemoryAlloc;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct KernelHeader {
    magic: u32,
    len: u32,
}

extern "C" {
    static lambix_kernel_header: KernelHeader;
    static lambix_kernel_start: u8;
    static bootloader_start: u8;
}

/// Returns the embedded kernel header, if the header is correct
pub fn get_embedded_kernel() -> Option<&'static [u8]> {
    let header = unsafe { lambix_kernel_header };
    if header.magic == u32::from_le_bytes(b"lamb".clone()) {
        let kernel_size = usize::try_from(header.len).ok()?;
        let slice =
            unsafe { core::slice::from_raw_parts(&lambix_kernel_start as *const u8, kernel_size) };
        Some(slice)
    } else {
        None
    }
}

pub fn get_available_memory<'a>(
    mem_map: MemoryInfoIter<'a>,
    needed_memory: usize,
    align: usize,
    excluded_ranges: &[Range<*const u8>],
) -> Option<&'static mut [u8]> {
    let kernel = get_embedded_kernel().expect("Failed to get embedded kernel");

    let exclude_range = Range {
        start: unsafe { addr_of!(bootloader_start) },
        end: kernel.as_ptr_range().end,
    };

    mem_map
        .filter(|m| matches!(m, MemoryInfo::Available(_)))
        .filter_map(|m| m.as_ptr_range::<u8>())
        .flat_map(|full_range| {
            let mut aligned_start = full_range
                .start
                .wrapping_add(full_range.start.align_offset(align));

            core::iter::from_fn(move || {
                let mut end = aligned_start.wrapping_add(needed_memory);
                end = end.wrapping_add(end.align_offset(align));

                let range = Range {
                    start: aligned_start,
                    end,
                };
                if full_range.contains(&range.start) && full_range.contains(&range.end) {
                    aligned_start = end;
                    Some(range)
                } else {
                    None
                }
            })
        })
        .filter(|range| !range.contains(&core::ptr::null()))
        .filter(move |range| {
            !core::iter::once(&exclude_range)
                .chain(excluded_ranges.iter())
                .any(|excluded| range.contains(&excluded.start) || range.contains(&excluded.end))
        })
        .map(|range| unsafe {
            core::slice::from_raw_parts_mut(
                range.start as _,
                range.end as usize - range.start as usize,
            )
        })
        .next()
}

pub fn exec_long_mode(
    kernel_info: &KernelInformation,
    kernel: &KernelMemoryAlloc,
    entry_virt: u64,
) -> ! {
    let stack_pointer = kernel.stack_virt.end;
    let upper_entrypoint = kernel.kernel_virt.start + entry_virt;

    println!(
        "Kernel will be executed at 0x{:x} with stack 0x{:x}",
        upper_entrypoint, stack_pointer
    );
    print!("Jumping to extracted kernel.. ");

    unsafe {
        core::arch::asm!(
            // Enable CR4.PAE and global pages
            "mov eax, cr4",
            "bts eax, 5",
            "bts eax, 7",
            "mov cr4, eax",

            // Enable long mode
            "mov ecx, 0xc0000080",
            "rdmsr",
            "bts eax, 8",
            "wrmsr",

            // Enable paging
            "mov eax, cr0",
            "bts eax, 31",
            "mov cr0, eax",
            out("edx") _,
            out("ecx") _,
            out("eax") _,
        );

        core::arch::asm!(
            "ljmp $0x18,$5f",
            ".code64",
            // Setup kernel stack
            "5: xorq %rbp, %rbp",
            "movq (%edx), %rsp",
            // Load kernel jump address
            "movq (%ecx), %rcx",
            // Jump to high address of the kernel
            "jmpq *%rcx",
            ".code32",
            in("edi") kernel_info,
            in("ecx") &upper_entrypoint,
            in("edx") &stack_pointer,
            options(att_syntax, noreturn)
        )
    }
}
