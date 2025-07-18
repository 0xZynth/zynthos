#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(zynthos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use zynthos::println;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler] 
fn panic(info: &PanicInfo) -> ! {
    zynthos::test_panic_handler(info);
}
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    zynthos::init();

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash");
    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
