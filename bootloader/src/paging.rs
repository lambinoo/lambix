use core::cmp::max;

use arch_amd64::paging::PagingTable;
use bootloader_types::multiboot2::BootInformation;

use crate::kernel_loader::get_available_memory;

const ALIGN_2MB: usize = 4096 * 512;

const STACK_TOP_INDEX: usize = 510;
const KERNEL_TOP_INDEX: usize = 511;

const EMPTY_TABLE: PagingTable = PagingTable::new();
static PAGING_TABLES: [PagingTable; 5] = [EMPTY_TABLE; 5];

const PRESENT_FLAG: u64 = 1;
const RW_FLAG: u64 = 1 << 1;
const PAGE_SIZE_FLAG: u64 = 1 << 7;

pub struct KernelMemoryAlloc {
    pub kernel: &'static mut [u8],
    pub stack: &'static mut [u8],
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
        ALIGN_2MB,
        &[boot_info.as_bytes().as_ptr_range()],
    )
    .expect("Not enough memory to unpack the kernel");

    let stack_memory = get_available_memory(
        memory_map.iter(),
        kernel_size,
        max(kernel_align, ALIGN_2MB),
        &[
            boot_info.as_bytes().as_ptr_range(),
            kernel_memory.as_ptr_range(),
        ],
    )
    .expect("Not enough memory for a stack for the kernel");

    setup_identity_paging();
    setup_mapping(&PAGING_TABLES[3], KERNEL_TOP_INDEX, &kernel_memory);
    setup_mapping(&PAGING_TABLES[4], STACK_TOP_INDEX, &stack_memory);
    apply_paging();

    KernelMemoryAlloc {
        kernel: kernel_memory,
        stack: stack_memory,
    }
}

fn setup_mapping(pg_table_2mb: &PagingTable, intermediate_index: usize, memory: &[u8]) {
    let root_table: &PagingTable = &PAGING_TABLES[0];
    let intermediate_mapping = &PAGING_TABLES[2];

    let needed_allocations = memory.len().div_ceil(ALIGN_2MB);
    if needed_allocations >= PagingTable::MAX_INDEX {
        panic!("Kernel is too large, aborting");
    }

    for i in 0..needed_allocations {
        let address = memory.as_ptr() as usize + ALIGN_2MB * i;
        pg_table_2mb.store(i, address as u64 | PRESENT_FLAG | RW_FLAG | PAGE_SIZE_FLAG);
    }

    intermediate_mapping.store(
        intermediate_index,
        core::ptr::from_ref(pg_table_2mb) as u64 | PRESENT_FLAG | RW_FLAG,
    );

    root_table.store(
        PagingTable::MAX_INDEX,
        core::ptr::from_ref(intermediate_mapping) as u64 | PRESENT_FLAG | RW_FLAG,
    );
}

fn setup_identity_paging() {
    let root_table = &PAGING_TABLES[0];
    let ident_map = &PAGING_TABLES[1];

    for idx in 0..PagingTable::MAX_INDEX {
        let addr = (idx as u64) << 30;
        ident_map.store(idx, addr | PRESENT_FLAG | RW_FLAG | PAGE_SIZE_FLAG);
    }

    root_table.store(
        0,
        core::ptr::from_ref(ident_map) as u64 | PRESENT_FLAG | RW_FLAG,
    );
}

fn apply_paging() {
    let table = &PAGING_TABLES[0];
    unsafe {
        core::arch::asm!(
            "mov cr3, {t}",
            t = in(reg) table
        );
    }
}
