use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering;

const PAGING_TABLE_SIZE: usize = 512;

#[repr(C, align(4096))]
pub struct PagingTable {
    inner: [AtomicU64; PAGING_TABLE_SIZE],
}

impl PagingTable {
    pub const MAX_INDEX: usize = PAGING_TABLE_SIZE - 1;
    pub const MAX_INDEX_U64: u64 = PAGING_TABLE_SIZE as u64 - 1;

    pub const fn new() -> Self {
        // We are just using this, so we can initialize this function in a const environment
        #[allow(clippy::declare_interior_mutable_const)]
        const DEFAULT: AtomicU64 = AtomicU64::new(0);

        Self {
            inner: [DEFAULT; PAGING_TABLE_SIZE],
        }
    }

    pub fn store(&self, idx: usize, value: u64) {
        self.inner[idx].store(value, Ordering::Relaxed);
    }

    pub fn fetch(&self, idx: usize) -> u64 {
        self.inner[idx].load(Ordering::Acquire)
    }
}

impl Default for PagingTable {
    fn default() -> Self {
        Self::new()
    }
}
