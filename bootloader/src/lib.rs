#![no_std]

use core::ptr::NonNull;

use core::ops::Range;
use multiboot2::BootInformation;

pub mod multiboot2;

pub type Address64 = [u8; 8];

#[derive(Debug)]
#[repr(C)]
pub struct KernelInformation {
    boot_info: u32,
    kernel_range: Range<u32>,
    initial_stack_range: Range<u32>,
    kernel_range_up: (Address64, Address64),
    initial_stack_range_up: (Address64, Address64),
}

impl KernelInformation {
    #[cfg(target_pointer_width = "32")]
    pub fn new(
        boot_info: &BootInformation,
        kernel_range: Range<*const u8>,
        stack_range: Range<*const u8>,
        kernel_range_up: Range<u64>,
        stack_range_up: Range<u64>,
    ) -> KernelInformation {
        let stable_abi_range = |range: Range<*const u8>| {
            u32::try_from(range.start as usize).unwrap()..u32::try_from(range.end as usize).unwrap()
        };

        Self {
            boot_info: u32::try_from(core::ptr::from_ref(boot_info) as usize).unwrap(),
            kernel_range: stable_abi_range(kernel_range),
            initial_stack_range: stable_abi_range(stack_range),
            kernel_range_up: (
                kernel_range_up.start.to_ne_bytes(),
                kernel_range_up.end.to_ne_bytes(),
            ),
            initial_stack_range_up: (
                stack_range_up.start.to_ne_bytes(),
                stack_range_up.end.to_ne_bytes(),
            ),
        }
    }

    #[cfg(target_pointer_width = "64")]
    pub fn kernel_vrange(&self) -> Range<usize> {
        usize::from_ne_bytes(self.kernel_range_up.0)..usize::from_ne_bytes(self.kernel_range_up.1)
    }

    #[cfg(target_pointer_width = "64")]
    pub fn stack_vrange(&self) -> Range<usize> {
        usize::from_ne_bytes(self.initial_stack_range_up.0)
            ..usize::from_ne_bytes(self.initial_stack_range_up.1)
    }

    #[cfg(target_pointer_width = "64")]
    pub fn kernel_phy_range(&self) -> Range<usize> {
        usize::try_from(self.kernel_range.start).unwrap()
            ..usize::try_from(self.kernel_range.end).unwrap()
    }

    #[cfg(target_pointer_width = "64")]
    pub fn stack_phy_range(&self) -> Range<usize> {
        usize::try_from(self.initial_stack_range.start).unwrap()
            ..usize::try_from(self.initial_stack_range.end).unwrap()
    }

    pub fn boot_info(&self) -> &'static BootInformation {
        unsafe {
            NonNull::new(usize::try_from(self.boot_info).unwrap() as *mut _)
                .unwrap()
                .as_ref()
        }
    }
}
