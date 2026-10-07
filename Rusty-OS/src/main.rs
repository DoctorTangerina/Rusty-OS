#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod test;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    //panic!("Panic at the disco!");

    #[cfg(test)]
    test_main();

    loop {}
}