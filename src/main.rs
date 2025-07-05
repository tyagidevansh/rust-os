#![no_std] // dont link the std lib
#![no_main] // disable rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;
use rust_os::serial_println;

// entry point of the program
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("hello world!!!!!!!!!");
    serial_println!("main main main");   
    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))] // conditional compilation
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
