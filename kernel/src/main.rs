#![no_std]
#![no_main]
#![feature(format_args_nl)]

#[macro_use]
mod early_println;

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    early_println!("panic {:#?}", info);
    loop {
        unsafe { asm!("hlt") };
    }
}

#[no_mangle]
extern "C" fn _start() -> ! {
    early_println!("done from long mode!");

    unreachable!()
}
