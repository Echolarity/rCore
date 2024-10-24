
#![no_main]
#![no_std]

#[macro_use]
mod console;
mod lang_items;
pub mod sbi;
use crate::console::LogLevel;
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));


macro_rules! log_level {
    ("ERROR") => { "ERROR" };
    ("WARN")  => { "WARN" };
    ("INFO")  => { "INFO" };
    ("DEBUG") => { "DEBUG" };
    ("TRACE") => { "TRACE" };
}

// 条件编译根据特性启用对应的日志级别
#[cfg(feature = "log_error")]
pub const LOG_LEVEL: &str = log_level!("ERROR");

#[cfg(feature = "log_warn")]
pub const LOG_LEVEL: &str = log_level!("WARN");

#[cfg(feature = "log_info")]
pub const LOG_LEVEL: &str = log_level!("INFO");

#[cfg(feature = "log_debug")]
pub const LOG_LEVEL: &str = log_level!("DEBUG");

#[cfg(feature = "log_trace")]
pub const LOG_LEVEL: &str = log_level!("TRACE");

// 如果没有任何特性启用，则默认使用 INFO
#[cfg(not(any(
    feature = "log_error",
    feature = "log_warn",
    feature = "log_info",
    feature = "log_debug",
    feature = "log_trace"
)))]
pub const LOG_LEVEL: &str = log_level!("INFO");

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    match LOG_LEVEL {
        "DEBUG" => {
            debug!("Hello from DEBUG mode!");
        },
        "INFO" => {
            info!("Hello from INFO mode!");
        },
        "WARN" => {
            warn!("Hello from WARN mode!");
        },
        "ERROR" => {
            error!("Hello from ERROR mode!");
        },
        "TRACE" => {
            trace!("Hello from TRACE mode!");
        },
        _ => {
            panic!("Invalid LOG_LEVEL");
        }
    }
    info!("This is an info message.");
    warn!("This is a warning.");
    error!("This is an error message.");
    debug!("This is a debug message.");
    trace!("This is a trace message.");
    print_memory_layout();
    sbi::shutdown(true);
    //panic!("Shutdown machine!");

}
extern "C" {
    // 定义内存段的起始和结束地址
    static stext: usize;    // .text 段起始地址
    static etext: usize;    // .text 段结束地址
    static srodata: usize;  // .rodata 段起始地址
    static erodata: usize;  // .rodata 段结束地址
    static sdata: usize;    // .data 段起始地址
    static edata: usize;    // .data 段结束地址
    static sbss: usize;     // .bss 段起始地址
    static ebss: usize;     // .bss 段结束地址
}

// 示例函数，打印内存布局信息
fn print_memory_layout() {
    unsafe {
        // 使用 info! 打印内存布局信息
        info!(".text [{:#x}, {:#x})", stext, etext);
        info!(".rodata [{:#x}, {:#x})", srodata, erodata);
        info!(".data [{:#x}, {:#x})", sdata, edata);
        info!(".bss [{:#x}, {:#x})", sbss, ebss);
    }
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}