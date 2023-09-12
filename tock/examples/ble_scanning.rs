//! An extremely simple libtock-rs example. Just prints out a message
//! using the Console capsule, then terminates.

#![no_main]
#![no_std]

use core::fmt::Write;
use libtock::console::Console;
use libtock::runtime::{set_main, stack_size};

mod ble;
use ble::{BLE};

set_main! {main}
stack_size! {0x1000}


// example file


fn callback(buf: &[u8], res: u32, len: u32) {
    writeln!(Console::writer(),"Callback {:?} {} {}", buf, res, len).unwrap();
}

fn main() {
    
    writeln!(Console::writer(), " - Start Scanning BLE...").unwrap();
    
    let err = BLE::passive_scan(&callback);
    writeln!(Console::writer(), "=> {:?}", err).unwrap();
    
}