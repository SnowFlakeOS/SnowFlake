// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//! Some code was borrowed from [System76 Firmware Update](https://github.com/system76/firmware-update)

use alloc::{String, Vec};
use core::char;

pub fn wstr(string: &str) -> Vec<u16> {
    let mut wstring = vec![];

    for c in string.chars() {
        wstring.push(c as u16);
    }
    wstring.push(0);

    wstring
}

pub fn nstr(wstring: *const u16) -> String {
    let mut string = String::new();

    let mut i = 0;
    loop {
        let w = unsafe { *wstring.offset(i) };
        i += 1;
        if w == 0 {
            break;
        }
        let c = unsafe { char::from_u32_unchecked(w as u32) };
        string.push(c);
    }

    string
}

pub fn utf8_to_string(vector: Vec<u8>) -> String {
    String::from_utf8(vector).unwrap()
}