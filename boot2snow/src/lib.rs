// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//! Some code was borrowed from [Tifflin Bootloader](https://github.com/thepowersgang/rust_os)

#![no_std]
#![feature(alloc)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(global_allocator)]
#![feature(lang_items)]
#![feature(try_trait)]
#![feature(proc_macro)]

use core::ops::Try;
use uefi::runtime_services::RuntimeServices;
use uefi::status::SUCCESS;
use uefi::boot_services::BootServices;
use uefi::{SimpleInputInterface,
			SimpleTextOutputInterface,
        	Handle,
			SystemTable,
			Status};
use uefi::boot_services::protocols::{GraphicsOutput, ModeInformation, PixelFormat};

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate alloc;
extern crate uefi;
extern crate x86_64;
extern crate uefi_alloc;
extern crate orbclient;

#[macro_use]
mod macros;

pub mod panic;

mod boot2snow;
mod conf;
mod io;
mod string;
mod memory_map;
mod paging;
mod fs;
mod image;
mod display;

#[path="../../share/elf.rs"]
mod elf;

#[path="../../share/uefi_proto.rs"]
mod kernel_proto;

#[path="../../share/color.rs"]
mod color;

// Globals used for panic handling and loging
static mut S_CONIN: *mut SimpleInputInterface = 1 as *mut _;
static mut S_CONOUT: *const SimpleTextOutputInterface = 1 as *const _;
static mut S_BOOT_SERVICES: *const BootServices = 0 as *const _;
static mut S_RUNTIME_SERVICES: *const RuntimeServices = 0 as *const _;
static mut S_IMAGE_HANDLE: Handle = 0 as *mut _;

pub type EntryPoint = extern "C" fn(usize, *const kernel_proto::Info) -> !;

#[global_allocator]
static ALLOCATOR: uefi_alloc::Allocator = uefi_alloc::Allocator;

pub fn get_conin() -> &'static mut SimpleInputInterface {
	unsafe { &mut *S_CONIN }
}

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

fn set_graphics_mode(output: &GraphicsOutput) -> Result<(), ()> {
    let mut max_i = None;
    let mut max_w = 0;
    let mut max_h = 0;

    for i in 0..output.mode.max_mode as usize {
        let mut mode_ptr: *mut ModeInformation = ::core::ptr::null_mut();
		let mut mode_size = 0;
        if (output.query_mode)(output, i as u32, &mut mode_size, &mut (mode_ptr as *const ModeInformation)).into_result().is_ok() {
			let mode = unsafe { &mut *mode_ptr };

			let w = mode.horizontal_resolution;
			let h = mode.vertical_resolution;
			let pixel_format = mode.pixel_format;
            if w >= max_w && h >= max_h && pixel_format == PixelFormat::BGRX {
                max_i = Some(i as u32);
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

#[no_mangle]
pub extern "win64" fn _start(image_handle: Handle, system_table: &SystemTable) -> Status
{
	let conin = system_table.con_in();
	let conout = system_table.con_out();

	unsafe {
		S_CONIN = conin as *const _ as *mut _;
		S_CONOUT = conout;
		S_IMAGE_HANDLE = image_handle;
		S_BOOT_SERVICES = system_table.boot_services;
        S_RUNTIME_SERVICES = system_table.runtime_services;

		uefi_alloc::init(get_boot_services());
	}

	let gop = GraphicsOutput::new(get_boot_services()).unwrap();

	{
    	if let Err(err) = set_graphics_mode(gop).into_result() {
        	println!("Sorry, set_graphics_mode() Failed :( {:?}", err);
        	loop {};
    	}

		let _ = conout.set_attribute(0x0F);

    	if let Err(err) = boot2snow::init().into_result() {
        	println!("Sorry, boot2snow::init() Failed :( {:?}", err);
        	loop {};
    	}
	}

    SUCCESS
}

#[no_mangle]
pub extern "C" fn memcpy(dst: *mut u8, src: *const u8, count: usize) {
	unsafe {
		asm!("rep movsb" : : "{rcx}" (count), "{rdi}" (dst), "{rsi}" (src) : "rcx", "rsi", "rdi" : "volatile");
	}
}

#[no_mangle]
pub extern "C" fn memset(dst: *mut u8, val: u8, count: usize) {
	unsafe {
		asm!("rep stosb" : : "{rcx}" (count), "{rdi}" (dst), "{al}" (val) : "rcx", "rdi" : "volatile");
	}
}

#[no_mangle]
pub extern "C" fn memcmp(dst: *mut u8, src: *const u8, count: usize) -> isize {
	unsafe {
		let rv: isize;
		asm!("repnz cmpsb ; movq $$0, $0 ; ja 1f; jb 2f; jmp 3f; 1: inc $0 ; jmp 3f; 2: dec $0; 3:" : "=r" (rv) : "{rcx}" (count), "{rdi}" (dst), "{rsi}" (src) : "rcx", "rsi", "rdi" : "volatile");
		rv
	}
}