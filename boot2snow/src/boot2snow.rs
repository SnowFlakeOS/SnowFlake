// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//! Some code was borrowed from [Tifflin Bootloader](https://github.com/thepowersgang/rust_os)

use core::{mem, slice, ptr};

use color::*;
use {EntryPoint, elf, kernel_proto};

use conf::Configuration;

use uefi::CStr16;
use uefi::status::*;
use uefi::boot_services::protocols;
use uefi::boot_services::{BootServices,
                          AllocateType,
                          MemoryDescriptor,
                          MemoryType};
use memory_map::{MM_BASE, memory_map};
use paging::paging;
use string::wstr;

static mut KERNEL_BASE: usize = 0;
static mut KERNEL_SIZE: usize = 0;
static STACK_BASE: usize = 0xFFFFFF0000080000;
static STACK_SIZE: usize = 0x1F000;

use {PATH_CONFIG,
     PATH_FALLBACK_KERNEL};
use {get_conout,
     get_boot_services,
     get_image_handle,
     get_runtime_services,
	 get_graphics_output};

pub extern fn init() -> Result<(), ()> {
    let boot_services = get_boot_services();
    let image_handle = get_image_handle();
    let runtime_services = get_runtime_services();
	let gop = get_graphics_output();

	{
		// Obtain the "LoadedImage" representing the bootloader, from which we get the boot volume
		let image_proto: &protocols::LoadedImage = boot_services.handle_protocol(&image_handle).expect("image_handle - LoadedImage");
		
		if image_proto.file_path.type_code() != (4,4) {
			panic!("Loader wans't loaded from a filesystem - type_code = {:?}", image_proto.file_path.type_code());
		}

		let system_volume_fs: &protocols::SimpleFileSystem = boot_services.handle_protocol(&image_proto.device_handle).expect("image_proto - FileProtocol");
		// - Get the root of this volume and load the bootloader configuration file from it
		let system_volume_root = system_volume_fs.open_volume().expect("system_volume_fs - File");
		let config = match Configuration::from_file(boot_services, &system_volume_root, PATH_CONFIG.into()) {
			Ok(c) => c,
			Err(e) => panic!("Failed to load config file: {:?}", e),
		};

		// - Load the kernel.
		let entrypoint = load_kernel_file(&system_volume_root, &config.kernel).expect("Unable to load kernel");
		let sections = load_kernel_sections(&system_volume_root, &config.kernel).expect("Unable to load sections");

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
			physbaseptr: gop.mode.frame_buffer_base as *mut Color,
			xresolution: unsafe { (*gop.mode.info).horizontal_resolution },
			yresolution: unsafe { (*gop.mode.info).vertical_resolution }
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

fn load_kernel_sections(sys_vol: &protocols::File, filename: &CStr16) -> Result<elf::PhEntIter<'static>, Status> {
	let mut kernel_file = match sys_vol.open_read(filename) {
		Ok(k) => k,
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

fn load_kernel_file(sys_vol: &protocols::File, filename: &CStr16) -> Result<EntryPoint, Status> {
    let boot_services = get_boot_services();

	let mut kernel_file = match sys_vol.open_read(filename) {
		Ok(k) => k,
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
		kernel_file.set_position(elf_hdr.e_phoff + (i as usize * mem::size_of::<elf::PhEnt>()) as u64).expect("PhEnt seek");
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
			kernel_file.set_position(ent.p_offset as u64).expect("seek segment");
			kernel_file.read( &mut data_slice[.. ent.p_filesz as usize] ).expect("read segment");
			for b in &mut data_slice[ent.p_filesz as usize .. ent.p_memsz as usize] {
				*b = 0;
			}
		}
	}
	// SAFE: Assuming that the executable is sane, and that it follows the correct calling convention
	Ok(unsafe { mem::transmute(elf_hdr.e_entry as usize) })
}

