#![no_std]
#![no_main]

mod panic;
mod uefi;
#[macro_use]
pub mod print;
mod arch;

use arch::target_arch::cpu::halt_loop;

use crate::uefi::EfiHandle;
use crate::uefi::EfiSystemTable;

const PL011: usize = 0x900_0000;

#[no_mangle]
pub extern "C" fn efi_main(image: EfiHandle, st: EfiSystemTable) -> ! {
    let hello = "HelloWorld by UART";
    for c in hello.bytes() {
        unsafe { core::ptr::write_volatile((PL011) as *mut u8, c) };
    }
    println!("HelloWorld!");
    halt_loop();
}
