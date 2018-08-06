use color::Color;
use arch::keyboard;
use display::Display;

pub fn uidraw(display: &mut Display) {
    let (width, height) = { (display.width(), display.height()) };
println! ("SnowFlake boot ok. ");
    loop {
        display.rect(0, 0, width, height, Color::rgb(50, 45, 55));
        
    }
}