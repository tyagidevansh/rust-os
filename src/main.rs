#![no_std] // dont link the std lib
#![no_main] // disable rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;


// entry point of the program
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("hello world!!!!!!!!!");
    panic!("some panic message");
    loop {}
}
// just loop infinitely for now if panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
