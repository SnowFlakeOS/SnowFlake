#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(try_trait)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(global_asm)]
#![feature(ptr_internals)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(global_allocator)]
#![feature(concat_idents)]
#![feature(thread_local)]
#![feature(custom_attribute)]
#![feature(abi_x86_interrupt)]
#![feature(core_intrinsics)]
#![allow(exceeding_bitshifts)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate once;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate alloc;
extern crate x86_64;
extern crate spin;
extern crate slab_allocator;
extern crate orbclient;
extern crate raw_cpuid;

#[macro_use]
mod macros;

pub mod panic;
pub mod kmain;
pub mod irq;
mod display;
mod console;
mod memory;
mod shell;
mod consts;
mod testui;

#[cfg(target_arch = "x86_64")]
#[path="../arch/x86_64/mod.rs"]
mod arch;

#[path="../../share/uefi_proto.rs"]
mod kernel_proto;

#[path="../../share/color.rs"]
mod color;

#[path="../../share/elf.rs"]
mod elf;

use core::mem;
use slab_allocator::LockedHeap;
use consts::*;

const WORD_SIZE: usize = mem::size_of::<usize>();

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

#[no_mangle]
pub unsafe extern fn memmove(dest: *mut u8, src: *const u8,
                             n: usize) -> *mut u8 {
    if src < dest as *const u8 {
        let n_usize: usize = n/WORD_SIZE; // Number of word sized groups
        let mut i: usize = n_usize*WORD_SIZE;

        // Copy `WORD_SIZE` bytes at a time
        while i != 0 {
            i -= WORD_SIZE;
            *((dest as usize + i) as *mut usize) =
                *((src as usize + i) as *const usize);
        }

        let mut i: usize = n;

        // Copy 1 byte at a time
        while i != n_usize*WORD_SIZE {
            i -= 1;
            *((dest as usize + i) as *mut u8) =
                *((src as usize + i) as *const u8);
        }
    } else {
        let n_usize: usize = n/WORD_SIZE; // Number of word sized groups
        let mut i: usize = 0;

        // Copy `WORD_SIZE` bytes at a time
        let n_fast = n_usize*WORD_SIZE;
        while i < n_fast {
            *((dest as usize + i) as *mut usize) =
                *((src as usize + i) as *const usize);
            i += WORD_SIZE;
        }

        // Copy 1 byte at a time
        while i < n {
            *((dest as usize + i) as *mut u8) =
                *((src as usize + i) as *const u8);
            i += 1;
        }
    }

    dest
}

pub const HEAP_OFFSET: usize = 0o_000_000_700_000_0000;
pub const HEAP_SIZE: usize = 1 * 1024 * 1024; // 1 MB

pub static mut KERNEL_BASE: usize = 0;

#[global_allocator]
static mut ALLOCATOR: LockedHeap = LockedHeap::empty();