#![no_std]
#![no_main]
#![feature(format_args_nl)]

#[macro_use]
mod early_println;

use core::{arch::asm, panic::PanicInfo, ptr::NonNull};

use bootloader::{multiboot2::MemoryInfo, KernelInformation};

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("panic {:#?}", info);
    loop {
        unsafe { asm!("hlt") };
    }
}

#[no_mangle]
extern "C" fn _start(kernel_info_ptr: u32) -> ! {
    println!("done.\n");

    let kernel_info = unsafe {
        let info_ptr = NonNull::new(kernel_info_ptr as usize as *mut KernelInformation)
            .expect("Invalid kernel information passed");
        info_ptr.as_ref()
    };

    println!("{kernel_info:#?}");

    let mut memory_info: [Option<MemoryInfo>; 20] = core::array::from_fn(|_| None);
    for (idx, mem) in kernel_info
        .boot_info()
        .memory_map()
        .unwrap()
        .iter()
        .enumerate()
    {
        memory_info[idx] = Some(mem.clone());
    }

    unreachable!()
}
