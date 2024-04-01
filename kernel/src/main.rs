#![no_std]
#![no_main]
#![feature(format_args_nl)]

#[macro_use]
mod early_println;

use core::{arch::asm, panic::PanicInfo, ptr::NonNull};

use bootloader_types::KernelInformation;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    early_println!("panic {:#?}", info);
    loop {
        unsafe { asm!("hlt") };
    }
}

#[no_mangle]
extern "C" fn _start(kernel_info_ptr: u32) -> ! {
    early_println!("done.\n");

    early_println!("Found kernel information at 0x{:x}", kernel_info_ptr);

    let kernel_info = unsafe {
        let info_ptr =
            NonNull::new(usize::try_from(kernel_info_ptr).unwrap() as *mut KernelInformation)
                .expect("Invalid kernel information passed");
        info_ptr.as_ref()
    };

    for mem in kernel_info.boot_info().memory_map().unwrap().iter() {
        early_println!("{:?}", mem);
    }

    unreachable!()
}
