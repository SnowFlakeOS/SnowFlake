// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

use core::fmt::{self, Write};
use arch::serial::serial;
use arch::serial::SerialPort;

pub fn _print(args: fmt::Arguments) {
    unsafe { serial().write_fmt(args).unwrap() }
}

