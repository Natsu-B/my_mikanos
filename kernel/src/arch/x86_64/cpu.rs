use core::arch::asm;

/// Halt Loop
///
/// Stop the cpu.
pub fn halt_loop() -> ! {
    loop {
        unsafe { asm!("hlt") };
    }
}
