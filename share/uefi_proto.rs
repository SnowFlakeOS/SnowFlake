mod color;
mod elf;
use color::*;
use elf::PhEntIter;

#[repr(C)]
pub struct Info {
	pub runtime_services: *const (),

	pub cmdline_ptr: *const u8,
	pub cmdline_len: usize,

	pub elf_sections: Option<PhEntIter<'static>>,
	pub kernel_base: usize,
	pub kernel_size: usize,
	pub stack_base: usize,
	pub stack_size: usize,

	pub map_addr: u64,
	pub map_len: u32,
	pub descriptor_size: u32,

	pub video_info: *const VideoInfo,
}

#[repr(C)]
pub struct VideoInfo {
	pub physbaseptr: *mut Color,
	pub xresolution: u32,
	pub yresolution: u32
}

// TODO: Grab this from libuefi
#[repr(C)]
#[derive(Copy,Clone)]
pub struct MemoryDescriptor
{
	pub ty: u32,
	_pad: u32,
	pub physical_start: u64,
	pub virtual_start: u64,
	pub number_of_pages: u64,
	pub attribute: u64,
	_pad2: u64,
}