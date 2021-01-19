//! # global attribute
//! - `#![no_std]`
//!   Disable standatd libraries, for we are running this on bare metal.
#![no_std]
//! - `#![no_main]`
//!   only _start: as entry point
//!   build our crt ourself?
#![no_main]
//! - `#![feature(llvm_asm)]`  
//!   Inline assembly code
#![feature(llvm_asm)]
//! - `#![feature(global_asm)]`
//!   Inline assembly file
#![feature(global_asm)]
//! - `#![feature(assoc_char_funcs)]`
//!   u32 to char
#![feature(assoc_char_funcs)]
//! - `#![feature(panic_info_message)]`  
//!   get panic msg
#![feature(panic_info_message)]

global_asm!(include_str!("entry.asm"));

#[macro_use]
mod kernel;

use kernel::console::LogLevel;

/// our _start
/// no name mangling so that it keeps label rust_main
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    log!(LogLevel::Verbose, "Runtime enviroment set up finished, system starting.");
    kernel::memory::init();
    kernel::interrupt::init();
    panic!("Drop off.");
}