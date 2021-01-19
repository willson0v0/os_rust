#![allow(unused)]
/// memory managment, allocator etc.

use crate::kernel::console::*;
const KERNBASE : usize = 0x80200000;    // opensbi took about 116KB
const PHYSTOP  : usize = 0x80800000;    // based on k210

pub fn init() {
    log!(LogLevel::Info, "Initializing Memory module.");
}