use crate::kernel::sbi::shutdown;
use crate::kernel::console::LogLevel;
use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    
    if let Some(location) = info.location() {
        log!(LogLevel::Fatal, "KERNEL PANIC @ {}: {}", location.file(), location.line());
    }
    else {
        log!(LogLevel::Fatal, "KERNEL PANIC");
    }
    log!(LogLevel::Fatal, "Reason: {}", info.message().unwrap());
    shutdown();
}

#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort()")
}
