#![no_std]
#![no_main]
#![feature(format_args_nl)]
#![feature(naked_functions)]

#[macro_use]
extern crate arch_amd64;

use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr::NonNull;

use arch_amd64::apic;
use arch_amd64::idt::IDT;
use bootloader::multiboot2::MemoryInfo;
use bootloader::KernelInformation;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{:#?}", info);
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

    apic::disable_legacy_pic();
    let local_apic = apic::LocalAPIC::get_local();
    println!(
        "Setting up APIC with id {:x} / {:x} at address {:x?}",
        local_apic.apic_id(),
        local_apic.apic_version(),
        local_apic
    );

    let idt = IDT::default();
    let loaded_idt = idt.load_idt();
    println!("IDT loaded: {:x?}", loaded_idt);

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

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
