use x86_64::structures::idt::{ InterruptDescriptorTable, InterruptStackFrame };
use crate::println;
use lazy_static::lazy_static; // perform static initialization when encountered for the first time (instead of at compile time)

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new(); // Creates the table
        idt.breakpoint.set_handler_fn(breakpoint_handler); // Install handler function
        idt
    };
}

// Creates a interruption desciptor table
pub fn init_idt() {
    IDT.load(); // Load the table
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}


#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}