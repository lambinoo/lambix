#![no_std]

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("hlt") };
    }
}
