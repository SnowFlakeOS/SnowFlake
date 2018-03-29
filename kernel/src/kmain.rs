use core::ops::Try;
use color::*;
use kernel_proto::Info;
use display::Display;
use console::{Console, set_console};

extern {
    static _magic: usize;
    static _info: *const Info;
}

#[no_mangle]
pub extern fn start_uefi() {
    let magic = unsafe { _magic };
    let info = unsafe { _info };

    let resolutin_w = unsafe { (*info).width };
    let resolutin_h = unsafe { (*info).height };
    let AREA = resolutin_w * resolutin_h;

    let mut vid_addr = unsafe { (*info).vid_addr };
    let mut display = Display::new(vid_addr, resolutin_w, resolutin_h);
    let mut console = Console::new(&mut display);
    set_console(&mut console);
    
    display.rect(0, 0, resolutin_w, resolutin_h, Color::rgb(0, 0, 0));
    println!("[INFO] SnowKernel {}", env!("CARGO_PKG_VERSION"));
}
