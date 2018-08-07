use color::Color;
use arch::keyboard;
use display::Display;

pub fn uidraw(display: &mut Display) {
    let (width, height) = { (display.width(), display.height()) };
    let (width2, height2) = { (20, 20) };

    println! ("SnowFlake UI Test.");
    display.rect(0, 0, width, height, Color::rgb(50, 45, 55));
    display.rect(0, 0, width2, height2, Color::rgb(255, 255, 255));
}