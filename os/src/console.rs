use crate::sbi::console_putchar;
use core::fmt::{self, Write};
use crate::LOG_LEVEL;
// ANSI 颜色代码
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[93m";
const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const GRAY: &str = "\x1b[90m";
// 日志级别


#[derive(Clone)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

fn get_color(level: LogLevel) -> &'static str {
    match level {
        LogLevel::Error => RED,
        LogLevel::Warn => YELLOW,
        LogLevel::Info => BLUE,
        LogLevel::Debug => GREEN,
        LogLevel::Trace => GRAY,
    }
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print_with_color(args: fmt::Arguments, level: LogLevel) {
    let l=level.clone();
    if should_log(l) {
        let color = get_color(level);
        Stdout.write_str(color).unwrap();
        Stdout.write_fmt(args).unwrap();
        Stdout.write_str(RESET).unwrap();
    }
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print_with_color(format_args!($fmt $(, $($arg)+)?), $crate::console::LogLevel::Info);
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print_with_color(format_args!(concat!($fmt, "\n") $(, $($arg)+)?), $crate::console::LogLevel::Info);
    }
}
// 添加一个辅助函数，用于将 LogLevel 转换为字符串
#[macro_export]
macro_rules! log {
    ($level:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        $crate::console::print_with_color(
            format_args!("[{}] {}\n", $crate::console::log_level_str($level), format_args!($fmt $(, $($arg)+)?)), 
            $level
        );
    }
}

// 添加一个辅助函数，用于将 LogLevel 转换为字符串
pub fn log_level_str(level: LogLevel) -> &'static str {
    match level {
        LogLevel::Error => "Error",
        LogLevel::Warn => " Warn",
        LogLevel::Info => " Info",
        LogLevel::Debug => "Debug",
        LogLevel::Trace => "Trace",
    }
}
fn log_level_priority(level: LogLevel) -> usize {
    match level {
        LogLevel::Error => 1,
        LogLevel::Warn => 2,
        LogLevel::Info => 3,
        LogLevel::Debug => 4,
        LogLevel::Trace => 5,
    }
}

fn should_log(level: LogLevel) -> bool {
    log_level_priority(level) <= log_level_priority(get_current_log_level())
}
fn get_current_log_level() -> LogLevel {
    match LOG_LEVEL {
        "ERROR" => LogLevel::Error,
        "WARN" => LogLevel::Warn,
        "INFO" => LogLevel::Info,
        "DEBUG" => LogLevel::Debug,
        "TRACE" => LogLevel::Trace,
        _ => LogLevel::Info,  // 默认是 INFO 级别
    }
}
// 封装具体的日志级别
#[macro_export]
macro_rules! info {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::log!(LogLevel::Info, $fmt $(, $($arg)+)?);
    }
}

#[macro_export]
macro_rules! warn {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::log!(LogLevel::Warn, $fmt $(, $($arg)+)?);
    }
}

#[macro_export]
macro_rules! error {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::log!(LogLevel::Error, $fmt $(, $($arg)+)?);
    }
}

#[macro_export]
macro_rules! debug {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::log!(LogLevel::Debug, $fmt $(, $($arg)+)?);
    }
}

#[macro_export]
macro_rules! trace {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::log!(LogLevel::Trace, $fmt $(, $($arg)+)?);
    }
}