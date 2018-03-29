use core::mem::size_of;

use color::*;
use {EntryPoint, elf, kernel_proto};

use conf::Configuration;

use uefi::status::*;
use uefi::boot_services::protocols;
use uefi::boot_services::{BootServices,
                          AllocateType,
                          MemoryDescriptor,
                          MemoryType};

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
		let entrypoint = load_kernel_file(boot_services, &system_volume_root, &config.kernel).expect("Unable to load kernel");

		// Save memory map
		let (map_key, map) = {
			let mut map_size = 0;
			let mut map_key = 0;
			let mut ent_size = 0;
			let mut ent_ver = 0;
			match unsafe { (boot_services.get_memory_map)(&mut map_size, ::core::ptr::null_mut(), &mut map_key, &mut ent_size, &mut ent_ver) }
			{
			    SUCCESS => {},
			    BUFFER_TOO_SMALL => {},
			    e => panic!("Sorry, get_memory_map() Failed :( - {:?}", e),
			}

			assert_eq!( ent_size, size_of::<MemoryDescriptor>() );
			let mut map;
			loop
			{
				map = boot_services.allocate_pool_vec( MemoryType::LoaderData, map_size / ent_size ).unwrap();
				match unsafe { (boot_services.get_memory_map)(&mut map_size, map.as_mut_ptr(), &mut map_key, &mut ent_size, &mut ent_ver) }
				{
				    SUCCESS => break,
				    BUFFER_TOO_SMALL => continue,
				    e => panic!("get_memory_map 2 - {:?}", e),
				}
			}
			unsafe {
				map.set_len( map_size / ent_size );
			}

			(map_key, map)
		};

		unsafe { 
			(boot_services.exit_boot_services)(image_handle, map_key).expect("exit_boot_services");
		}

		let boot_info = kernel_proto::Info {
			runtime_services: runtime_services as *const _ as *const (),
			
			// TODO: Get from the configuration
			cmdline_ptr: 1 as *const u8,
			cmdline_len: 0,
			
<<<<<<< HEAD
			map_addr: map.as_ptr() as usize as u64,
			map_entnum: map.len() as u32,
			map_entsz: size_of::<MemoryDescriptor>() as u32,

            vid_addr: gop.mode.frame_buffer_base as *mut Color,
            width: unsafe { (*gop.mode.info).horizontal_resolution },
            height: unsafe { (*gop.mode.info).vertical_resolution }
		};
		
		// - Execute kernel (passing a magic value and general boot information)
		entrypoint(0x71FF0EF1, &boot_info);
    }

	Ok(())
}

fn load_kernel_file(boot_services: &::uefi::boot_services::BootServices, sys_vol: &protocols::File, filename: &::uefi::CStr16) -> Result<EntryPoint, ::uefi::Status>
{
	let mut kernel_file = match sys_vol.open_read(filename) {
		Ok(k) => k,
		Err(e) => panic!("Failed to open kernel '{}' - {:?}", filename, e),
	};

=======
			map_addr: /* memory_map */ 0 as usize as u64,
			map_entnum: (memory_map_size / descriptor_size) as u32,
			map_entsz: size_of::<MemoryDescriptor>() as u32,

            vid_addr: bitmap as usize as u64,
            width: resolutin_w,
            height: resolutin_h,
    };

    kernel_file(0x71FF0EF1, &boot_info);
    Ok(())
}

