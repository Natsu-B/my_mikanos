use crate::println;
use crate::arch::target_arch::cpu::halt_loop;

#[panic_handler]
pub fn panic(panic: &core::panic::PanicInfo) -> ! {
    println!("{}",panic);
    halt_loop();
}