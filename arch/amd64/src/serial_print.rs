use core::fmt::Write;
use core::fmt::{self};

#[macro_export]
macro_rules! io_write_port {
    (u8,  $port:expr, $value:expr) => { core::arch::asm!("out dx, al", in("dx") $port, in("al") $value as u8) };
    (u16, $port:expr, $value:expr) => { core::arch::asm!("out dx, ax", in("dx") $port, in("ax") $value as u16) };
    (u32, $port:expr, $value:expr) => { core::arch::asm!("out dx, eax", in("dx") $port, in("eax") $value as u32) };
}

/// Default serial port used by the bootloader
pub const IO_PORT_PRINT: IOPort = IOPort(0x3F8);

#[derive(Debug)]
pub struct IOPort(u16);

impl core::fmt::Write for IOPort {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &c in s.as_bytes() {
            unsafe {
                io_write_port!(u8, self.0, c);
            };
        }
        Ok(())
    }
}

#[inline(never)]
pub fn print(args: fmt::Arguments) {
    let mut port = IO_PORT_PRINT;
    let _ = port.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    () => {};

    ($($arg:tt)*) => {
        {
            $crate::serial_print::print(format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! println {
    () => { print!("\n"); };

    ($($arg:tt)*) => {
        {
            $crate::serial_print::print(format_args_nl!($($arg)*));
        }
    };
}
