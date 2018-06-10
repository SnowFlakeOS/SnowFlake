// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//! Some code was borrowed from [Tifflin Bootloader](https://github.com/thepowersgang/rust_os)

use core::{mem, slice};
use uefi::status::*;
use uefi::boot_services::{AllocateType,
                          MemoryDescriptor,
                          MemoryType};
use uefi::boot_services::protocols::PixelFormat;
use orbclient::{Color as InnerColor, Renderer};

use {EntryPoint, elf, kernel_proto};
use color::*;
use conf::load_conf;
use memory_map::{MM_BASE, memory_map};
use paging::paging;
use fs::{find, load};
use io::wait_timeout;
use image;
use image::Image;
use display::{Display, Output};

static mut KERNEL_BASE: usize = 0;
static mut KERNEL_SIZE: usize = 0;
static mut KERNEL_ENTRY: usize = 0;
static STACK_BASE: usize = 0x80000;
static STACK_SIZE: usize = 0x1F000;

use {get_boot_services,
     get_image_handle,
     get_runtime_services};

pub extern fn init() -> Result<(), ()> {
    let boot_services = get_boot_services();
    let image_handle = get_image_handle();
    let runtime_services = get_runtime_services();

	{
    	let (mut display, vid_addr) = {
        	let output = Output::one().unwrap();

        	let mut mode: u32 = 0;

    		for i in 0..output.0.mode.max_mode {
        		let info = output.0.query_mode(i).unwrap();

        		if info.pixel_format != PixelFormat::RGBX
            	&& info.pixel_format != PixelFormat::BGRX { continue; }
        		if info.horizontal_resolution > 1920 && info.vertical_resolution > 1080 { continue; }
        		if info.horizontal_resolution == 1920 && info.vertical_resolution == 1080 { mode = i; break; }
        		mode = i;
    		};

        	let _ = output.0.set_mode(mode);
    	    let vid_addr = output.0.mode.frame_buffer_base;

        	(Display::new(output), vid_addr)
		};

		let conf = load_conf();

    	let mut splash = Image::new(0, 0);
    	{
        	if let Ok(data) = load("\\boot2snow\\only_logo.bmp") {
            	if let Ok(image) = image::bmp::parse(&data) {
                	splash = image;
            	}
        	}
		}

    	{
        	let bg = InnerColor::rgb(0x00, 0x00, 0x00);

        	display.set(bg);

        	{
            	let x = (display.width() as i32 - splash.width() as i32) / 2;
            	let y = ((display.height() as i32 - splash.height() as i32) / 2) as i32 - 32;
            	splash.draw(&mut display, x, y);
			}

			display.sync();
		}

		status_msg(&mut display, splash.height(), concat!("Boot2Snow ", env!("CARGO_PKG_VERSION")));
		wait_timeout(conf.boot_timeout);

		// - Load the kernel.
		let entrypoint = load_kernel_file(&conf.kernel).expect("Unable to load kernel");
		let sections = load_kernel_sections(&conf.kernel).expect("Unable to load sections");

		// Save memory map
		let (map_size, map_key, ent_size, ent_ver, map) = unsafe { memory_map() };

		unsafe { 
			(boot_services.exit_boot_services)(image_handle, map_key).expect("Sorry, exit_boot_services() failed");
			(runtime_services.set_virtual_address_map)(map_size, ent_size, ent_ver, map).expect("Sorry, set_virtual_address_map() failed :(");
		}

        unsafe {
            asm!("cli" : : : "memory" : "intel", "volatile");
            paging();
        }

		let video_info = kernel_proto::VideoInfo {
			physbaseptr: vid_addr as *mut Color,
			xresolution: display.width(),
			yresolution: display.height()
		};

		let boot_info = kernel_proto::Info {
			runtime_services: runtime_services as *const _ as *const (),
			
			// TODO: Get from the configuration
			cmdline_ptr: 1 as *const u8,
			cmdline_len: 0,

			elf_sections: Some(sections),
			kernel_base: unsafe { KERNEL_BASE },
			kernel_size: unsafe { KERNEL_SIZE },
			stack_base: STACK_BASE,
			stack_size: STACK_BASE + STACK_SIZE,
			
			map_addr: MM_BASE,
			map_len: (map_size / ent_size) as u32,
			descriptor_size: mem::size_of::<MemoryDescriptor>() as u32,

           video_info: &video_info
		};
		
		// - Execute kernel (passing a magic value and general boot information)
		unsafe { asm!("mov rsp, $0" : : "r"(STACK_BASE + STACK_SIZE) : "memory" : "intel", "volatile") };
		entrypoint(0x71FF0EF1, &boot_info);
    }

	Ok(())
}

