// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//! Some code was borrowed from [System76 Firmware Update](https://github.com/system76/firmware-update)

use core::{char, mem, ptr};
use core::fmt::{self, Write};
use uefi::{Void, InputKey, SimpleInputInterface, EfiLogger};
use uefi::boot_services::{Event, TimerDelay};

use get_conin;
use get_conout;
use get_boot_services;

pub fn _print(args: fmt::Arguments) {
    EfiLogger(get_conout()).write_fmt(args).unwrap();
}

/*pub fn wait_key() -> Result<char, status::Error> {
    let uefi = unsafe { &mut *::UEFI };

    let mut index = 0;
    (uefi.BootServices.WaitForEvent)(1, &uefi.ConsoleIn.WaitForKey, &mut index)?;

    let mut input = TextInputKey {
        ScanCode: 0,
        UnicodeChar: 0
    };

    (uefi.ConsoleIn.ReadKeyStroke)(uefi.ConsoleIn, &mut input)?;

    Ok(unsafe {
        char::from_u32_unchecked(input.UnicodeChar as u32)
    })
}*/

pub fn wait_timeout(timeout: u64) {
    let boot_services = get_boot_services();
    let conin = get_conin();

    let mut event: *mut Void = unsafe { ptr::null_mut() };
    unsafe { (boot_services.create_event)(0x80000000, 0, None, ptr::null_mut(), &mut event);
            (boot_services.set_timer)(event, TimerDelay::Periodic, 10000) };

    let mut index = 0;
    let mut input = InputKey {
        scan_code: 0,
        unicode_char: 0
    };

    for num in 0..(timeout * 100) {
        unsafe { (boot_services.wait_for_event)(2, &[conin.wait_for_key, event] as *const *mut Void, &mut index) };
        
        if index == 0 {
            input = conin.read_key_stroke().unwrap();
            match unsafe { char::from_u32_unchecked(input.unicode_char as u32) } {
                'r' => break,
                _ => continue
            }
        }
    }
}