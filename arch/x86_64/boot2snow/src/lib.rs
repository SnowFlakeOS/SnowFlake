#![no_std]
#![feature(alloc)]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(global_allocator)]
#![feature(lang_items)]
#![feature(try_trait)]
#![feature(ptr_internals)]

#[macro_use]
extern crate alloc;
extern crate compiler_builtins;
extern crate uefi;
extern crate uefi_alloc;
extern crate orbclient;

use core::ptr;
use uefi::reset::ResetType;
use uefi::status::Status;

#[global_allocator]
static ALLOCATOR: uefi_alloc::Allocator = uefi_alloc::Allocator;

pub static mut HANDLE: uefi::Handle = uefi::Handle(0);
pub static mut UEFI: *mut uefi::system::SystemTable = 0 as *mut uefi::system::SystemTable;

#[macro_use]
mod macros;

pub mod display;
pub mod exec;
pub mod fs;
pub mod image;
pub mod io;
pub mod loaded_image;
pub mod null;
pub mod panic;
pub mod pointer;
pub mod proto;
pub mod rt;
pub mod shell;
pub mod string;
pub mod text;
pub mod vars;
pub mod conf;

pub mod boot2snow;
