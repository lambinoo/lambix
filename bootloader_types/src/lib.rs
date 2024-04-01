#![no_std]

use core::ptr::NonNull;

use core::ops::Range;
use multiboot2::BootInformation;

pub mod multiboot2;

#[derive(Debug)]
#[repr(C)]
pub struct KernelInformation {
    boot_info: u32,
    kernel_range: Range<u32>,
    initial_stack_range: Range<u32>,
}

impl KernelInformation {
    pub fn new(
        boot_info: &BootInformation,
        kernel_range: Range<*const u8>,
        stack_range: Range<*const u8>,
    ) -> KernelInformation {
        let stable_abi_range = |range: Range<*const u8>| Range {
            start: u32::try_from(range.start as usize).unwrap(),
            end: u32::try_from(range.end as usize).unwrap(),
        };

        Self {
            boot_info: u32::try_from(core::ptr::from_ref(boot_info) as usize).unwrap(),
            kernel_range: stable_abi_range(kernel_range),
            initial_stack_range: stable_abi_range(stack_range),
        }
    }

    pub fn boot_info(&self) -> &'static BootInformation {
        unsafe {
            NonNull::new(usize::try_from(self.boot_info).unwrap() as *mut _)
                .unwrap()
                .as_ref()
        }
    }
}
