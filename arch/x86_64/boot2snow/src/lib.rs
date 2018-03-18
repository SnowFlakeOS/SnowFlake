#![no_std]
#![feature(asm)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![feature(try_trait)]
#![feature(use_extern_macros)]

extern crate uefi;
extern crate rlibc;
extern crate compiler_builtins;

#[macro_use]
extern crate utf16_literal;

use core::ops::Try;
use uefi::{SimpleTextOutput, 
                SimpleFileSystem,
                Handle, 
                SystemTable, 
                Status, 
                Attribute,
                Console,
                ForegroundColor, 
                BackgroundColor, 
                initialize_lib,
                get_system_table,
                get_handle,
                lib_memory_map};

use uefi::graphics::{PixelFormat, Pixel, GraphicsOutputProtocol};
use core::{mem, ptr};
use core::fmt::Write;

#[macro_use]
mod macros;

mod boot2snow;
mod io;
mod panic;

pub static mut MODE: u32 = 0;

fn console_max_mode(output: &Console) -> Result<(), ()> {
    let mut max_i = None;
    let mut max_w = 0;
    let mut max_h = 0;

    for i in 0..output.mode().max_mode as usize {
        let mut w = 0;
        let mut h = 0;
        if output.query_mode(i, &mut w, &mut h) == Status::Success {
            if w >= max_w && h >= max_h {
                max_i = Some(i);
                max_w = w;
                max_h = h;
            }
        }
    }

    if let Some(i) = max_i {
        output.set_mode(i);
    }

    Ok(())
}

fn graphics_max_mode(gop: &GraphicsOutputProtocol) -> Result<(), ()> {
    let mut mode: u32 = 0;
    for i in 0..gop.get_max_mode() {
        let info = gop.query_mode(i).unwrap();

        if info.pixel_format != PixelFormat::RedGreenBlue
            && info.pixel_format != PixelFormat::BlueGreenRed { continue; }
        if info.horizontal_resolution > 1920 && info.vertical_resolution > 1080 { continue; }
        if info.horizontal_resolution == 1920 && info.vertical_resolution == 1080 { mode = i; break; }
        mode = i;
    };

    gop.set_mode(mode);

    unsafe{ MODE = mode };

    Ok(())
}

#[allow(unreachable_code)]
#[no_mangle]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub extern "win64" fn _start(hdl: Handle, sys: SystemTable) -> Status {
    initialize_lib(&hdl, &sys);
    
    let bs = get_system_table().boot_services();
    let rs = get_system_table().runtime_services();
    let console = get_system_table().console();
    let gop = GraphicsOutputProtocol::new().unwrap();

    let _ = bs.set_watchdog_timer(0, 0);
    let _ = console.set_attribute(Attribute::new(ForegroundColor::White, BackgroundColor::Black));

    if let Err(err) = console_max_mode(&console).into_result() {
        println!("Failed to console max mode: {:?}", err);
    }

    if let Err(err) = graphics_max_mode(&gop).into_result() {
        println!("Failed to graphics max mode: {:?}", err);
    }

    if let Err(err) = boot2snow::init().into_result() {
        println!("Sorry, boot2snow::init() Failed :( {:?}", err);
        loop {};
    }

    loop {}
    Status::Success
}
