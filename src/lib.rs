#![feature(lang_items)]
#![feature(const_fn, unique)]
#![feature(const_unique_new)]
#![no_std]

extern crate rlibc;

#[macro_use]
pub mod vbe;

pub use vbe::load;

extern crate volatile;
extern crate spin;

#[no_mangle]
pub extern fn init()
{
    // ATTENTION: we have a very small stack and no guard page

    let vbe_info = unsafe{ vbe::load(0x7E00) };

    if vbe_info.x_res() == 1024 {
        let hello = b"Hello World!";
        let color_byte = 0x1f; // white foreground, blue background

        let mut hello_colored = [color_byte; 24];
        for (i, char_byte) in hello.into_iter().enumerate() {
            hello_colored[i*2] = *char_byte;
        }

        // write `Hello World!` to the center of the VGA text buffer
        let buffer_ptr = (0xb8000 + 1988) as *mut _;
        unsafe { *buffer_ptr = hello_colored };
    }

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
