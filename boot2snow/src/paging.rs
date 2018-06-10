// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//! Some code was borrowed from [Redox OS Bootloader for EFI](https://github.com/redox-os/bootloader-efi)

use core::ptr;
use x86_64::PhysicalAddress;
use x86_64::registers::control_regs::*;
use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

static PT_BASE: u64 = 0x70000;

pub unsafe fn paging() {
    // Zero PML4, PDP, and 4 PD
    ptr::write_bytes(PT_BASE as *mut u8, 0, 6 * 4096);

    let mut base = PT_BASE;

    // Link first PML4 and second to last PML4 to PDP
    ptr::write(base as *mut u64, 0x71000 | 1 << 1 | 1);
    ptr::write((base + 510*8) as *mut u64, 0x71000 | 1 << 1 | 1);
    // Link last PML4 to PML4
    ptr::write((base + 511*8) as *mut u64, 0x70000 | 1 << 1 | 1);

    // Move to PDP
    base += 4096;

    // Link first four PDP to PD
    ptr::write(base as *mut u64, 0x72000 | 1 << 1 | 1);
    ptr::write((base + 8) as *mut u64, 0x73000 | 1 << 1 | 1);
    ptr::write((base + 16) as *mut u64, 0x74000 | 1 << 1 | 1);
    ptr::write((base + 24) as *mut u64, 0x75000 | 1 << 1 | 1);

    // Move to PD
    base += 4096;

    // Link all PD's (512 per PDP, 2MB each)
    let mut entry = 1 << 7 | 1 << 1 | 1;
    for i in 0..4*512 {
        ptr::write((base + i*8) as *mut u64, entry);
        entry += 0x200000;
    }

    // Enable FXSAVE/FXRSTOR, Page Global, Page Address Extension, and Page Size Extension
    let mut cr4 = cr4();
    cr4 |= Cr4::ENABLE_SSE | Cr4::ENABLE_GLOBAL_PAGES | Cr4::ENABLE_PAE | Cr4::ENABLE_PSE;
    cr4_write(cr4);

    // Enable Long mode and NX bit
    let mut efer = rdmsr(IA32_EFER);
    efer |= 1 << 11 | 1 << 8;
    wrmsr(IA32_EFER, efer);

    // Set new page map
    cr3_write(PhysicalAddress(PT_BASE));

    // Enable paging, write protect kernel, protected mode
    let mut cr0 = cr0();
    cr0 |= Cr0::ENABLE_PAGING | Cr0::WRITE_PROTECT | Cr0::PROTECTED_MODE;
    cr0_write(cr0);
}