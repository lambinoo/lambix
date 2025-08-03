#![no_std]

pub struct MemoryBlocks {}

pub struct MemoryBlock {
    start: *const (),
    size: usize,
}
