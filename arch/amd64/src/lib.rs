#![no_std]
#![feature(asm_const)]
#![feature(naked_functions)]
#![feature(format_args_nl)]

pub mod descriptors;
pub mod gdt;
pub mod paging;

#[cfg(target_arch = "x86_64")]
#[macro_use]
pub mod idt;

#[macro_use]
pub mod serial_print;
