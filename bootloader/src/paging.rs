use core::{cmp::max, ops::Range};

use arch_amd64::paging::PagingTable;
use bootloader::multiboot2::BootInformation;

use crate::kernel_loader::get_available_memory;

const ALIGN_2MB: usize = 4096 * 512;

const STACK_TOP_INDEX: usize = 509;
const KERNEL_TOP_INDEX: usize = 511;

const EMPTY_TABLE: PagingTable = PagingTable::new();

static ROOT_TABLE: PagingTable = EMPTY_TABLE;
static HIGH_TABLE: PagingTable = EMPTY_TABLE;
static KERNEL_TABLE: PagingTable = EMPTY_TABLE;
static STACK_TABLE: PagingTable = EMPTY_TABLE;
static IDENTITY_TABLE: PagingTable = EMPTY_TABLE;

const PRESENT_FLAG: u64 = 1;
const RW_FLAG: u64 = 1 << 1;
const PAGE_SIZE_FLAG: u64 = 1 << 7;
const NX_FLAG: u64 = 1 << 63;

pub struct KernelMemoryAlloc {
    pub kernel: &'static mut [u8],
    pub stack: &'static mut [u8],
    pub kernel_virt: Range<u64>,
    pub stack_virt: Range<u64>,
}

///
pub fn setup_kernel_memory(
    boot_info: &BootInformation,
    kernel_size: usize,
    kernel_align: usize,
) -> KernelMemoryAlloc {
    let memory_map = boot_info.memory_map().expect("No memory map available");

    let kernel_memory = get_available_memory(
        memory_map.iter(),
        kernel_size,
        max(kernel_align, ALIGN_2MB),
        &[boot_info.as_bytes().as_ptr_range()],
    )
    .expect("Not enough memory to unpack the kernel");

    let stack_memory = get_available_memory(
        memory_map.iter(),
        kernel_align,
        ALIGN_2MB,
        &[
            boot_info.as_bytes().as_ptr_range(),
            kernel_memory.as_ptr_range(),
        ],
    )
    .expect("Not enough memory for a stack for the kernel");

    let kernel_address = setup_mapping(&KERNEL_TABLE, KERNEL_TOP_INDEX, &kernel_memory, RW_FLAG);
    let stack_address = setup_mapping(&STACK_TABLE, STACK_TOP_INDEX, &stack_memory, RW_FLAG);
    setup_identity_paging();
    apply_paging();

    let kernel_vrange =
        kernel_address..(kernel_address + u64::try_from(kernel_memory.len()).unwrap());
    let stack_vrange = stack_address..(stack_address + u64::try_from(stack_memory.len()).unwrap());

    KernelMemoryAlloc {
        kernel: kernel_memory,
        stack: stack_memory,
        kernel_virt: kernel_vrange,
        stack_virt: stack_vrange,
    }
}

fn setup_mapping(pg_table_2mb: &PagingTable, high_index: usize, memory: &[u8], flags: u64) -> u64 {
    let needed_allocations = memory.len().div_ceil(ALIGN_2MB);
    if needed_allocations >= PagingTable::MAX_INDEX {
        panic!("Kernel is too large, aborting");
    }

    for i in 0..needed_allocations {
        let address = (memory.as_ptr() as usize + ALIGN_2MB * i) as u64;
        pg_table_2mb.store(i, address | PRESENT_FLAG | PAGE_SIZE_FLAG | flags);
    }

    HIGH_TABLE.store(
        high_index,
        core::ptr::from_ref(pg_table_2mb) as u64 | PRESENT_FLAG | RW_FLAG,
    );

    ROOT_TABLE.store(
        PagingTable::MAX_INDEX,
        core::ptr::from_ref(&HIGH_TABLE) as u64 | PRESENT_FLAG | RW_FLAG,
    );

    u64::MAX << 39 | (high_index as u64) << 30
}

fn setup_identity_paging() {
    for idx in 0..PagingTable::MAX_INDEX {
        let addr = (idx as u64) << 30;
        IDENTITY_TABLE.store(idx, addr | PRESENT_FLAG | RW_FLAG | PAGE_SIZE_FLAG);
    }

    ROOT_TABLE.store(
        0,
        core::ptr::from_ref(&IDENTITY_TABLE) as u64 | PRESENT_FLAG | RW_FLAG,
    );
}

fn apply_paging() {
    let root_table = &ROOT_TABLE;
    unsafe {
        core::arch::asm!(
            "mov cr3, {t}",
            t = in(reg) root_table
        );
    }
}
