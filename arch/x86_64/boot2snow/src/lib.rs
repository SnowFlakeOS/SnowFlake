#![no_std]
#![feature(alloc)]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(global_allocator)]
#![feature(lang_items)]
#![feature(try_trait)]
#![feature(proc_macro)]

use core::ops::Try;
use core::mem;
use utf16_literal::utf16;
use uefi::runtime_services::RuntimeServices;
use uefi::boot_services::protocols;
use uefi::boot_services::protocols::{GraphicsOutput, PixelFormat};
use uefi::status::SUCCESS;
use uefi::boot_services::BootServices;
use uefi::{SimpleTextOutputInterface,
        		Handle,
				SystemTable,
				Status};

#[macro_use]
extern crate alloc;
extern crate compiler_builtins;
extern crate uefi;
extern crate uefi_alloc;
extern crate utf16_literal;

#[macro_use]
mod macros;

pub mod panic;

mod boot2snow;
mod conf;
mod io;
mod string;

#[path="../../../../share/elf.rs"]
mod elf;

#[path="../../../../share/uefi_proto.rs"]
mod kernel_proto;

#[path="../../../../share/color.rs"]
mod color;

#[global_allocator]
static ALLOCATOR: uefi_alloc::Allocator = uefi_alloc::Allocator;

static PATH_CONFIG: &'static [u16] = utf16!("boot2snow\\boot2snow.conf\0");
static PATH_FALLBACK_KERNEL: &'static [u16] = utf16!("boot2snow\\kernel.bin\0");

// Globals used for panic handling and loging
static mut S_CONOUT: *const SimpleTextOutputInterface = 1 as *const _;
static mut S_BOOT_SERVICES: *const BootServices = 0 as *const _;
static mut S_RUNTIME_SERVICES: *const RuntimeServices = 0 as *const _;
static mut S_GRAPHICS_OUTPUT: *const GraphicsOutput = 0 as *const _;
static mut S_IMAGE_HANDLE: Handle = 0 as *mut _;

pub type EntryPoint = extern "cdecl" fn(usize, *const kernel_proto::Info) -> !;

pub fn get_conout() -> &'static SimpleTextOutputInterface {
	unsafe { &*S_CONOUT }
}

pub fn get_boot_services() -> &'static BootServices {
	unsafe { &*S_BOOT_SERVICES }
}

pub fn get_image_handle() -> Handle {
	unsafe { S_IMAGE_HANDLE }
}

pub fn get_runtime_services() -> &'static RuntimeServices {
	unsafe { &*S_RUNTIME_SERVICES }
}

pub fn get_graphics_output() -> &'static GraphicsOutput {
	unsafe { &*S_GRAPHICS_OUTPUT }
}

fn set_text_mode(output: &SimpleTextOutputInterface) -> Result<(), ()> {
    let mut max_i = None;
    let mut max_w = 0;
    let mut max_h = 0;

    for i in 0..output.mode.max_mode as usize {
        let mut w = 0;
        let mut h = 0;
        if output.query_mode(i, &mut w, &mut h).into_result().is_ok() {
            if w >= max_w && h >= max_h {
                max_i = Some(i);
                max_w = w;
                max_h = h;
            }
        }
    }

    if let Some(i) = max_i {
        let _ = output.set_mode(i);
    }

    Ok(())
}

fn set_graphics_mode(output: &GraphicsOutput) -> Result<(), ()> {
	let mut mode: u32 = 0;

    for i in 0..output.mode.max_mode {
        let info = output.query_mode(i).unwrap();

        if info.pixel_format != PixelFormat::RGBX
            && info.pixel_format != PixelFormat::BGRX { continue; }
        if info.horizontal_resolution > 1920 && info.vertical_resolution > 1080 { continue; }
        if info.horizontal_resolution == 1920 && info.vertical_resolution == 1080 { mode = i; break; }
        mode = i;
    };

    let _ = output.set_mode(mode);

	Ok(())
}

#[no_mangle]
pub extern "win64" fn _start(image_handle: Handle, system_table: &SystemTable) -> Status
{
	let conout = system_table.con_out();

	unsafe {
		S_CONOUT = conout;
		S_IMAGE_HANDLE = image_handle;
		S_BOOT_SERVICES = system_table.boot_services;
        S_RUNTIME_SERVICES = system_table.runtime_services;

		uefi_alloc::init(::core::mem::transmute(&mut get_boot_services()));
	}

	let gop = GraphicsOutput::new(get_boot_services()).unwrap();

	{
		if let Err(err) = set_text_mode(system_table.con_out).into_result() {
        	println!("Sorry, set_text_mode() Failed :( {:?}", err); 
    	}

		if let Err(err) = set_graphics_mode(gop).into_result() {
        	println!("Sorry, set_graphics_mode() Failed :( {:?}", err); 
    	}

		unsafe { S_GRAPHICS_OUTPUT = gop };

    	if let Err(err) = boot2snow::init().into_result() {
        	println!("Sorry, boot2snow::init() Failed :( {:?}", err);
        	loop {};
    	}
	}

    SUCCESS
}

