//! Some code was borrowed from [Redox OS Kernel](https://gitlab.redox-os.org/redox-os/kernel)

use core::intrinsics::{volatile_load, volatile_store};
use raw_cpuid::CpuId;
use x86_64::registers::model_specific::Msr;

pub const IA32_APIC_BASE: u32 = 0x1b;
pub const IA32_X2APIC_SIVR: u32 = 0x80f;
pub const IA32_X2APIC_APICID: u32 = 0x802;
pub const IA32_X2APIC_VERSION: u32 = 0x803;
pub const IA32_X2APIC_EOI: u32 = 0x80b;
pub const IA32_X2APIC_ICR: u32 = 0x830;

pub static mut APIC: Apic = Apic {
    address: 0,
    x2: false,
};

pub unsafe fn init() {
    APIC.init();
}

pub unsafe fn init_ap() {
    APIC.init_ap();
}

pub struct Apic {
    pub address: usize,
    pub x2: bool
}

impl Apic {
    unsafe fn init(&mut self) {
        self.address = (Msr::new(IA32_APIC_BASE).read() as usize & 0xFFFF_0000) + ::KERNEL_BASE;
        self.x2 = CpuId::new().get_feature_info().unwrap().has_x2apic();
        self.init_ap();
    }

    unsafe fn init_ap(&mut self) {
        if self.x2 {
            Msr::new(IA32_APIC_BASE).write(Msr::new(IA32_APIC_BASE).read() | 1 << 10);
            Msr::new(IA32_X2APIC_SIVR).write(0x100);
        } else {
            self.write(0xF0, 0x100);
        }
    }

    unsafe fn read(&self, reg: u32) -> u32 {
        volatile_load((self.address + reg as usize) as *const u32)
    }

    unsafe fn write(&mut self, reg: u32, value: u32) {
        volatile_store((self.address + reg as usize) as *mut u32, value);
    }

    pub fn id(&self) -> u32 {
        if self.x2 {
            unsafe { Msr::new(IA32_X2APIC_APICID).read() as u32 }
        } else {
            unsafe { self.read(0x20) }
        }
    }

    pub fn version(&self) -> u32 {
        if self.x2 {
            unsafe { Msr::new(IA32_X2APIC_VERSION).read() as u32 }
        } else {
            unsafe { self.read(0x30) }
        }
    }

    pub fn icr(&self) -> u64 {
        if self.x2 {
            unsafe { Msr::new(IA32_X2APIC_ICR).read() }
        } else {
            unsafe {
                (self.read(0x310) as u64) << 32 | self.read(0x300) as u64
            }
        }
    }

    pub fn set_icr(&mut self, value: u64) {
        if self.x2 {
            unsafe { Msr::new(IA32_X2APIC_ICR).write(value); }
        } else {
            unsafe {
                while self.read(0x300) & 1 << 12 == 1 << 12 {}
                self.write(0x310, (value >> 32) as u32);
                self.write(0x300, value as u32);
                while self.read(0x300) & 1 << 12 == 1 << 12 {}
            }
        }
    }

    pub fn ipi(&mut self, apic_id: usize) {
        let mut icr = 0x4040;
        if self.x2 {
            icr |= (apic_id as u64) << 32;
        } else {
            icr |= (apic_id as u64) << 56;
        }
        self.set_icr(icr);
    }

    pub unsafe fn eoi(&mut self) {
        if self.x2 {
            Msr::new(IA32_X2APIC_EOI).write(0);
        } else {
            self.write(0xB0, 0);
        }
    }
}
