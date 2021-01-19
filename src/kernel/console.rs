//! Use to format string & console r/w
//! Note that we have no access to std, so bare with str.
#![allow(unused)]

use crate::kernel::sbi::*;
use core::fmt::{self, Write};

pub const FG_BLACK      :u8 = 30;
pub const FG_RED        :u8 = 31;
pub const FG_GREEN      :u8 = 32;
pub const FG_YELLOW     :u8 = 33;
pub const FG_BLUE       :u8 = 34;
pub const FG_MAGENTA    :u8 = 35;
pub const FG_CYAN       :u8 = 36;
pub const FG_WHITE      :u8 = 37;

pub const FG_B_BLACK    :u8 = 90;
pub const FG_B_RED      :u8 = 91;
pub const FG_B_GREEN    :u8 = 92;
pub const FG_B_YELLOW   :u8 = 93;
pub const FG_B_BLUE     :u8 = 94;
pub const FG_B_MAGENTA  :u8 = 95;
pub const FG_B_CYAN     :u8 = 96;
pub const FG_B_WHITE    :u8 = 97;

pub const FG_DEFAULT    :u8 = 39;

pub const BG_BLACK      :u8 = 40;
pub const BG_RED        :u8 = 41;
pub const BG_GREEN      :u8 = 42;
pub const BG_YELLOW     :u8 = 43;
pub const BG_BLUE       :u8 = 44;
pub const BG_MAGENTA    :u8 = 45;
pub const BG_CYAN       :u8 = 46;
pub const BG_WHITE      :u8 = 47;

pub const BG_B_BLACK    :u8 = 100;
pub const BG_B_RED      :u8 = 101;
pub const BG_B_GREEN    :u8 = 102;
pub const BG_B_YELLOW   :u8 = 103;
pub const BG_B_BLUE     :u8 = 104;
pub const BG_B_MAGENTA  :u8 = 105;
pub const BG_B_CYAN     :u8 = 106;
pub const BG_B_WHITE    :u8 = 107;

pub const BG_DEFAULT    :u8 = 49;

#[derive(PartialOrd)]
#[derive(PartialEq)]
pub enum LogLevel {
    Verbose = 0,
    Info    = 1,
    Good    = 2,
    Warning = 3,
    Error   = 4,
    Fatal   = 5,
}
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> { 
        let s = match *self {
            LogLevel::Verbose   => "[ VERBOSE ]",
            LogLevel::Info      => "[   INFO  ]",
            LogLevel::Good      => "[   Good  ]",
            LogLevel::Warning   => "[ WARNING ]",
            LogLevel::Error     => "[  ERROR  ]",
            LogLevel::Fatal     => "[  FATAL  ]"
        };
        f.write_str(s)
    }
}

pub const MIN_LOG_LEVEL : LogLevel = LogLevel::Verbose;

/// For core::fmt::Write, if you implement write_str, you get write_fmt for free!
struct Stdout;
impl Write for Stdout {
    fn write_str(&mut self, to_write: &str) -> core::result::Result<(), core::fmt::Error> { 
        let mut buffer = [0u8; 4];
        for c in to_write.chars() {
            for code_point in c.encode_utf8(&mut buffer).as_bytes().iter() {
                console_putchar(*code_point as char);
            }
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::kernel::console::print(format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}

pub fn set_log_color(ll: LogLevel) {
    match ll {
        LogLevel::Verbose   => set_color(FG_B_BLACK,  BG_DEFAULT),
        LogLevel::Info      => set_color(FG_DEFAULT,  BG_DEFAULT),
        LogLevel::Good      => set_color(FG_B_GREEN,  BG_DEFAULT),
        LogLevel::Warning   => set_color(FG_B_YELLOW, BG_DEFAULT),
        LogLevel::Error     => set_color(FG_B_RED,    BG_DEFAULT),
        LogLevel::Fatal     => set_color(FG_B_RED,    BG_DEFAULT)
    }
}

pub fn set_color(fg: u8, bg: u8) {
    print!("\x1b[{};{}m", fg, bg);
}

pub fn reset_color() {
    set_color(FG_DEFAULT, BG_DEFAULT);
}

#[macro_export]
macro_rules! log {
    ($log_level: path, $fmt: literal $(, $($arg: tt)+)?) => {
        if $log_level >= $crate::kernel::console::MIN_LOG_LEVEL {
            $crate::kernel::console::set_log_color($log_level);
            print!("{} @ {}: {} \t", $log_level, file!(), line!());
            $crate::kernel::console::print(format_args!($fmt $(, $($arg)+)?));
            $crate::kernel::console::reset_color();
            println!();
        }
    };
}