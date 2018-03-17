use core::{char, mem};
use core::fmt::{self, Write};
use uefi::{Status,
                Event,
                SimpleTextInput,
                SimpleTextOutput,
                get_system_table};
//use uefi::boot::TimerDelay;

pub struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, string: &str) -> Result<(), fmt::Error> {
        let _ = get_system_table().console().write(string);

        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
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
}

pub fn wait_timeout(timeout: u64) {
    let uefi = unsafe { &mut *::UEFI };

    let mut event: Event = unsafe { mem::zeroed() };
    unsafe { (uefi.BootServices.CreateEvent)(0x80000000, 0, None, ::core::ptr::null_mut(), &mut event);
                  (uefi.BootServices.SetTimer)(event, TimerDelay::Periodic, 10000) };

    let mut index = 0;
    let mut input = TextInputKey {
        ScanCode: 0,
        UnicodeChar: 0
    };

    for num in 0..(timeout * 100) {
        unsafe { (uefi.BootServices.WaitForEvent)(2, &[uefi.ConsoleIn.WaitForKey, event] as *const Event, &mut index) };
        
        if index == 0 {
            unsafe { (uefi.ConsoleIn.ReadKeyStroke)(uefi.ConsoleIn, &mut input) };
            match unsafe { char::from_u32_unchecked(input.UnicodeChar as u32) } {
                'r' => break,
                _ => continue
            }
        }
    }
}*/