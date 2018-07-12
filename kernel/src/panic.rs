//! Some code was borrowed from [System76 Firmware Update](https://github.com/system76/firmware-update)

use core::fmt;

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {}

#[no_mangle]
pub extern fn ___chkstk_ms() {}

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(fmt: fmt::Arguments, file: &'static str, line: u32) -> ! {
    print!("\nPANIC in {} at line {}: {}", file, line, fmt);
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn _Unwind_Resume() {
    loop {}
}
