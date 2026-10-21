#![no_std]

#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
use core::panic::PanicInfo;

mod exception;

pub mod qemu;
pub mod serial;
pub mod test;
pub mod vga_buffer;

pub use qemu::exit::{exit_qemu, QemuExitCode};
pub use test::definitions::{test_panic_handler, test_runner, Testable};

pub fn init() {
    exception::interrupt::init_idt();   
}

/// Entry point for `cargo test`
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}