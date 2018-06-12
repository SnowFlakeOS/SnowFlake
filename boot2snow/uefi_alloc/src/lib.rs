#![feature(alloc)]
#![feature(allocator_api)]
#![feature(const_fn)]
#![feature(try_trait)]
#![no_std]

extern crate alloc;
extern crate uefi;

use alloc::heap::{Alloc, AllocErr, Layout};
use core::ops::Try;
use uefi::Void;
use uefi::boot_services::{BootServices, MemoryType};

static mut S_BOOT_SERVICES: *const BootServices = 0 as *const BootServices;

pub unsafe fn init(table: *const BootServices) {
    S_BOOT_SERVICES = table;
}

fn get_boot_services() -> Option<&'static BootServices> {
    unsafe {
        if S_BOOT_SERVICES as usize == 0 {
            None
        } else {
            Some(&*S_BOOT_SERVICES)
        }
    }
}

pub struct Allocator;

unsafe impl<'a> Alloc for &'a Allocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
	let boot_services = get_boot_services().unwrap();
        let size = layout.size();
        let align = layout.align();

        // TODO: add support for other alignments.
        if align > 8 {
            let details = "Unsupported alignment for allocation, UEFI can only allocate 8-byte aligned addresses";
            Err(AllocErr::Unsupported { details })
        } else {
            boot_services
                .allocate_pool(size)
                .map(|addr| addr as *mut u8)
                // This is the only possible error, according to the spec.
                .map_err(|_status| AllocErr::Exhausted { request: layout })
	}
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, _layout: Layout) {
	let boot_services = get_boot_services().unwrap();
       let _ = (boot_services.free_pool)(ptr as *mut Void);
    }
}
