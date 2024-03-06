#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("hlt") };
    }
}

extern "C" fn _start() {}
