#![feature(lang_items)] // required for defining the panic handler
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(try_trait)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(global_asm)]

#[macro_use]
extern crate bitflags;
extern crate x86_64;
extern crate uefi;

use core::ptr;

#[macro_use]
mod macros;

pub mod panic;
pub mod kmain;
mod display;
mod console;

#[path="../../share/uefi_proto.rs"]
mod kernel_proto;

#[path="../../share/color.rs"]
mod color;

global_asm!(include_str!("entry.s"));

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