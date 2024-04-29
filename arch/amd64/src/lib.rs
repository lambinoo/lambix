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

#[cfg(target_arch = "x86_64")]
pub mod apic;

#[macro_use]
pub mod serial_print;

#[derive(Default, Debug)]
pub struct MSR {
    low: u32,
    high: u32,
}

impl MSR {
    /// Read a MSR register
    /// This is a privileged operation, and may not be called from a user context
    /// (not that we have any :D)
    pub fn read(register: u32) -> MSR {
        let mut msr = MSR::default();
        unsafe {
            core::arch::asm!(
                "rdmsr",
                in("ecx") register,
                out("edx") msr.high,
                out("eax") msr.low,
            )
        };
        msr
    }

    /// Write to a MSR register
    /// This is a privileged operation, and may not be called from a user context
    /// (not that we have any :D)
    pub fn write(register: u32, msr: MSR) {
        unsafe {
            core::arch::asm!(
                "wrmsr",
                in("ecx") register,
                in("edx") msr.high,
                in("eax") msr.low,
            )
        };
    }
}

#[cfg(target_arch = "x86_64")]
pub mod cpuid;
