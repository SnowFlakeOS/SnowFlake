use core::fmt::{self, Write};
use color::*;
use display::Display;

static mut DISPLAY: *mut Display = 0 as *mut _;
static mut CONSOLE: *mut Console = 0 as *mut _;

pub struct Console {
    w: u32,
    h: u32,
    x: i32,
    y: i32
}

pub fn get_console() -> *mut Console {
	unsafe { CONSOLE }
}

pub fn set_console(console: &mut Console) {
    unsafe { CONSOLE = console }
}

pub struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, string: &str) -> Result<(), fmt::Error> {
        let console = get_console();
        unsafe { (*console).write(string, Color::rgb(255, 255, 255)) };
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

impl Console {
    pub fn new(display: &mut Display) -> Self {
        let w = display.width().clone();
        let h = display.height().clone();

        unsafe { DISPLAY = display };

        Self {
            w,
            h,
            x: 0,
            y: 0
        }
    }

    pub fn write(&mut self, s: &str, color: Color) {
        let display = unsafe{ DISPLAY };
        let prompt = s.clone();

        for c in prompt.chars() {
            if self.x == self.w as i32 || c == '\n' { self.newline(); } else {
                unsafe { (*display).char(self.x, self.y, c, color) };
                self.x += 8;
            }
        }
    }

    pub fn newline(&mut self) {
        self.x = 0;
        self.y += 14;
    }
}
