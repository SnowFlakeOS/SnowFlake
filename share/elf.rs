// NOTE: A given executable may not use all of this
#![allow(dead_code)]
#![allow(non_camel_case_types)]

pub type Elf64_Half = u16;
pub type Elf64_Addr = u64;
pub type Elf64_Off = u64;
pub type Elf64_Sword = i32;
pub type Elf64_Word = u32;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ElfHeader {
	pub e_ident: [u8; 16],
	pub e_object_type: Elf64_Half,
	pub e_machine_type: Elf64_Half,
	pub e_version: Elf64_Word,

	pub e_entry: Elf64_Addr,
	pub e_phoff: Elf64_Off,
	pub e_shoff: Elf64_Off,

	pub e_flags: Elf64_Word,
	pub e_ehsize: Elf64_Half,

	pub e_phentsize: Elf64_Half,
	pub e_phnum: Elf64_Half,

	pub e_shentsize: Elf64_Half,
	pub e_shnum: Elf64_Half,
	pub e_shstrndx: Elf64_Half,
}

impl ElfHeader {
	pub fn check_header(&self) {
		assert_eq!(&self.e_ident[..8], b"\x7FELF\x02\x01\x01\x00");	// Elf64, LSB, Version, Pad
		assert_eq!(self.e_version, 1);
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct PhEnt {
	pub p_type: Elf64_Word,
	pub p_flags: Elf64_Word,
	pub p_offset: Elf64_Off,
	pub p_vaddr: Elf64_Addr,
	pub p_paddr: Elf64_Addr,	// aka load
	pub p_filesz: Elf64_Addr,
	pub p_memsz: Elf64_Addr,
	pub p_align: Elf64_Addr,
}

impl PhEnt {
    pub fn start_address(&self) -> usize {
        self.p_paddr as usize
    }

    pub fn end_address(&self) -> usize {
        (self.p_paddr + self.p_memsz) as usize
    }

    pub fn flags(&self) -> ElfSectionFlags {
        ElfSectionFlags::from_bits_truncate(self.p_flags.into())
    }

    pub fn is_allocated(&self) -> bool {
        self.flags().contains(ElfSectionFlags::ELF_SECTION_ALLOCATED)
	}
}

#[repr(C)]
#[derive(Copy,Clone)]
pub struct ShEnt {
	sh_name: Elf64_Word,
	sh_type: Elf64_Word,
	sh_flags: Elf64_Word,
	sh_addr: Elf64_Addr,
	sh_offset: Elf64_Off,
	sh_size: Elf64_Word,
	sh_link: Elf64_Word,
	sh_info: Elf64_Word,
	sh_addralign: Elf64_Word,
	sh_entsize: Elf64_Word,
}

#[derive(Copy,Clone)]
pub struct ElfFile(pub ElfHeader);
impl ElfFile
{
	pub fn check_header(&self) {
		self.0.check_header();
	}
	pub fn phents(&self) -> PhEntIter {
		assert_eq!( self.0.e_phentsize as usize, ::core::mem::size_of::<PhEnt>() );
		// SAFE: Assuming the file is correct...
		let slice: &[PhEnt] = unsafe {
			let ptr = (&self.0 as *const _ as usize + self.0.e_phoff as usize) as *const PhEnt;
			::core::slice::from_raw_parts( ptr, self.0.e_phnum as usize )
			};
		println!("phents() - slice = {:p}+{}", slice.as_ptr(), slice.len());
		PhEntIter( slice )
	}
	fn shents(&self) -> &[ShEnt] {
		assert_eq!( self.0.e_shentsize as usize, ::core::mem::size_of::<ShEnt>() );
		// SAFE: Assuming the file is correct...
		unsafe {
			let ptr = (&self.0 as *const _ as usize + self.0.e_shoff as usize) as *const ShEnt;
			::core::slice::from_raw_parts( ptr, self.0.e_shnum as usize )
		}
	}

	pub fn entrypoint(&self) -> usize {
		self.0.e_entry as usize
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct PhEntIter<'a>(pub &'a [PhEnt]);
impl<'a> Iterator for PhEntIter<'a> {
	type Item = PhEnt;
	fn next(&mut self) -> Option<PhEnt> {
		if self.0.len() == 0 {
			None
		}
		else {
			let rv = self.0[0].clone();
			self.0 = &self.0[1..];
			Some(rv)
		}
	}
}
struct ShEntIter<'a>(&'a [ShEnt]);
impl<'a> Iterator for ShEntIter<'a> {
	type Item = ShEnt;
	fn next(&mut self) -> Option<ShEnt> {
		if self.0.len() == 0 {
			None
		}
		else {
			let rv = self.0[0].clone();
			self.0 = &self.0[1..];
			Some(rv)
		}
	}
}

pub fn elf_get_size(file_base: &ElfFile) -> u32
{
	println!("elf_get_size(file_base={:p})", file_base);
	file_base.check_header();

	let mut max_end = 0;
	for phent in file_base.phents()
	{
		if phent.p_type == 1
		{
			println!("- {:#x}+{:#x} loads +{:#x}+{:#x}",
				phent.p_paddr, phent.p_memsz,
				phent.p_offset, phent.p_filesz
				);
			
			let end = (phent.p_paddr + phent.p_memsz) as usize;
			if max_end < end {
				max_end = end;
			}
		}
	}
	// Round the image size to 4KB
	let max_end = (max_end + 0xFFF) & !0xFFF;
	println!("return load_size={:#x}", max_end);
	max_end as u32
}

/// Returns program entry point
pub fn elf_load_segments(file_base: &ElfFile, output_base: *mut u8) -> u32
{
	println!("elf_load_segments(file_base={:p}, output_base={:p})", file_base, output_base);
	for phent in file_base.phents()
	{
		if phent.p_type == 1
		{
			println!("- {:#x}+{:#x} loads +{:#x}+{:#x}",
				phent.p_paddr, phent.p_memsz,
				phent.p_offset, phent.p_filesz
				);
			
			let (dst,src) = unsafe {
				let dst = ::core::slice::from_raw_parts_mut( (output_base as usize + phent.p_paddr as usize) as *mut u8, phent.p_memsz as usize );
				let src = ::core::slice::from_raw_parts( (file_base as *const _ as usize + phent.p_offset as usize) as *const u8, phent.p_filesz as usize );
				(dst, src)
				};
			for (d, v) in Iterator::zip( dst.iter_mut(), src.iter().cloned().chain(::core::iter::repeat(0)) )
			{
				*d = v;
			}
		}
	}
	

	let rv = (file_base.entrypoint() - 0x80000000 + output_base as usize) as u32;
	println!("return entrypoint={:#x}", rv);
	rv
}

#[derive(Copy,Clone,Debug)]
pub struct SymEnt {
	st_name: u32,
	st_value: u32,
	st_size: u32,
	st_info: u8,
	st_other: u8,
	st_shndx: u16,
}
#[repr(C)]
#[derive(Debug)]
pub struct SymbolInfo {
	base: *const SymEnt,
	count: usize,
	string_table: *const u8,
	strtab_len: usize,
}
/// Returns size of data written to output_base
pub extern "C" fn elf_load_symbols(file_base: &ElfFile, output: &mut SymbolInfo) -> u32
{
	println!("elf_load_symbols(file_base={:p}, output={:p})", file_base, output);
	*output = SymbolInfo {base: 0 as *const _, count: 0, string_table: 0 as *const _, strtab_len: 0};
	let mut pos = ::core::mem::size_of::<SymbolInfo>();
	for ent in file_base.shents()
	{
		if ent.sh_type == 2
		{
			println!("Symbol table at +{:#x}+{:#x}, string table {}", ent.sh_offset, ent.sh_size, ent.sh_link);
			let strtab = file_base.shents()[ent.sh_link as usize];
			//let strtab_bytes = unsafe { ::core::slice::from_raw_parts( (file_base as *const _ as usize + strtab.sh_offset as usize) as *const u8, strtab.sh_size as usize ) };
			//println!("- strtab = {:?}", ::core::str::from_utf8(strtab_bytes));

			output.base = (output as *const _ as usize + pos) as *const _;
			output.count = ent.sh_size as usize / ::core::mem::size_of::<SymEnt>();
			unsafe {
				let bytes = ent.sh_size as usize;
				let src = ::core::slice::from_raw_parts( (file_base as *const _ as usize + ent.sh_offset as usize) as *const SymEnt, output.count );
				let dst = ::core::slice::from_raw_parts_mut( output.base as *mut SymEnt, output.count );
				for (d,s) in Iterator::zip( dst.iter_mut(), src.iter() ) {
					//println!("- {:?} = {:#x}+{:#x}", ::core::str::from_utf8(&strtab_bytes[s.st_name as usize..].split(|&v|v==0).next().unwrap()), s.st_value, s.st_size);
					*d = *s;
				}
				pos += bytes;
			}
			output.string_table = (output as *const _ as usize + pos) as *const _;
			output.strtab_len = strtab.sh_size as usize;
			unsafe {
				let bytes = strtab.sh_size as usize;
				let src = ::core::slice::from_raw_parts( (file_base as *const _ as usize + strtab.sh_offset as usize) as *const u8, bytes );
				let dst = ::core::slice::from_raw_parts_mut( output.string_table as *mut u8, bytes );
				for (d,s) in Iterator::zip( dst.iter_mut(), src.iter() ) {
					*d = *s;
				}
				pos += bytes;
			}
			break ;
		}
	}

	println!("- output = {:?}", output);
	pos as u32
}

bitflags! {
    pub struct ElfSectionFlags: u64 {
        const ELF_SECTION_WRITABLE = 0x1;
        const ELF_SECTION_ALLOCATED = 0x2;
        const ELF_SECTION_EXECUTABLE = 0x4;
        // plus environment-specific use at 0x0F000000
        // plus processor-specific use at 0xF0000000
    }
}