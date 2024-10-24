use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // 检查是否存在位置信息
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} - {}",
            location.file(),
            location.line(),
            info.message() // 确保有信息输出
        );
    } else {
        // 没有位置信息时的处理
        println!(
            "Panicked: {}",
            info.message()
        );
    }
    
    shutdown(true) // 关闭系统
}
