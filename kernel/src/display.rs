use alloc::boxed::Box;
use core::cmp;
use core::ops::Try;
use orbclient::{Color, Renderer};
use uefi::graphics::{GraphicsOutput, GraphicsBltOp, GraphicsBltPixel};
use uefi::guid::{Guid, GRAPHICS_OUTPUT_PROTOCOL_GUID};

use proto::Protocol;

pub struct Output(pub &'static mut GraphicsOutput);

impl Protocol<GraphicsOutput> for Output {
    fn guid() -> Guid {
        GRAPHICS_OUTPUT_PROTOCOL_GUID
    }

    fn new(inner: &'static mut GraphicsOutput) -> Self {
        Output(inner)
    }
}

pub struct Display {
    output: Output,
    scale: u32,
    w: u32,
    h: u32,
    data: Box<[Color]>,
    font: &'static [u8],
}

impl Display {
    pub fn new(output: Output) -> Self {
        let w = output.0.Mode.Info.HorizontalResolution;
        let h = output.0.Mode.Info.VerticalResolution;
        let scale = if h > 1440 {
            2
        } else {
            1
        };
        Self {
            output: output,
            scale: scale,
            w: w,
            h: h,
            data: vec![Color::rgb(0, 0, 0); w as usize * h as usize].into_boxed_slice(),
            font: include_bytes!("../../res/unifont.font"),
        }
    }

    pub fn scale(&self) -> u32 {
        self.scale
    }

    pub fn scroll(&mut self, rows: usize, color: Color) {
        let scale = self.scale as usize;
        self.inner_scroll(rows * scale, color);
    }

    pub fn blit(&mut self, x: i32, y: i32, w: u32, h: u32) -> bool {
        let scale = self.scale;
        self.inner_blit(
            x * scale as i32,
            y * scale as i32,
            w * scale,
            h * scale
        )
    }

    fn inner_blit(&mut self, x: i32, y: i32, w: u32, h: u32) -> bool {
        let status = (self.output.0.Blt)(
            self.output.0,
            self.data.as_mut_ptr() as *mut GraphicsBltPixel,
            GraphicsBltOp::BufferToVideo,
            x as usize,
            y as usize,
            x as usize,
            y as usize,
            w as usize,
            h as usize,
            0
        );
        status.into_result().is_ok()
    }

    fn inner_scroll(&mut self, rows: usize, color: Color) {
        let width = self.w as usize;
        let height = self.h as usize;
        if rows > 0 && rows < height {
            let off1 = rows * width;
            let off2 = height * width - off1;
            unsafe {
                let data_ptr = self.data.as_mut_ptr() as *mut u32;
                fast_copy(data_ptr as *mut u8, data_ptr.offset(off1 as isize) as *const u8, off2 as usize * 4);
                fast_set32(data_ptr.offset(off2 as isize), color.0, off1 as usize);
            }
        }
    }

    fn inner_pixel(&mut self, x: i32, y: i32, color: Color) {
        let w = self.w;
        let h = self.h;

        if x >= 0 && y >= 0 && x < w as i32 && y < h as i32 {
            let new = color.0;

            let alpha = (new >> 24) & 0xFF;
            if alpha > 0 {
                let old = &mut self.data[y as usize * w as usize + x as usize];
                if alpha >= 255 {
                    old.0 = new;
                } else {
                    let n_r = (((new >> 16) & 0xFF) * alpha) >> 8;
                    let n_g = (((new >> 8) & 0xFF) * alpha) >> 8;
                    let n_b = ((new & 0xFF) * alpha) >> 8;

                    let n_alpha = 255 - alpha;
                    let o_a = (((old.0 >> 24) & 0xFF) * n_alpha) >> 8;
                    let o_r = (((old.0 >> 16) & 0xFF) * n_alpha) >> 8;
                    let o_g = (((old.0 >> 8) & 0xFF) * n_alpha) >> 8;
                    let o_b = ((old.0 & 0xFF) * n_alpha) >> 8;

                    old.0 = ((o_a << 24) | (o_r << 16) | (o_g << 8) | o_b) + ((alpha << 24) | (n_r << 16) | (n_g << 8) | n_b);
                }
            }
        }
    }

    fn inner_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
        let self_w = self.w;
        let self_h = self.h;

        let start_y = cmp::max(0, cmp::min(self_h as i32 - 1, y));
        let end_y = cmp::max(start_y, cmp::min(self_h as i32, y + h as i32));

        let start_x = cmp::max(0, cmp::min(self_w as i32 - 1, x));
        let len = cmp::max(start_x, cmp::min(self_w as i32, x + w as i32)) - start_x;

        let alpha = (color.0 >> 24) & 0xFF;
        if alpha > 0 {
            if alpha >= 255 {
                for y in start_y..end_y {
                    unsafe {
                        fast_set32(self.data.as_mut_ptr().offset((y * self_w as i32 + start_x) as isize) as *mut u32, color.0, len as usize);
                    }
                }
            } else {
                for y in start_y..end_y {
                    for x in start_x..start_x + len {
                        self.inner_pixel(x, y, color);
                    }
                }
            }
        }
    }

    /// Draw a line
    fn inner_line(&mut self, argx1: i32, argy1: i32, argx2: i32, argy2: i32, color: Color) {
        let mut x = argx1;
        let mut y = argy1;

        let dx = if argx1 > argx2 { argx1 - argx2 } else { argx2 - argx1 };
        let dy = if argy1 > argy2 { argy1 - argy2 } else { argy2 - argy1 };

        let sx = if argx1 < argx2 { 1 } else { -1 };
        let sy = if argy1 < argy2 { 1 } else { -1 };

        let mut err = if dx > dy { dx } else {-dy} / 2;
        let mut err_tolerance;

        loop {
            self.inner_pixel(x, y, color);

            if x == argx2 && y == argy2 { break };

            err_tolerance = 2 * err;

            if err_tolerance > -dx { err -= dy; x += sx; }
            if err_tolerance < dy { err += dx; y += sy; }
        }
    }

    /// Draw a circle. Negative radius will fill in the inside
    pub fn inner_circle(&mut self, x0: i32, y0: i32, radius: i32, filled: bool, color: Color) {
        let mut x = 1;
        let mut y = radius.abs();
        let mut distance = 0;
        let mut err = 0;

        if filled {
            self.inner_line(0 + x0, radius + y0, 0 + x0, -radius + y0, color);
            self.inner_line(radius + x0, 0 + y0, -radius + x0, 0 + y0, color);
        } else {
            self.inner_pixel(0 + x0, radius + y0, color);
            self.inner_pixel(0 + x0, -radius + y0, color);
            self.inner_pixel(radius + x0, 0 + y0, color);
            self.inner_pixel(-radius + x0, 0 + y0, color);
        }

        distance = -radius;

        while x <= y {
            distance += (x << 1) - 1;

            if distance >= 0 {
                y -= 1;

                distance += (-y << 1) + 2;
            }

            if filled {
                self.inner_line(-x + x0, y + y0, x + x0, y + y0, color);
                self.inner_line(-x + x0, -y + y0, x + x0, -y + y0, color);
                self.inner_line(-y + x0, x + y0, y + x0, x + y0, color);
                self.inner_line(-y + x0, -x + y0, y + x0, -x + y0, color);
            } else {
                self.inner_pixel(x + x0, y + y0, color);
                self.inner_pixel(x + x0, -y + y0, color);
                self.inner_pixel(-x + x0, y + y0, color);
                self.inner_pixel(-x + x0, -y + y0, color);
                self.inner_pixel(y + x0, x + y0, color);
                self.inner_pixel(y + x0, -x + y0, color);
                self.inner_pixel(-y + x0, x + y0, color);
                self.inner_pixel(-y + x0, -x + y0, color);
            }

            x += 1;
        }
    }
}

