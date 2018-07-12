use x86_64::structures::idt::ExceptionStackFrame;

use super::{apic, pic};

unsafe fn trigger(irq: u8) {
    extern {
        fn irq_trigger(irq: u8);
    }

    if irq < 16 {
        if irq >= 8 {
            pic::SLAVE.mask_set(irq - 8);
            pic::MASTER.ack();
            pic::SLAVE.ack();
        } else {
            pic::MASTER.mask_set(irq);
            pic::MASTER.ack();
        }
    }

    irq_trigger(irq);
}

pub extern "x86-interrupt" fn keyboard_handler(
    _stack_frame: &mut ExceptionStackFrame) 
{
    unsafe { trigger(1) }
}