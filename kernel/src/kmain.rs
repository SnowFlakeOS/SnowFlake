// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

use core::ops::Try;
use core::mem;
use core::ptr;

use color::*;
use kernel_proto::{Info, MemoryDescriptor};
use display::Display;
use console::{Console, set_console};

use memory;

#[no_mangle]
pub extern "C" fn kmain(magic: usize, boot_info: *const Info) -> ! {
    let info = unsafe { &*boot_info };
    let video_info = unsafe { &*(*info).video_info };

    let resolutin_w = video_info.xresolution;
    let resolutin_h = video_info.yresolution;
    let AREA = resolutin_w * resolutin_h;

    let vid_addr = video_info.physbaseptr;
    let mut display = Display::new(vid_addr, resolutin_w, resolutin_h);
    let mut console = Console::new(&mut display);
    let elf_sections = info.elf_sections;

    set_console(&mut console);
    
    display.rect(0, 0, resolutin_w, resolutin_h, Color::rgb(0, 0, 0));
    
    println!("SnowKernel {}", env!("CARGO_PKG_VERSION"));
    println!("Screen resolution is {}x{}", resolutin_w, resolutin_h);
    println!("Kernel heap start : {:#x} | size : {:#x}", ::HEAP_START, ::HEAP_SIZE);
    println!("Kernel start : {:#x} | end : {:#x}", info.kernel_base, info.kernel_base + info.kernel_size);

    unsafe {
        memory::init(0, (info.kernel_base + (info.kernel_size + 4095)/4096) * 4096);
        ::ALLOCATOR.init(::HEAP_START, ::HEAP_SIZE);
    }

    panic!("Test panic");
}
