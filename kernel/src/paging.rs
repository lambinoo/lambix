use core::ops::Range;

use arch_amd64::paging::PagingTable;
use bootloader::KernelInformation;

const EARLY_PAGE_TABLE_COUNT: usize = 64;
const DEFAULT_PAGING_TABLE: PagingTable = PagingTable::new();

const PRESENT_FLAG: usize = 1;
const RW_FLAG: usize = 1 << 1;
const PAGE_SIZE_FLAG: usize = 1 << 7;
const NX_FLAG: usize = 1 << 63;

pub static EARLY_PAGE_TABLES: [PagingTable; EARLY_PAGE_TABLE_COUNT] =
    [DEFAULT_PAGING_TABLE; EARLY_PAGE_TABLE_COUNT];

fn initialize_early_kernel_memory_impl(kernel_info: &KernelInformation) {}

/// We take a pointer instead of a reference, as this will invalidate the kernel information
/// pointer itself
pub unsafe fn initialize_early_kernel_memory(kernel_info_ptr: *mut KernelInformation) {
    initialize_early_kernel_memory_impl(kernel_info_ptr.as_ref().unwrap())
}

pub fn register_memory(range: Range<*const ()>) {}
