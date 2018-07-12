pub mod apic;
pub mod pic;
pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod irq;
pub mod serial;
pub mod keyboard;
pub mod paging;

pub unsafe fn init() {
    //paging::init();
    //gdt::init();
    idt::init();
    pic::init();
    apic::init();
    keyboard::init().expect("Sorry, keyboard::init() Failed :(");
}