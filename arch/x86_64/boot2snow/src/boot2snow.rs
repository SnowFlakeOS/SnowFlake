#[path="../../../../share/elf.rs"]
mod elf;

#[path="../../../../share/uefi_proto.rs"]
mod kernel_proto;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ptr;
use core::mem::size_of;
use orbclient::{Color, Renderer};
use uefi::reset::ResetType;
use uefi::status::{Error, Result, Status};
use uefi::boot::{BootServices, TimerDelay};
use uefi::Event;
use uefi::memory::{MemoryType, MemoryDescriptor, VirtualAddress};
use uefi::text::TextInputKey;

use conf::{Conf, load_conf};
use exec::exec_path;
use display::{Display, Output};
use fs::{File, Dir, find, load};
use image::{self, Image};
use io::{wait_key, wait_timeout};
use proto::Protocol;
use text::TextDisplay;
use vars::{
    get_boot_current,
    get_boot_next, set_boot_next,
    get_boot_item, set_boot_item,
    get_os_indications, set_os_indications,
    get_os_indications_supported
};

pub fn init() -> Result<()> {
    let uefi = unsafe { &mut *::UEFI };
    let handle = unsafe { ::HANDLE };
    let conf: Conf = load_conf();

    let (mut display, vid_addr) = {
        let output = Output::one()?;

        let mut max_i = 0;
        let mut max_w = 0;
        let mut max_h = 0;

        for i in 0..output.0.Mode.MaxMode {
            let mut mode_ptr = ::core::ptr::null_mut();
            let mut mode_size = 0;
            (output.0.QueryMode)(output.0, i, &mut mode_size, &mut mode_ptr)?;

            let mode = unsafe { &mut *mode_ptr };
            let w = mode.HorizontalResolution;
            let h = mode.VerticalResolution;
            if w >= max_w && h >= max_h {
                max_i = i;
                max_w = w;
                max_h = h;
            }
        }

        let _ = (output.0.SetMode)(output.0, max_i);
        let vid_addr = output.0.Mode.FrameBufferBase;

        (Display::new(output), vid_addr)
    };

    let mut splash = Image::new(0, 0);
    {
        if let Ok(data) = load("\\boot2snow\\only_logo.bmp") {
            if let Ok(image) = image::bmp::parse(&data) {
                splash = image;
            }
        }
    }

    {
        let bg = Color::rgb(0x00, 0x00, 0x00);

        display.set(bg);

        {
            let x = (display.width() as i32 - splash.width() as i32) / 2;
            let y = ((display.height() as i32 - splash.height() as i32) / 2) as i32 - 32;
            splash.draw(&mut display, x, y);
        }

        display.sync();

        status_msg(&mut display, splash.height(), concat!("Boot2Snow ", env!("CARGO_PKG_VERSION")));
    }
    
    let kernel_file = load_kernel_file().expect("Sorry, Unable to load kernel :(");

    wait_timeout(conf.boot_timeout);

    let (map, map_size, map_key, ent_size, ent_ver) = { 
        let mut map_size = 0;
		let mut map_key = 0;
		let mut ent_size = 0;
        let mut ent_ver = 0;

        unsafe { (uefi.BootServices.GetMemoryMap)(&mut map_size, ::core::ptr::null_mut(), &mut map_key, &mut ent_size, &mut ent_ver) };

        assert_eq!( ent_size, size_of::<::uefi::memory::MemoryDescriptor>() );
	    let mut map;

	    loop {
            map = uefi.BootServices.AllocatePoolVec( ::uefi::memory::MemoryType::EfiLoaderData, map_size / ent_size );
            match unsafe { (uefi.BootServices.GetMemoryMap)(&mut map_size, map.as_mut_ptr(), &mut map_key, &mut ent_size, &mut ent_ver) } {
				::uefi::status::Status(0) => break,
				::uefi::status::Status(5) => continue,
				e => panic!("Sorry, GetMemoryMap() Failed :( - {:?}", e)
            }
        }

        unsafe {
			map.set_len( map_size / ent_size );
        }

        (map, map_size, map_key, ent_size, ent_ver)
    };

    unsafe {
        match (uefi.BootServices.ExitBootServices)(handle, map_key) {
			e => panic!("Sorry, ExitBootServices Failed :( - {:?}", e)
        }
    };

    let mut map_tmp = unsafe { map.ptr.as_ptr() };

    unsafe {
        for i in 0..map.len {
            if (*map_tmp).Attribute | 0x8000000000000000 != 0 { // EFI_MEMORY_RUNTIME
                (*map_tmp).VirtualStart = VirtualAddress((*map_tmp).PhysicalStart.0);
            }

            map_tmp = ((map_tmp as u8) + size_of::<MemoryDescriptor>() as u8) as *mut MemoryDescriptor;
        }
    };
    
    unsafe {
        match  (uefi.RuntimeServices.SetVirtualAddressMap)(map_size, ent_size, ent_ver, map.ptr.as_ptr()) {
            e => panic!("Sorry, ExitBootServices Failed :( - {:?}", e)
        }
    }

    let boot_info = kernel_proto::Info {
		runtime_services: uefi.RuntimeServices as *const _ as *const (),

		cmdline_ptr: 1 as *const u8,
		cmdline_len: 0,
		
		map_addr: map.as_ptr() as usize as u64,
		map_entnum: map.len() as u32,
		map_entsz: size_of::<::uefi::memory::MemoryDescriptor>() as u32,

        vid_addr: vid_addr as u64,
        width: display.width(),
        height: display.height()
    };

    //kernel_file(0x71FF0EF1, &boot_info);

    Ok(())
}

