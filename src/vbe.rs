// "Tifflin" Kernel
// - By John Hodge (thePowersGang)
//
// arch/amd64/boot.rs
//! Boot information.
//!
//! Parsing and exposure of the bootloader-provided data

pub unsafe fn load(vbe_addr: usize) -> &'static VbeModeInfo  {
    let vbe_info = VbeModeInfo::from_raw_parts(vbe_addr);
    vbe_info
}

#[repr(C)]
#[allow(unused)]
#[derive(Debug)]
pub struct VbeModeInfo {
	attributes: u16,
	window_attrs: [u8; 2],
	granuality: u16,
	window_size: u16,
	window_segments: [u16; 2],
	win_pos_fcn_fptr: [u16; 2],	// Pointer to INT 10h, AX=4F05h
	
	pitch: u16,
	x_res: u16, y_res: u16,
	char_w: u8, char_h: u8,
	n_planes: u8,
	bpp: u8,
	n_banks: u8,
	memory_model: u8,
	bank_size: u8,
	n_pages: u8,
	_resvd: u8,	// reserved
	
	// VBE 1.2+
	red_mask: u8,	red_position: u8,
	green_mask: u8, green_position: u8,
	blue_mask: u8,  blue_position: u8,
	rsv_mask: u8,   rsv_position: u8,
	directcolor_attributes: u8,

	// VBE v2.0+
	physbase: u32,
	offscreen_ptr: u32,	// Start of offscreen memory
	offscreen_size_kb: u16,	// Size of offscreen memory
	
	// -- VBE v3.0
	lfb_pitch: u16,
	image_count_banked: u8,
	image_count_lfb: u8,
}

impl VbeModeInfo 
{
    unsafe fn from_raw_parts(vbe_addr: usize) -> &'static VbeModeInfo {
        &*(vbe_addr as *const VbeModeInfo)
    }

	pub fn pitch(&self) -> u16 {
		self.pitch
	}

	pub fn x_res(&self) -> u16 {
		self.x_res
	}

	pub fn y_res(&self) -> u16 {
		self.y_res
	}
}