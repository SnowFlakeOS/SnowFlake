#![feature(lang_items)]
#![feature(const_fn, unique)]
#![feature(const_unique_new)]
#![feature(ptr_internals)]
#![feature(asm)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;

#[macro_use]
pub mod vga_buffer;
pub mod vbe;

#[path="../../arch/x86_64/x86_io.rs"]
pub mod x86_io;

pub use vbe::load;
pub use core::ptr;
pub use x86_io::{ inb, outb };

#[no_mangle]
pub extern fn kmain()
{
    // ATTENTION: we have a very small stack and no guard page

    vga_buffer::clear_screen();
    println!("Welcome to SnowFlake 0.1.0!\n");

    loop {  }
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! { loop{} }