type EntryPoint = extern "cdecl" fn(usize, *const kernel_proto::Info) -> !;
fn load_kernel_file() -> Result<EntryPoint> {
    let uefi = unsafe { &mut *::UEFI };
    let conf: Conf = load_conf();

	let mut kernel_file = match find(&conf.kernel) {
		Ok(k) => k,
		Err(e) => panic!("Failed to open kernel '{}' - {:?}", conf.kernel, e),
	};
    
	// Load kernel from this file (ELF).
	let elf_hdr = {
		let mut hdr = elf::ElfHeader::default();
		// SAFE: Converts to POD for read
		kernel_file.1.read( unsafe { ::core::slice::from_raw_parts_mut( &mut hdr as *mut _ as *mut u8, size_of::<elf::ElfHeader>() ) } ).expect("ElfHeader read");
		hdr
	};
    
	elf_hdr.check_header();
	for i in 0 .. elf_hdr.e_phnum
	{
		let mut ent = elf::PhEnt::default();
		kernel_file.1.set_position(elf_hdr.e_phoff as u64 + (i as usize * size_of::<elf::PhEnt>()) as u64 );
		// SAFE: Converts to POD for read
		kernel_file.1.read( unsafe { ::core::slice::from_raw_parts_mut( &mut ent as *mut _ as *mut u8, size_of::<elf::PhEnt>() ) } );

		if ent.p_type == 1 {
			println!("- {:#x}+{:#x} loads +{:#x}+{:#x}",
				ent.p_paddr, ent.p_memsz,
				ent.p_offset, ent.p_filesz
			);
			
			let mut addr = ent.p_paddr as usize;
			// SAFE: Correct call to FFI
			unsafe { (uefi.BootServices.AllocatePages)(
				::uefi::boot::AllocType::Address,
				::uefi::memory::MemoryType::EfiLoaderData,
				(ent.p_memsz + 0xFFF) as usize / 0x1000,
				&mut addr
			) };
			
			// SAFE: This memory has just been allocated by the above
			let data_slice = unsafe { ::core::slice::from_raw_parts_mut(ent.p_paddr as usize as *mut u8, ent.p_memsz as usize) };
			kernel_file.1.set_position(ent.p_offset as u64);
			kernel_file.1.read( &mut data_slice[.. ent.p_filesz as usize] );
			for b in &mut data_slice[ent.p_filesz as usize .. ent.p_memsz as usize] {
				*b = 0;
			}
		}
	}
	// SAFE: Assuming that the executable is sane, and that it follows the correct calling convention
	Ok(unsafe { ::core::mem::transmute(elf_hdr.e_entry as u64) })
}

fn status_msg(display: &mut Display, splash_height: u32, msg: &str) {
    let prompt = msg.clone();
    let mut x = (display.width() as i32 - prompt.len() as i32 * 8) / 2;
    let y = ((display.height() as i32 - splash_height as i32) / 2) as i32 + 256;

    let rect_x = 0;
    let rect_y = (y - 16);
    let rect_width = display.width();
    let rect_height = (y + 16) as u32;

    display.rect(rect_x, rect_y, rect_width, rect_height, Color::rgb(0x00, 0x00, 0x00));

    for c in prompt.chars() {
        display.char(x, y, c, Color::rgb(0xff, 0xff, 0xff));
        x += 8;
    }

    display.sync();
}
