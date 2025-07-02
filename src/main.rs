#![no_std] // dont link the std lib
#![no_main] // disable rust-level entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

// entry point of the program
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
// just loop infinitely for now if panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
