// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

use x86_64::instructions::interrupts::enable;

use kernel_proto::{Info, MemoryDescriptor};
use display::Display;
use color::*;

use arch;
use memory;
use shell;
use testui;

#[no_mangle]
pub extern "C" fn kmain(magic: usize, boot_info: *const Info) -> ! {
    let info = unsafe { &*boot_info };
    let video_info = unsafe { &*(*info).video_info };

    let resolution_w = video_info.xresolution;
    let resolution_h = video_info.yresolution;
    let vid_addr = video_info.physbaseptr;
    let elf_sections = info.elf_sections;

    unsafe { 
        //enable();
        ::KERNEL_BASE = info.kernel_base;
    }

    println!("SnowKernel {}", env!("CARGO_PKG_VERSION"));
    println!("Screen resolution is {}x{}", resolution_w, resolution_h);
    println!("Kernel heap start : {:#x} | size : {:#x}", ::HEAP_OFFSET, ::HEAP_SIZE);
    println!("Kernel start : {:#x} | end : {:#x}", info.kernel_base, info.kernel_base + info.kernel_size);

    unsafe {
        memory::init(0, (info.kernel_base + (info.kernel_size + 4095)/4096) * 4096);
        //let (mut active_table, tcb_offset) = paging::init(0, info.kernel_base, info.kernel_base + info.kernel_size, info.stack_base, info.stack_base + info.stack_size);
        ::ALLOCATOR.init(::HEAP_OFFSET, ::HEAP_SIZE);
        arch::init();
        asm!("int3");
    }
    
    let mut display = Display::new(vid_addr as *mut Color, resolution_w, resolution_h);

    {
        let (w, h) = { (resolution_w / 3, 8) };
        let (x, y) = { (video_info.splashx - w as i32 / 2, video_info.splashy) };
        display.rect(x, y, w, 100, Color::rgb(0, 0, 0));
        display.rounded_rect(x, y, w, h, 2, false, Color::rgb(255, 255, 255));

        progress_bar(&mut display, x, y, resolution_w, 100);
    }

    shell::execute(&mut display);

    panic!("Test panic");
}

fn progress_bar(display: &mut Display, x: i32, y: i32, resolution_w: u32, progress: u32) {
    let (w, h) = { ((resolution_w / 3) / 100 as u32, 8) };
    let progress = (progress as f32 * 1.35) as u32;
    let (p1, p2) = { (progress - 1, progress - 2) };
    
    for i in 0..progress {
        if i == 0 {
            display.rounded_rect(x, y, w + 4, h, 2, true, Color::rgb(255, 255, 255));
        } else if i == p2 { } else if i == p1 {
            display.rounded_rect(x + (p2 * w) as i32, y, w * 2, h, 2, true, Color::rgb(255, 255, 255));
        } else {
            display.rect(x + (i * w) as i32, y, (w as f32 * 1.5) as u32, h, Color::rgb(255, 255, 255));
        }
    }
}