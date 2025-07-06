#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use rust_os::{exit_qemu, QemuExitCode, serial_println};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    serial_println!("======stack overflow hopefully=======");
    
    rust_os::gdt::init();
    init_test_idt(); // diff init coz we want to exit_qemu in test

    stack_overflow();

    panic!("execution continued after stack overflow");
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(rust_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read(); // prevent optimizations
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  rust_os::test_panic_handler(info);
}