fn status_msg(display: &mut Display, splash_height: u32, msg: &str) {
    let prompt = msg.clone();
    let mut x = (display.width() as i32 - prompt.len() as i32 * 8) / 2;
    let y = ((display.height() as i32 - splash_height as i32) / 2) as i32 + 256;

    let rect_x = 0;
    let rect_y = (y - 16);
    let rect_width = display.width();
    let rect_height = (y + 16) as u32;

    display.rect(rect_x, rect_y, rect_width, rect_height, InnerColor::rgb(0x00, 0x00, 0x00));

    for c in prompt.chars() {
        display.char(x, y, c, InnerColor::rgb(0xff, 0xff, 0xff));
        x += 8;
    }

    display.sync();
}

fn load_kernel_sections(filename: &str) -> Result<elf::PhEntIter<'static>, Status> {
	let mut kernel_file = match find(filename) {
		Ok(k) => { k.1 },
		Err(e) => panic!("Failed to open kernel '{}' - {:?}", filename, e),
	};

    // Load kernel from this file (ELF).
	let elf_hdr = {
		let mut hdr = elf::ElfHeader::default();
		// SAFE: Converts to POD for read
		kernel_file.read( unsafe { slice::from_raw_parts_mut( &mut hdr as *mut _ as *mut u8, mem::size_of::<elf::ElfHeader>() ) } ).expect("ElfHeader read");
		hdr
	};

    let slice: &[elf::PhEnt] = unsafe {
		let ptr = (&elf_hdr as *const _ as usize + elf_hdr.e_phoff as usize) as *const elf::PhEnt;
	    slice::from_raw_parts(ptr, elf_hdr.e_phnum as usize)
	};

    Ok(elf::PhEntIter(slice))
}

fn load_kernel_file(filename: &str) -> Result<EntryPoint, Status> {
    let boot_services = get_boot_services();

	let mut kernel_file = match find(filename) {
		Ok(k) => { k.1 },
		Err(e) => panic!("Failed to open kernel '{}' - {:?}", filename, e),
	};

	// Load kernel from this file (ELF).
	let elf_hdr = {
		let mut hdr = elf::ElfHeader::default();
		// SAFE: Converts to POD for read
		kernel_file.read( unsafe { slice::from_raw_parts_mut( &mut hdr as *mut _ as *mut u8, mem::size_of::<elf::ElfHeader>() ) } ).expect("ElfHeader read");
		hdr
	};

	elf_hdr.check_header();
	for i in 0 .. elf_hdr.e_phnum {
		let mut ent = elf::PhEnt::default();
		kernel_file.set_position(elf_hdr.e_phoff + (i as usize * mem::size_of::<elf::PhEnt>()) as u64);
		// SAFE: Converts to POD for read
		kernel_file.read( unsafe { slice::from_raw_parts_mut( &mut ent as *mut _ as *mut u8, mem::size_of::<elf::PhEnt>() ) } ).expect("PhEnt read");
		
		if ent.p_type == 1 {
			println!("- {:#x}+{:#x} loads +{:#x}+{:#x}",
				ent.p_paddr, ent.p_memsz,
				ent.p_offset, ent.p_filesz
				);

			unsafe {
				KERNEL_BASE = ent.p_vaddr as usize;
				KERNEL_SIZE = ent.p_memsz as usize;
			}
			
			let mut addr = ent.p_paddr;
			// SAFE: Correct call to FFI
			unsafe { (boot_services.allocate_pages)(
				AllocateType::Address,
				MemoryType::LoaderData,
				(ent.p_memsz + 0xFFF) as usize / 0x1000,
				&mut addr
			).expect("Allocating pages for program segment") };
			
			// SAFE: This memory has just been allocated by the above
			let data_slice = unsafe { slice::from_raw_parts_mut(addr as usize as *mut u8, ent.p_memsz as usize) };
			kernel_file.set_position(ent.p_offset as u64);
			kernel_file.read( &mut data_slice[.. ent.p_filesz as usize] );
			for b in &mut data_slice[ent.p_filesz as usize .. ent.p_memsz as usize] {
				*b = 0;
			}
		}
	}
	// SAFE: Assuming that the executable is sane, and that it follows the correct calling convention
	Ok(unsafe { mem::transmute(elf_hdr.e_entry as usize) })
}

