use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("Panicked: {:#?}", info);
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}
