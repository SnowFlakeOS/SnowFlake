// "Tifflin" Kernel
// - By John Hodge (thePowersGang)
//
// arch/amd64/boot.rs
//! Boot information.
//!
//! Parsing and exposure of the bootloader-provided data

pub unsafe fn load(vbe_addr: usize) -> &'static VBEModeInfo  {
    let vbe_info = VBEModeInfo::from_raw_parts(vbe_addr);
    vbe_info
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(packed)]
pub struct VBEModeInfo {
    attributes: u16,
    win_a: u8,
    win_b: u8,
    granularity: u16,
    winsize: u16,
    segment_a: u16,
    segment_b: u16,
    winfuncptr: u32,
    bytesperscanline: u16,
    pub xresolution: u16,
    pub yresolution: u16,
    xcharsize: u8,
    ycharsize: u8,
    numberofplanes: u8,
    bitsperpixel: u8,
    numberofbanks: u8,
    memorymodel: u8,
    banksize: u8,
    numberofimagepages: u8,
    unused: u8,
    redmasksize: u8,
    redfieldposition: u8,
    greenmasksize: u8,
    greenfieldposition: u8,
    bluemasksize: u8,
    bluefieldposition: u8,
    rsvdmasksize: u8,
    rsvdfieldposition: u8,
    directcolormodeinfo: u8,
    pub physbaseptr: u32,
    offscreenmemoryoffset: u32,
    offscreenmemsize: u16,
}

impl VBEModeInfo {
    unsafe fn from_raw_parts(vbe_addr: usize) -> &'static VBEModeInfo {
        &*(vbe_addr as *const VBEModeInfo)
    }
}
