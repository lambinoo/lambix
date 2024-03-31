#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(asm_const)]

pub mod descriptors;
pub mod gdt;
pub mod idt;
pub mod paging;
