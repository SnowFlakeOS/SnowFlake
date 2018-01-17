#![feature(lang_items)]
#![feature(const_fn, unique)]
#![feature(const_unique_new)]
#![feature(asm)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;

#[macro_use]
pub mod vga_buffer;
pub mod vbe;

#[path="arch/x86_64/x86_io.rs"]
pub mod x86_io;

pub use vbe::load;
pub use core::ptr;
pub use x86_io::{ inb, outb };

#[no_mangle]
pub extern fn init()
{
    // ATTENTION: we have a very small stack and no guard page

    let vbe_info = unsafe { vbe::load(0x7E00) };

    if vbe_info.xresolution == 1024 {
        let buff: *mut u8 = vbe_info.physbaseptr as *mut _;

        vga_buffer::clear_screen();
        println!("Welcome to SnowFlake 0.1.0!\n");
        println!("Screen resolution is {}x{}", vbe_info.xresolution, vbe_info.yresolution);
        println!("Video memory address is 0x{:x}", vbe_info.physbaseptr);
    }

    let total = {
        let low = unsafe {
            outb(0x70, 0x30);
            inb(0x71)
        };

        let high = unsafe {
            outb(0x70, 0x31);
            inb(0x71)
        };

        ((low | high) as isize) << 8
    };

    println!("Delected memory is {} KB", total);

    while let scancode = unsafe { inb(0x60) } {
        if scancode == 1 {
            println!("Scancode is {}", scancode);
        }
    }
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! { loop{} }
