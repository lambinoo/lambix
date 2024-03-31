use core::{
    ops::{Range, RangeBounds},
    ptr::addr_of,
};

use crate::{
    multiboot::{MemoryInfo, MemoryInfoIter},
    EARLY_GDT,
};

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
        .filter(|range| !range.contains(&core::ptr::null()))
        .filter_map(|range| unsafe {
            Some(core::slice::from_raw_parts_mut(
                range.start as *mut u8,
                usize::try_from(range.start.offset_from(range.end)).ok()?,
            ))
        })
        .flat_map(|buffer| buffer.chunks_exact_mut(needed_memory))
        .filter(move |chunk| {
            let range = chunk.as_ptr_range();
            !core::iter::once(&exclude_range)
                .chain(excluded_ranges.iter())
                .any(|excluded| range.contains(&excluded.start) || range.contains(&excluded.end))
        })
        .next()
}

pub fn exec_long_mode(entrypoint: *const u8, stack: &mut [u8]) -> ! {
    let range = stack.as_ptr_range();

    unsafe {
        core::arch::asm!(
            "mov eax, cr4",
            "bts eax, 5",
            "bts eax, 7",
            "mov cr4, eax",
            "mov ecx, 0xc0000080",
            "rdmsr",
            "bts eax, 8",
            "wrmsr",
            "mov eax, cr0",
            "bts eax, 31",
            "mov cr0, eax",
            out("edx") _,
            out("ecx") _,
            out("eax") _,
        );

        core::arch::asm!(
            "mov {s}, %esp",
            "xor %ebp, %ebp",
            "ljmp $0x18,$5f",
            "5: jmp *{entry}",
            s = in(reg) range.end,
            entry = in(reg) entrypoint,
            options(att_syntax, noreturn)
        )
    }
}
