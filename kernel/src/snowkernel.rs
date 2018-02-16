use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ptr;
use core::mem::size_of;
use orbclient::{Color, Renderer};
use uefi::reset::ResetType;
use uefi::status::{Error, Result, Status};
use uefi::boot::BootServices;

use display::{Display, Output};
use fs::{File, Dir, find, load};
use image::{self, Image};
use io::wait_key;
use proto::Protocol;
use text::TextDisplay;
use vars::{
    get_boot_current,
    get_boot_next, set_boot_next,
    get_boot_item, set_boot_item,
    get_os_indications, set_os_indications,
    get_os_indications_supported
};

pub fn init() -> Result<()> {
    let uefi = unsafe { &mut *::UEFI };
    let handle = unsafe { ::HANDLE };

    let mut display = {
        let output = Output::one()?;

        let mut max_i = 0;
        let mut max_w = 0;
        let mut max_h = 0;

        for i in 0..output.0.Mode.MaxMode {
            let mut mode_ptr = ::core::ptr::null_mut();
            let mut mode_size = 0;
            (output.0.QueryMode)(output.0, i, &mut mode_size, &mut mode_ptr)?;

            let mode = unsafe { &mut *mode_ptr };
            let w = mode.HorizontalResolution;
            let h = mode.VerticalResolution;
            if w >= max_w && h >= max_h {
                max_i = i;
                max_w = w;
                max_h = h;
            }
        }

        let _ = (output.0.SetMode)(output.0, max_i);

        Display::new(output)
    };

    let mut only_logo = Image::new(0, 0);
    {
        if let Ok(data) = load("\\boot2snow\\only_logo.bmp") {
            if let Ok(image) = image::bmp::parse(&data) {
                only_logo = image;
            }
        }
    }

    let mut full_logo = Image::new(0, 0);
    {
        if let Ok(data) = load("\\boot2snow\\full_logo.bmp") {
            if let Ok(image) = image::bmp::parse(&data) {
                full_logo = image;
            }
        }
    }

    {
        let x = (display.width() as i32 - only_logo.width() as i32) / 2;
        let y = ((display.height() as i32 - only_logo.height() as i32) / 2) as i32 - 32;
        only_logo.draw(&mut display, x, y);

        display.sync();
    }

    update_progressbar(&mut display, only_logo.height(), 5, 50);

    {
        let x = (display.width() as i32 - full_logo.width() as i32) / 2;
        let y = ((display.height() as i32 - full_logo.height() as i32) / 2) as i32 - 32;
        full_logo.draw(&mut display, x, y);

        display.sync();
    }

    update_progressbar(&mut display, only_logo.height(), 50, 100);

    Ok(())
}

fn update_progressbar(display: &mut Display, splash_height: u32, start_progress: u8, stop_progress: u8) {
    // TODO: Animation here
    let progress = start_progress;

    for progress in start_progress..stop_progress {
        _progressbar(display, splash_height, progress);
    }
}

fn _progressbar(display: &mut Display, splash_height: u32, progress: u8) {
    let width = display.width() as f32 / 3.5;

    let rect_width = ((width / 100.0) * progress as f32) as u32;
    let rect_height = 10 as u32;

    let x = (display.width() as i32 / 2) - width as i32 / 2;
    let y = ((display.height() as i32 + splash_height as i32) / 2) as i32;

    display.rounded_rect(x, y, rect_width, rect_height, 3, true, Color::rgb(0xff, 0xff, 0xff));

    display.sync();
}
