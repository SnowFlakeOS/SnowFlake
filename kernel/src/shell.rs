// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

use arch::keyboard;
use display::Display;
use testui;

pub fn execute(display: &mut Display) {
    println!("Starting Snowflake Minimal Shell... (Debug)");
    print!("///// Welcome to Snowflake Minimal Shell /////");

    loop {
        print!("\n> ");
        let gets = unsafe { keyboard::gets() };
        let mut command = gets.split_whitespace();

        print!("\n");

        match command.nth(0).unwrap() {
            "help" => {
                print!("Snowflake OS Kernel {}", env!("CARGO_PKG_VERSION"));
            },
            "exit" => {
                println!("Exiting Shell... (Debug)");
                break
            },
            "start" => {
                testui::uidraw(display);
            }
            _ => {
                print!("Unknown command :(");
            }
        }
    }
}