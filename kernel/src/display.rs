//! Some code was borrowed from [System76 Firmware Update](https://github.com/system76/firmware-update) and [Redox OS Orbital Client Library](https://github.com/redox-os/orbclient)

use core::cmp;
use color::*;

pub struct Display {
    output: *mut Color,
    scale: u32,
    w: u32,
    h: u32,
    font: &'static [u8]
}

impl Display {
    pub fn new(output: *mut Color, w: u32, h: u32) -> Self {
        let scale = if h > 1440 { 2 } else { 1 };
        Self {
            output,
            scale,
            w: w * scale,
            h: h * scale,
            font: include_bytes!("../../res/unifont.font")
        }
    }

    pub fn scale(&self) -> u32 {
        self.scale
    }

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }

    pub fn scroll(&mut self, rows: usize, color: Color) {
        let scale = self.scale as usize;
        self.inner_scroll(rows * scale, color);
    }
    
    pub fn pixel(&mut self, x: i32, y: i32, color: Color) {
        self.inner_pixel(x, y, color);
    }

    pub fn rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
        self.inner_rect(x, y, w, h, color);
    }

    pub fn line(&mut self, argx1: i32, argy1: i32, argx2: i32, argy2: i32, color: Color) {
        self.line(argx1, argy1, argx2, argy2, color);
    }

    pub fn circle(&mut self, x0: i32, y0: i32, radius: i32, filled: bool, color: Color) {
        self.inner_circle(x0, y0, radius, filled, color);
    }

    pub fn string(&mut self, x: i32, y: i32, s: &str, color: Color) {
        let prompt = s.clone();
        let mut x = x.clone();

        for c in prompt.chars() {
            self.char(x, y, c, color);
            x += 8;
        }
    }

    /// Draw a character, using the loaded font
    pub fn char(&mut self, x: i32, y: i32, c: char, color: Color) {
        let mut offset = (c as usize) * 16;
        for row in 0..16 {
            let row_data = if offset < self.font.len() {
                self.font[offset]
            } else {
                0
            };

            for col in 0..8 {
                let pixel = (row_data >> (7 - col)) & 1;
                if pixel > 0 {
                    self.inner_pixel(x + col as i32, y + row as i32, color);
                }
            }
            offset += 1;
        }
    }

    /// Draw a piece of an arc. Negative radius will fill in the inside
    pub fn arc(&mut self, x0: i32, y0: i32, radius: i32, parts: u8, color: Color) {
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

    pub fn rounded_rect(&mut self, x: i32, y: i32, w: u32, h: u32, radius: u32, filled: bool, color: Color) {
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


    fn inner_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
        let self_w = self.w;
        let self_h = self.h;

        let start_y = cmp::max(0, cmp::min(self_h as i32 - 1, y));
        let end_y = cmp::max(start_y, cmp::min(self_h as i32, y + h as i32));

        let start_x = cmp::max(0, cmp::min(self_w as i32 - 1, x));
        let len = cmp::max(start_x, cmp::min(self_w as i32, x + w as i32)) - start_x;

        let alpha = (color.0 >> 24) & 0xFF;
        if alpha <= 0 { return };

        if alpha >= 255 {
            for y in start_y..end_y {
                unsafe {
                    fast_set32(self.output.offset((y * self_w as i32 + start_x) as isize) as *mut u32, color.0, len as usize);
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

    fn inner_scroll(&mut self, rows: usize, color: Color) {
        let width = self.w as usize;
        let height = self.h as usize;
        if rows > 0 && rows < height {
            let off1 = rows * width;
            let off2 = height * width - off1;
            unsafe {
                let output_ptr = self.output as *mut u32;
                fast_copy(output_ptr as *mut u8, output_ptr.offset(off1 as isize) as *const u8, off2 as usize * 4);
                fast_set32(output_ptr.offset(off2 as isize), color.0, off1 as usize);
            }
        }
    }

    fn inner_pixel(&mut self, x: i32, y: i32, color: Color) {
        let w = self.w;
        unsafe { *self.output.offset((w as isize * y as isize) + x as isize) = color };
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

        loop {
            self.inner_pixel(x, y, color);

            if x == argx2 && y == argy2 { break };

            let err_tolerance = 2 * err;
            if err_tolerance > -dx { err -= dy; x += sx; }
            if err_tolerance < dy { err += dx; y += sy; }
        }
    }

    /// Draw a circle. Negative radius will fill in the inside
    fn inner_circle(&mut self, x0: i32, y0: i32, radius: i32, filled: bool, color: Color) {
        let mut x = 1;
        let mut y = radius.abs();

        if filled {
            self.inner_line(x0, radius + y0, x0, -radius + y0, color);
            self.inner_line(radius + x0, y0, -radius + x0, y0, color);
        } else {
            self.inner_pixel(x0, radius + y0, color);
            self.inner_pixel(x0, -radius + y0, color);
            self.inner_pixel(radius + x0, y0, color);
            self.inner_pixel(-radius + x0, y0, color);
        }

        let mut distance = -radius;

        while x <= y {
            distance += (x << 1) - 1;

            if distance >= 0 {
                y -= 1;

                distance += (-y << 1) + 2;
            }

            if filled {
                self.inner_line(x0 - x, y0 + y, x0 + x, y0 + y, color);
                self.inner_line(x0 - x, y0 - y, x0 + x, y0 - y, color);
                self.inner_line(x0 - y, y0 + x, x0 + y, y0 + x, color);
                self.inner_line(x0 - y, y0 - x, x0 + y, y0 - x, color);
            } else {
                self.inner_pixel(x0 + x, y0 + y, color);
                self.inner_pixel(x0 + x, y0 - y, color);
                self.inner_pixel(x0 - x, y0 + y, color);
                self.inner_pixel(x0 - x, y0 - y, color);
                self.inner_pixel(x0 + y, y0 + x, color);
                self.inner_pixel(x0 + y, y0 - x, color);
                self.inner_pixel(x0 - y, y0 + x, color);
                self.inner_pixel(x0 - y, y0 - x, color);
            }

            x += 1;
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