impl Renderer for Display {
    /// Get the width of the image in pixels
    fn width(&self) -> u32 {
        self.w/self.scale
    }

    /// Get the height of the image in pixels
    fn height(&self) -> u32 {
        self.h/self.scale
    }

    /// Return a reference to a slice of colors making up the image
    fn data(&self) -> &[Color] {
        &self.data
    }

    /// Return a mutable reference to a slice of colors making up the image
    fn data_mut(&mut self) -> &mut [Color] {
        &mut self.data
    }

    fn sync(&mut self) -> bool {
        let w = self.width();
        let h = self.height();
        self.blit(0, 0, w, h)
    }

    fn pixel(&mut self, x: i32, y: i32, color: Color) {
        self.rect(x, y, 1, 1, color);
    }

    /// Draw a character, using the loaded font
    fn char(&mut self, x: i32, y: i32, c: char, color: Color) {
        let mut offset = (c as usize) * 16;
        for row in 0..16 {
            let row_data;
            if offset < self.font.len() {
                row_data = self.font[offset];
            } else {
                row_data = 0;
            }

            for col in 0..8 {
                let pixel = (row_data >> (7 - col)) & 1;
                if pixel > 0 {
                    self.pixel(x + col as i32, y + row as i32, color);
                }
            }
            offset += 1;
        }
    }

