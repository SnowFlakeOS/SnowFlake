#[path="../../../../share/elf.rs"]
mod elf;

#[path="../../../../share/uefi_proto.rs"]
mod kernel_proto;

use core::{mem, ptr};
use core::mem::size_of;
use core::fmt::Write;

use uefi::{SimpleTextOutput, 
                SimpleFileSystem,
                LoadedImageProtocol,
                AllocateType, 
                MemoryType,
                File,
                get_system_table,
                get_handle,
                lib_memory_map};

use uefi::graphics::{PixelFormat, Pixel, GraphicsOutputProtocol};

struct RGB{
    r:u8,
    g:u8,
    b:u8
}

impl RGB{
    fn new()->Self{
        Self{
            r:0,
            g:0,
            b:0
        }
    }

    fn hsv2rgb(&mut self,h:u8,s:u8,v:u8){
        let h = h as f64 /255.0;
        let s = s as f64 /255.0;
        let v = v as f64 /255.0;
        let mut r = v;
        let mut g = v;
        let mut b = v;

        let mut h=h;
        if s > 0.0 {
            h *= 6.0;
            let  i = h as u32;
            let f = h - (i as f64);
            match i{
                0=>{g *= 1.0 - s * (1.0 - f); b *= 1.0 - s;},
                1=>{r *= 1.0 - s * f; b *= 1.0 - s;},
                2=>{r *= 1.0 - s; b *= 1.0 - s * (1.0 - f);},
                3=>{r *= 1.0 - s;g *= 1.0 - s * f;},
                4=>{r *= 1.0 - s * (1.0 - f);g *= 1.0 - s;},
                5=>{g *= 1.0 - s;b *= 1.0 - s * f;},
                _=>{}
            }
        }
        self.r=(r*255.0) as u8;
        self.g=(g*255.0) as u8;
        self.b=(b*255.0) as u8;
    }
}

pub extern fn init() -> Result<(), ()> {
    let bs = get_system_table().boot_services();
    let rs = get_system_table().runtime_services();
    let gop = &mut GraphicsOutputProtocol::new().unwrap();
    let mode = unsafe { ::MODE };
    let tm = rs.get_time().unwrap();
    let info = gop.query_mode(mode).unwrap();
    let resolutin_w : usize = info.horizontal_resolution as usize;
    let resolutin_h : usize = info.vertical_resolution as usize;
    let AREA : usize = resolutin_w * resolutin_h;
    
    {
        let bitmap = bs.allocate_pool::<Pixel>(mem::size_of::<Pixel>() * AREA).unwrap();
        let mut c = RGB::new();
        c.hsv2rgb(255, 255, 255);
        let px = Pixel::new(c.r,c.g,c.b);
            
        let mut count = 0;
        while count < AREA {
            unsafe{
                *bitmap.offset(count as isize) = px.clone();
            };
            count += 1;
        }

        gop.draw(bitmap, resolutin_w/2-400, resolutin_h/2-300, resolutin_w, resolutin_h);
        bs.stall(100000);
    }

    let loaded_image: &LoadedImageProtocol = bs.handle_protocol::<LoadedImageProtocol>(::uefi::get_handle()).unwrap();

    let simple_fs = match bs.handle_protocol::<SimpleFileSystem>(&loaded_image.device_handle) {
        Ok(val) => val,
        Err(status) => panic!("Error! {}", status.str())
    };

    let simple_vol = simple_fs.open_volume().expect("Sorry, simple_fs.open_volume() Failed :(");

    //let kernel_file = load_kernel_file(simple_vol, "/boot2snow/kernel.bin");

    let (memory_map, memory_map_size, map_key, descriptor_size, descriptor_version) = ::uefi::lib_memory_map();
    let _ = bs.exit_boot_services(::uefi::get_handle(), &map_key);
    let _ = rs.set_virtual_address_map(&memory_map_size, &descriptor_size, &descriptor_version, memory_map);

    Ok(())
}

type EntryPoint = extern "cdecl" fn(usize, *const kernel_proto::Info) -> !;
fn load_kernel_file(simple_vol: &File, filename: &[u16]) -> Result<EntryPoint, ()> {
    let bs = get_system_table().boot_services();
    
    let mut kernel_file = match simple_vol.open_read(filename) {
		Ok(k) => k,
		Err(e) => panic!("Failed to open kernel '{:?}' - {:?}", filename, e),
    };
 
	// Load kernel from this file (ELF).
	let elf_hdr = {
		let mut hdr = elf::ElfHeader::default();
		// SAFE: Converts to POD for read
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
			for b in &mut data_slice[ent.p_filesz as usize .. ent.p_memsz as usize] {
				*b = 0;
			}
		}
	}
	// SAFE: Assuming that the executable is sane, and that it follows the correct calling convention
	Ok(unsafe { ::core::mem::transmute(elf_hdr.e_entry as u64) })
}

