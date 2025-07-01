#![no_std] // dont link the std lib
#![no_main] // disable rust-level entry points

use core::panic::PanicInfo;

// entry point of the program [doesnt return anything]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

// just loop infinitely for now if panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
