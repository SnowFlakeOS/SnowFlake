// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

use string::utf8_to_string;
use fs::load;
use core::mem;
use alloc::vec::Vec;
use alloc::string::{ToString, String};

pub struct Conf {
    pub kernel: String,
    pub kernel_option: String,
    pub boot_timeout: u64
}

pub fn load_conf() -> Conf {
    let mut conf: Conf = unsafe { mem::zeroed() };
    
    if let Ok(file) = load("\\boot2snow\\boot2snow.conf") {
        let line: Vec<String> = utf8_to_string(file).replace(" ", "").split("\n")
            .map(|s: &str| s.to_string())
            .collect();

        for data in &line {
            let s = data.split("=").nth(0).unwrap().to_string();
            if s == "kernel" {
                conf.kernel = data.split("=").nth(1).unwrap().to_string();
            } else if s == "kernel_option" {
                conf.kernel_option = data.split("=").nth(1).unwrap().to_string();
            } else if s == "boot_timeout" {
                conf.boot_timeout = data.split("=").nth(1).unwrap().to_string().parse::<u64>().unwrap();
            }
        }
    }

    conf
} 