use core::sync::atomic::{AtomicU64, Ordering};

const PAGING_TABLE_SIZE: usize = 512;

#[repr(C, align(4096))]
pub struct PagingTable {
    inner: [AtomicU64; PAGING_TABLE_SIZE],
}

impl PagingTable {
    const DEFAULT: AtomicU64 = AtomicU64::new(0);
    pub const MAX_INDEX: usize = PAGING_TABLE_SIZE;

    pub const fn new() -> Self {
        Self {
            inner: [Self::DEFAULT; PAGING_TABLE_SIZE],
        }
    }

    pub fn store(&self, idx: usize, value: u64) {
        self.inner[idx].store(value, Ordering::Relaxed);
    }

    pub fn fetch(&self, idx: usize) -> u64 {
        self.inner[idx].load(Ordering::Acquire)
    }
}
