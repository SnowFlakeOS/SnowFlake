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

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate alloc;
extern crate uefi;
extern crate x86_64;
extern crate slab_allocator;
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

use slab_allocator::LockedHeap;

// Globals used for panic handling and loging
static mut S_CONIN: *mut SimpleInputInterface = 1 as *mut _;
static mut S_CONOUT: *const SimpleTextOutputInterface = 1 as *const _;
static mut S_BOOT_SERVICES: *const BootServices = 0 as *const _;
static mut S_RUNTIME_SERVICES: *const RuntimeServices = 0 as *const _;
static mut S_IMAGE_HANDLE: Handle = 0 as *mut _;

pub type EntryPoint = extern "C" fn(usize, *const kernel_proto::Info) -> !;

pub const HEAP_OFFSET: usize = 0o_000_000_070_000_0000;
pub const HEAP_SIZE: usize = 50 * 1024 * 1024;

#[global_allocator]
static mut ALLOCATOR: LockedHeap = LockedHeap::empty();

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

		::ALLOCATOR.init(::HEAP_OFFSET, ::HEAP_SIZE);
	}

	{
		if let Err(err) = set_text_mode(system_table.con_out).into_result() {
        	println!("Sorry, set_text_mode() Failed :( {:?}", err); 
    	}

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