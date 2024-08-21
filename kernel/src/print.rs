use crate::uefi::{output::EfiOutputProtocol, EfiStatus::EfiSuccess};

use core::fmt::{self, Write};

struct Printer {
    p: *const EfiOutputProtocol,
}

static mut PRINTER: Printer = Printer {
    p: core::ptr::null(),
};

impl Write for Printer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.p.is_null() {
            return Err(fmt::Error {});
        }

        let p = unsafe { &*self.p };
        let mut buf = [0; 256];
        let mut pointer = 0;

        for x in s.encode_utf16() {
            if pointer >= buf.len() - 1 {
                let status = (p.output_string)(p, buf.as_ptr());
                if status != EfiSuccess {
                    return Err(fmt::Error {});
                }
                pointer = 0;
            }
            if x == b'\n' as u16 {
                buf[pointer] = b'\r' as u16;
                buf[pointer + 1] = x;
                let status = (p.output_string)(p, buf.as_ptr());
                if status != EfiSuccess {
                    return Err(fmt::Error {});
                }
                pointer = 0;
                continue;
            }
            buf[pointer] = x;
            pointer += 1;
        }
        buf[pointer] = 0;
        if (p.output_string)(p, buf.as_ptr()) == EfiSuccess {
            Ok(())
        } else {
            Err(fmt::Error {})
        }
    }
}

pub fn init(output_service: *const EfiOutputProtocol) {
    unsafe { PRINTER.p = output_service };
}

pub fn print(args: fmt::Arguments) {
    let _ = unsafe { PRINTER.write_fmt(args) };
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(concat!($fmt, "\n"), $($arg)*));
}
/*
fn _print(arg: fmt::Arguments) {
    
}*/