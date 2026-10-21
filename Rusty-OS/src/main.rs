#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rusty_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rusty_os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    rusty_os::init();

    println!("Hello World{}", "!");
    //panic!("Panic at the disco!");

    x86_64::instructions::interrupts::int3();

    println!("Actually works");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rusty_os::test_panic_handler(info)
}