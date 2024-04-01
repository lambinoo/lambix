#![no_std]
#![no_main]
#![feature(format_args_nl)]

#[macro_use]
mod early_println;

use core::{arch::asm, panic::PanicInfo};

use arch_amd64::multiboot2::BootInformation;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    early_println!("panic {:#?}", info);
    loop {
        unsafe { asm!("hlt") };
    }
}

#[no_mangle]
extern "C" fn _start(boot_info: u32) -> ! {
    early_println!("done");

    early_println!("Found boot information at 0x{:x}", boot_info);

    let boot_info = usize::try_from(boot_info).unwrap();
    let boot_info =
        unsafe { BootInformation::from_ptr(boot_info as *mut _, BootInformation::BOOT_MAGIC) }
            .unwrap();

    for mem in boot_info.memory_map().unwrap().iter() {
        early_println!("{:?}", mem);
    }

    unreachable!()
}
