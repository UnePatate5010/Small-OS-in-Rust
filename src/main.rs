// Main file -> Entry point of the OS

#![no_std] // No standard librray
#![no_main] // No main function (entry point is '_start')
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::println;

// Function called upon panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Called when testing
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}


// Entry point function to the OS (name '_start' by default)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    os::init();

    // Trigger a page fault
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };

    // Unit tests
    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    loop {}
}