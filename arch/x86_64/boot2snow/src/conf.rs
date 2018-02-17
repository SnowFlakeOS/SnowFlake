use string::{wstr, utf8_to_string};
use fs::{File, Dir, find, load};

pub struct Conf {
    kernel: &'static str,
    kernel_option: &'static str,
    boot_timer: u16
}

pub fn load_conf() -> Conf {
    let mut conf = Conf { kernel: "", kernel_option: "", boot_timer: 0 };
    
    if let Ok(file) = load("\\boot2snow\\boot2snow.conf") {
        for data in utf8_to_string(file).replace(" ", "").split("\n") {
            println!("{}", data);
        }
    }

    conf
} 