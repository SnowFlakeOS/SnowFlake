use core::ops::Try;
use uefi::runtime_services::RuntimeServices;
use color::*;
use kernel_proto::{Info, MemoryDescriptor};
use display::Display;
use console::{Console, set_console};

extern {
    static _magic: usize;
    static _info: *const Info;
}

#[no_mangle]
pub extern fn start_uefi() {
    let magic = unsafe { _magic };
    let info = unsafe { &*_info };
    let video_info = unsafe { &*(*info).video_info };

    let resolutin_w = video_info.xresolution;
    let resolutin_h = video_info.yresolution;
    let AREA = resolutin_w * resolutin_h;

    let vid_addr = video_info.physbaseptr;
    let mut display = Display::new(vid_addr, resolutin_w, resolutin_h);
    let mut console = Console::new(&mut display);
    let map = info.map_addr as *const MemoryDescriptor;
    set_console(&mut console);

    enable_nxe_bit();
    enable_write_protect_bit();
    
    display.rect(0, 0, resolutin_w, resolutin_h, Color::rgb(0, 0, 0));
    
    println!("SnowKernel {}", env!("CARGO_PKG_VERSION"));

    panic!("Test panic");
}

// https://github.com/phil-opp/blog_os/blob/post_10/src/lib.rs

fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}