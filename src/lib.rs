#![feature(lang_items)]
#![feature(const_fn, unique)]
#![feature(const_unique_new)]
#![no_std]

extern crate rlibc;

#[macro_use]
mod vga_buffer;

extern crate volatile;
extern crate spin;

#[no_mangle]
pub extern fn init()
{
    // ATTENTION: we have a very small stack and no guard page
    //vga_buffer::clear_screen();
    //println!("Hello World{}", "!");

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
