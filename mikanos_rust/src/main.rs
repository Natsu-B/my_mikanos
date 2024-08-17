#![no_std]
#![no_main]

const PL011: usize = 0x900_0000;

pub struct EfiSystemTable {}

#[repr(transparent)]
pub struct EfiHandle();

#[no_mangle]
pub extern "C" fn efi_main(image: EfiHandle, st: EfiSystemTable) -> ! {
    let string = "HelloRustUEFIWorld!!!";
    for i in string.as_bytes() {
        unsafe {
            core::ptr::write_volatile(PL011 as *mut u32, *i as u32);
        }
    }
    loop {
        unsafe { core::arch::asm!("wfi") };
    }
}

#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("wfi") }
    }
}
