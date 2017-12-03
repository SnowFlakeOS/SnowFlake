#![feature(lang_items)]
#![feature(const_fn, unique)]
#![feature(const_unique_new)]
#![no_std]

extern crate rlibc;

#[macro_use]
pub mod vga_buffer;

pub mod vbe;

pub use vbe::load;

pub use core::ptr;

extern crate volatile;
extern crate spin;

#[no_mangle]
pub extern fn init()
{
    // ATTENTION: we have a very small stack and no guard page

    let vbe_info = unsafe{ vbe::load(0x7E00) };

    if vbe_info.xresolution == 1024 {
        let buff: *mut u8 = vbe_info.physbaseptr as *mut _;

        vga_buffer::clear_screen();
        println!("Welcome to SnowFlake 0.1.0!\n");
        println!("Screen resolution is {}x{}", vbe_info.xresolution, vbe_info.yresolution);
        println!("Video memory address is 0x{:x}", vbe_info.physbaseptr);
    }

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
