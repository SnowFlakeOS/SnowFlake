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
    get_os_indications_supported};

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

    let mut splash = Image::new(0, 0);
    {
        if let Ok(data) = load("\\boot2snow\\splash.bmp") {
            if let Ok(image) = image::bmp::parse(&data) {
                splash = image;
            }
        }
    }

    {
        let bg = Color::rgb(0x00, 0x00, 0x00);

        display.set(bg);

        {
            let x = (display.width() as i32 - splash.width() as i32) / 2;
            let y = ((display.height() as i32 - splash.height() as i32) / 2) as i32 - 16;
            splash.draw(&mut display, x, y);
        }

        display.sync();

        status_msg(&mut display, splash.height(), &format!("SnowKernel {} is loaded", env!("CARGO_PKG_VERSION")));
    }

    {
        let radius = -30 as i32;
        let mut x = (display.width() as i32) + (radius + (radius as f32 / 1.75) as i32);
        let mut y = (display.height() as i32) + (radius + (radius as f32 / 1.75) as i32);

        display.circle(x, y, radius, Color::rgb(0x61, 0x61, 0x61));
        display.arc(x, y, radius, 160, Color::rgb(0xff, 0xff, 0xff));
        display.circle(x, y, (radius as f32 / 1.075) as i32, Color::rgb(0x00, 0x00, 0x00));

        let prompt = "25%";

        x -= (prompt.len() * 4) as i32;
        y -= (prompt.len() as f32 * 2.5) as i32;

        for c in prompt.chars() {
            display.char(x, y, c, Color::rgb(0xff, 0xff, 0xff));
            x += 8;
        }

        display.sync();
    }

    Ok(())
}

fn status_msg(display: &mut Display, splash_height: u32, msg: &str) {
    let prompt = msg.clone();
    let mut x = (display.width() as i32 - prompt.len() as i32 * 8) / 2;
    let y = ((display.height() as i32 - splash_height as i32) / 2) as i32 + 256;
    
    let rect_x = 0;
    let rect_y = (y - 16);
    let rect_width = display.width();
    let rect_height = (y + 16) as u32;

    display.rect(rect_x, rect_y, rect_width, rect_height, Color::rgb(0x00, 0x00, 0x00));

    for c in prompt.chars() {
        display.char(x, y, c, Color::rgb(0xff, 0xff, 0xff));
        x += 8;
    }

    display.sync();
}