// Main file -> Entry point of the OS

#![no_std] // No standard librray
#![no_main] // No main function (entry point is '_start')
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::println;
use bootloader::{ BootInfo, entry_point };

entry_point!(kernel_main);

// Function called upon panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os::hlt_loop();
}

// Called when testing
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}

// Entry point function to the OS (name '_start' by default)
pub fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use os::memory;
    use os::memory::BootInfoFrameAllocator;
    use x86_64::{ structures::paging::Page, VirtAddr };

    println!("Hello World{}", "!");

    os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe {
        page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e);
    }

    // Unit tests
    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    os::hlt_loop();
}
