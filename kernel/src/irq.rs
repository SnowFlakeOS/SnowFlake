#[no_mangle]
pub extern fn irq_trigger(irq: u8) {
    match irq {
        1 => { println!("test") },
        _ => { println!("What The") }
    }
}