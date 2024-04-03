#![no_std]

use core::ptr::NonNull;

use core::ops::Range;
use multiboot2::BootInformation;

pub mod multiboot2;

#[cfg_attr(target_width_pointer = "32", Derive(Debug))]
#[repr(C)]
pub struct KernelInformation {
    boot_info: u32,
    kernel_range: Range<u32>,
    initial_stack_range: Range<u32>,
    kernel_vrange: Range<u64>,
    stack_vrange: Range<u64>,
}

fn convert_range<T: TryInto<usize>>(range: Range<T>) -> Range<*const ()> {
    let start = TryInto::<usize>::try_into(range.start).unwrap_or_default() as _;
    let end = TryInto::<usize>::try_into(range.end).unwrap_or_default() as _;
    start..end
}

impl core::fmt::Debug for KernelInformation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("KernelInformation")
            .field("boot_info", &format_args!("0x{:x}", &self.boot_info))
            .field(
                "kernel_range",
                &format_args!("{:?}", convert_range(self.kernel_range.clone())),
            )
            .field(
                "initial_stack_range",
                &format_args!("{:?}", convert_range(self.initial_stack_range.clone())),
            )
            .field(
                "kernel_vrange",
                &format_args!("{:?}", convert_range(self.kernel_vrange.clone())),
            )
            .field(
                "stack_vrange",
                &format_args!("{:?}", convert_range(self.stack_vrange.clone())),
            )
            .finish()
    }
}

impl KernelInformation {
    pub fn new(
        boot_info: &BootInformation,
        kernel_range: Range<*const u8>,
        stack_range: Range<*const u8>,
        kernel_vrange: Range<u64>,
        stack_vrange: Range<u64>,
    ) -> KernelInformation {
        let stable_abi_range = |range: Range<*const u8>| Range {
            start: u32::try_from(range.start as usize).unwrap(),
            end: u32::try_from(range.end as usize).unwrap(),
        };

        Self {
            boot_info: u32::try_from(core::ptr::from_ref(boot_info) as usize).unwrap(),
            kernel_range: stable_abi_range(kernel_range),
            initial_stack_range: stable_abi_range(stack_range),
            kernel_vrange,
            stack_vrange,
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