    fn rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
        let scale = self.scale;
        self.inner_rect(
            x * scale as i32,
            y * scale as i32,
            w * scale,
            h * scale,
            color
        );
    }

    fn set(&mut self, color: Color) {
        let w = self.width();
        let h = self.height();
        self.rect(0, 0, w, h, color);
    }

    /// Draw a line
    fn line(&mut self, argx1: i32, argy1: i32, argx2: i32, argy2: i32, color: Color) {
        let mut x = argx1;
        let mut y = argy1;

        let dx = if argx1 > argx2 { argx1 - argx2 } else { argx2 - argx1 };
        let dy = if argy1 > argy2 { argy1 - argy2 } else { argy2 - argy1 };

        let sx = if argx1 < argx2 { 1 } else { -1 };
        let sy = if argy1 < argy2 { 1 } else { -1 };

        let mut err = if dx > dy { dx } else {-dy} / 2;
        let mut err_tolerance;

        loop {
            self.pixel(x, y, color);

            if x == argx2 && y == argy2 { break };

            err_tolerance = 2 * err;

            if err_tolerance > -dx { err -= dy; x += sx; }
            if err_tolerance < dy { err += dx; y += sy; }
        }
    }

    fn lines(&mut self, points: &[[i32; 2]], color: Color) {
        if points.len() == 0 {
            // when no points given, do nothing
        } else if points.len() == 1 {
            self.pixel(points[0][0], points[0][1], color);
        } else {
            for i in 0..points.len() - 1 {
                self.line(points[i][0], points[i][1], points[i+1][0], points[i+1][1], color);
            }
        }
    }

        /// Draw a rect with rounded corners
    fn rounded_rect(&mut self, x: i32, y: i32, w: u32, h: u32, radius: u32, filled: bool, color: Color) {
        let w = w as i32;
        let h = h as i32;
        let r = radius as i32;


        if filled {
            //Draw inside corners
            self.arc(x + r, y + r, -r, 1 << 4 | 1 << 6, color);
            self.arc(x + w - 1 - r, y + r, -r, 1 << 5 | 1 << 7, color);
            self.arc(x + r, y + h - 1 - r,- r, 1 << 0 | 1 << 2, color);
            self.arc(x + w - 1 - r, y + h - 1 - r, -r, 1 << 1 | 1 << 3, color);

            // Draw inside rectangles
            self.rect(x + r, y, (w - 1 - r * 2) as u32, r as u32 + 1, color);
            self.rect(x + r, y + h - 1 - r, (w - 1 - r * 2) as u32, r as u32 + 1, color);
            self.rect(x, y + r + 1, w as u32, (h - 2 - r * 2) as u32, color);
        } else {
            //Draw outside corners
            self.arc(x + r, y + r, r, 1 << 4 | 1 << 6, color);
            self.arc(x + w - 1 - r, y + r, r, 1 << 5 | 1 << 7, color);
            self.arc(x + r, y + h - 1 - r, r, 1 << 0 | 1 << 2, color);
            self.arc(x + w - 1 - r, y + h - 1 - r, r, 1 << 1 | 1 << 3, color);

            // Draw outside rectangles
            self.rect(x + r + 1, y, (w - 2 - r * 2) as u32, 1, color);
            self.rect(x + r + 1, y + h - 1, (w - 2 - r * 2) as u32, 1, color);
            self.rect(x, y + r + 1, 1, (h - 2 - r * 2) as u32, color);
            self.rect(x + w - 1, y + r + 1, 1, (h - 2 - r * 2) as u32, color);
        }
    }

    /// Draw a piece of an arc. Negative radius will fill in the inside
    fn arc(&mut self, x0: i32, y0: i32, radius: i32, parts: u8, color: Color) {
        let mut x = radius.abs();
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            if radius < 0 {
                if parts & 1 << 0 != 0 { self.rect(x0 - x, y0 + y, x as u32, 1, color); }
                if parts & 1 << 1 != 0 { self.rect(x0, y0 + y, x as u32 + 1, 1, color); }
                if parts & 1 << 2 != 0 { self.rect(x0 - y, y0 + x, y as u32, 1, color); }
                if parts & 1 << 3 != 0 { self.rect(x0, y0 + x, y as u32 + 1, 1, color); }
                if parts & 1 << 4 != 0 { self.rect(x0 - x, y0 - y, x as u32, 1, color); }
                if parts & 1 << 5 != 0 { self.rect(x0, y0 - y, x as u32 + 1, 1, color); }
                if parts & 1 << 6 != 0 { self.rect(x0 - y, y0 - x, y as u32, 1, color); }
                if parts & 1 << 7 != 0 { self.rect(x0, y0 - x, y as u32 + 1, 1, color); }
            } else if radius == 0 {
                self.pixel(x0, y0, color);
            } else {
                if parts & 1 << 0 != 0 { self.pixel(x0 - x, y0 + y, color); }
                if parts & 1 << 1 != 0 { self.pixel(x0 + x, y0 + y, color); }
                if parts & 1 << 2 != 0 { self.pixel(x0 - y, y0 + x, color); }
                if parts & 1 << 3 != 0 { self.pixel(x0 + y, y0 + x, color); }
                if parts & 1 << 4 != 0 { self.pixel(x0 - x, y0 - y, color); }
                if parts & 1 << 5 != 0 { self.pixel(x0 + x, y0 - y, color); }
                if parts & 1 << 6 != 0 { self.pixel(x0 - y, y0 - x, color); }
                if parts & 1 << 7 != 0 { self.pixel(x0 + y, y0 - x, color); }
            }

            y += 1;
            err += 1 + 2*y;
            if 2*(err-x) + 1 > 0 {
                x -= 1;
                err += 1 - 2*x;
            }
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
#[cold]
pub unsafe fn fast_copy(dst: *mut u8, src: *const u8, len: usize) {
    asm!("cld
        rep movsb"
        :
        : "{rdi}"(dst as usize), "{rsi}"(src as usize), "{rcx}"(len)
        : "cc", "memory", "rdi", "rsi", "rcx"
        : "intel", "volatile");
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
#[cold]
pub unsafe fn fast_set32(dst: *mut u32, src: u32, len: usize) {
    asm!("cld
        rep stosd"
        :
        : "{rdi}"(dst as usize), "{eax}"(src), "{rcx}"(len)
        : "cc", "memory", "rdi", "rcx"
        : "intel", "volatile");
}
