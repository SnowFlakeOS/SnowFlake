use x86_64::structures::idt::{Idt, ExceptionStackFrame};

use super::interrupts;
use super::irq;
use super::gdt;

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();

        //idt.divide_by_zero.set_handler_fn(divide_by_zero);
        idt.breakpoint.set_handler_fn(interrupts::breakpoint_handler);
        idt.double_fault.set_handler_fn(interrupts::double_fault_handler);

        //idt.interrupts[33].set_handler_fn(irq::keyboard_handler); // 33 -> Keyboard

        idt
    };
}

pub fn init() {
    IDT.load();
}

