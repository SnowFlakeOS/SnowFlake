//! Some code was borrowed from [Redox OS Kernel](https://gitlab.redox-os.org/redox-os/kernel)

use alloc::string::String;

use x86_64::instructions::port::Port;

pub static mut KEYBOARD: Keyboard = Keyboard::new();

pub mod us {
    static US: [[char; 2]; 58] = [
        ['\0', '\0'],
        ['\x1B', '\x1B'],
        ['1', '!'],
        ['2', '@'],
        ['3', '#'],
        ['4', '$'],
        ['5', '%'],
        ['6', '^'],
        ['7', '&'],
        ['8', '*'],
        ['9', '('],
        ['0', ')'],
        ['-', '_'],
        ['=', '+'],
        ['\x7F', '\x7F'],
        ['\t', '\t'],
        ['q', 'Q'],
        ['w', 'W'],
        ['e', 'E'],
        ['r', 'R'],
        ['t', 'T'],
        ['y', 'Y'],
        ['u', 'U'],
        ['i', 'I'],
        ['o', 'O'],
        ['p', 'P'],
        ['[', '{'],
        [']', '}'],
        ['\n', '\n'],
        ['\0', '\0'],
        ['a', 'A'],
        ['s', 'S'],
        ['d', 'D'],
        ['f', 'F'],
        ['g', 'G'],
        ['h', 'H'],
        ['j', 'J'],
        ['k', 'K'],
        ['l', 'L'],
        [';', ':'],
        ['\'', '"'],
        ['`', '~'],
        ['\0', '\0'],
        ['\\', '|'],
        ['z', 'Z'],
        ['x', 'X'],
        ['c', 'C'],
        ['v', 'V'],
        ['b', 'B'],
        ['n', 'N'],
        ['m', 'M'],
        [',', '<'],
        ['.', '>'],
        ['/', '?'],
        ['\0', '\0'],
        ['\0', '\0'],
        ['\0', '\0'],
        [' ', ' ']
    ];

    pub fn get_char(scan_code: u8, shift: bool) -> char {
        if let Some(c) = US.get(scan_code as usize) {
            if shift {
                c[1]
            } else {
                c[0]
            }
        } else {
            '\0'
        }
    }
}

pub struct Keyboard {
    control: Port<u8>,
    input: Port<u8>,
    shift_down: bool,
    caps_lock: bool,
    num_lock: bool,
    scroll_lock: bool,
}

pub unsafe fn init() -> Result<(), ()> {
    let _ = KEYBOARD.control.write(0xAE);

    KEYBOARD.input_ack();

    let _ = KEYBOARD.input.write(0xF4);

    KEYBOARD.ack();
    KEYBOARD.change_led(false, false, false);

    Ok(())
}

pub unsafe fn enable_a20() {
    KEYBOARD.control.write(0xD0);

    KEYBOARD.output_ack();

    let out_data = KEYBOARD.input.read() | 0x01;

    KEYBOARD.output_ack();

    KEYBOARD.control.write(0xD1);
    KEYBOARD.input.write(out_data);
}

pub unsafe fn gets() -> String {
    let mut input: String = String::new();

    loop {
        if KEYBOARD.is_output() {
            let c = KEYBOARD.scan_code();
            let mut f = if c == 0xE1 { 1 } else { 0 };

            if (c & 0x80) != 0 {
                f |= 1;
            }

            if !KEYBOARD.keyboard_handler(c) {
                if (f & 1) == 0 {
                    if c != 207 {
                        let ascii = KEYBOARD.convert_ascii(c);
                        let backspace: char = 127 as char;
                        let len = input.len();

                        if ascii == '\n' { 
                            break;
                        } else if ascii == backspace {
                            if len != 0 {
                                input.remove(len - 1);
                            }
                        } else {
                            input.push(ascii);
                        }
                        print!("{}", ascii);
                    }
                }
            }
        }
    }

    input
}

impl Keyboard {
    pub const fn new() -> Self {
        Self {
            control: Port::new(0x64),
            input: Port::new(0x60),
            shift_down: false,
            caps_lock: false,
            num_lock: false,
            scroll_lock: false
        }
    }

    pub unsafe fn is_output(&self) -> bool {
        if (self.control.read() & 0x01) != 0 { return true } else { return false }
    }

    pub unsafe fn is_input(&self) -> bool {
        if (self.control.read() & 0x02) != 0 { return true } else { return false }
    }

    pub unsafe fn scan_code(&self) -> u8 {
        while !self.is_output() { }
        self.input.read()
    }

    pub unsafe fn ack(&self) {
        for i in 0..100 {
            self.output_ack();

            if self.input.read() == 0xFA { break }
        }
    }

    unsafe fn output_ack(&self) {
        for i in 0..0xFFFF {  if self.is_output() { break } }
    }

    unsafe fn input_ack(&self) {
        for i in 0..0xFFFF {  if !self.is_input() { break } }
    }

    pub unsafe fn change_led(&mut self, caps_lock: bool, num_lock: bool, scroll_lock: bool) -> Result<(), ()>{
        self.input_ack();

        let _ = self.input.write(0xED);

        self.input_ack();
        self.ack();

        let _ = self.input.write(((caps_lock as u8) << 2) | ((num_lock as u8) << 1) | (scroll_lock as u8));

        self.input_ack();
        self.ack();

        Ok(())
    }

    pub unsafe fn keyboard_handler(&mut self, scan_code: u8) -> bool {
        let mut is_changed = true;
        let (down, c) = if (scan_code & 0x80) != 0 { (false, scan_code & 0x7F) } else { (true, scan_code) };

        match c {
            42 | 54  => { self.shift_down = down },
            58 => { if down { self.caps_lock ^= true } },
            69 => { if down { self.num_lock ^= true } },
            70 => { if down { self.scroll_lock ^= true } },
            _ => { is_changed = false }
        }

        let (caps_lock, num_lock, scroll_lock) = { (self.caps_lock, self.num_lock, self.scroll_lock) };

        let _ = self.change_led(caps_lock, num_lock, scroll_lock);

        is_changed
    }

    pub unsafe fn convert_ascii(&self, scan_code: u8) -> char {
        let shift = if self.shift_down || self.caps_lock { true } else { false };
        us::get_char(scan_code, shift)
    }
}