type EntryPoint = extern "cdecl" fn(usize, *const kernel_proto::Info) -> !;
fn load_kernel_file(simple_vol: &File, filename: &[u16]) -> Result<EntryPoint, ()> {
    let bs = get_system_table().boot_services();
    
    let mut kernel_file = match simple_vol.open_read(filename) {
		Ok(k) => k,
		Err(e) => panic!("Failed to open kernel '{:?}' - {:?}", filename, e),
    };
 
>>>>>>> 54b2b70a2ee02e4ffea5b8eb418a797a006e9595
	// Load kernel from this file (ELF).
	let elf_hdr = {
		let mut hdr = elf::ElfHeader::default();
		// SAFE: Converts to POD for read
<<<<<<< HEAD
		kernel_file.read( unsafe { ::core::slice::from_raw_parts_mut( &mut hdr as *mut _ as *mut u8, size_of::<elf::ElfHeader>() ) } ).expect("ElfHeader read");
		hdr
	};

	elf_hdr.check_header();
	for i in 0 .. elf_hdr.e_phnum
	{
		let mut ent = elf::PhEnt::default();
		kernel_file.set_position(elf_hdr.e_phoff as u64 + (i as usize * size_of::<elf::PhEnt>()) as u64 ).expect("PhEnt seek");
		// SAFE: Converts to POD for read
		kernel_file.read( unsafe { ::core::slice::from_raw_parts_mut( &mut ent as *mut _ as *mut u8, size_of::<elf::PhEnt>() ) } ).expect("PhEnt read");
		
		if ent.p_type == 1
		{
			println!("- {:#x}+{:#x} loads +{:#x}+{:#x}",
				ent.p_paddr, ent.p_memsz,
				ent.p_offset, ent.p_filesz
				);
			
			let mut addr = ent.p_paddr as u64;
			// SAFE: Correct call to FFI
			unsafe { (boot_services.allocate_pages)(
				AllocateType::Address,
				MemoryType::LoaderData,
				(ent.p_memsz + 0xFFF) as usize / 0x1000,

				&mut addr
			).expect("Allocating pages for program segment") };
			
			// SAFE: This memory has just been allocated by the above
			let data_slice = unsafe { ::core::slice::from_raw_parts_mut(ent.p_paddr as usize as *mut u8, ent.p_memsz as usize) };
			kernel_file.set_position(ent.p_offset as u64).expect("seek segment");
			kernel_file.read( &mut data_slice[.. ent.p_filesz as usize] ).expect("read segment");
=======
		let _ = kernel_file.read( unsafe { ::core::slice::from_raw_parts_mut( &mut hdr as *mut _ as *mut u8, size_of::<elf::ElfHeader>() ) } ).expect("Fail to read ElfHeader :(");
		hdr
	};
    
	elf_hdr.check_header();
	for i in 0 .. elf_hdr.e_phnum {
		let mut ent = elf::PhEnt::default();
		let _ = kernel_file.set_position(elf_hdr.e_phoff as u64 + (i as usize * size_of::<elf::PhEnt>()) as u64 ).unwrap();
		// SAFE: Converts to POD for read
		let _ = kernel_file.read( unsafe { ::core::slice::from_raw_parts_mut( &mut ent as *mut _ as *mut u8, size_of::<elf::PhEnt>() ) } ).expect("Fail to read Kernel :(");

		if ent.p_type == 1 {
            println!("- {:#x}+{:#x} loads +{:#x}+{:#x}",
				ent.p_vaddr, ent.p_memsz,
				ent.p_offset, ent.p_filesz
            );

			let mut addr = ent.p_vaddr as u64;
			// SAFE: Correct call to FFI
			let _ = bs.allocate_pages(
				            AllocateType::Address,
				            MemoryType::LoaderData,
				            (ent.p_memsz + 0xFFF) as usize / 0x1000,
				            &mut addr);
			
			// SAFE: This memory has just been allocated by the above
			let data_slice = unsafe { ::core::slice::from_raw_parts_mut(ent.p_vaddr as usize as *mut u8, ent.p_memsz as usize) };
			let _ = kernel_file.set_position(ent.p_offset as u64);
			let _ = kernel_file.read( &mut data_slice[.. ent.p_filesz as usize] );
>>>>>>> 54b2b70a2ee02e4ffea5b8eb418a797a006e9595
			for b in &mut data_slice[ent.p_filesz as usize .. ent.p_memsz as usize] {
				*b = 0;
			}
		}
	}
	// SAFE: Assuming that the executable is sane, and that it follows the correct calling convention
<<<<<<< HEAD
	Ok(unsafe { ::core::mem::transmute(elf_hdr.e_entry as usize) })
=======
	Ok(unsafe { ::core::mem::transmute(elf_hdr.e_entry as u64) })
>>>>>>> 54b2b70a2ee02e4ffea5b8eb418a797a006e9595
}

