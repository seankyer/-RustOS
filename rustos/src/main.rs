#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)] // don't mangle the name of this function since linker looks for `_start`
pub extern "C" fn _start() -> ! {
    loop {
        // Dummy write for debugging
        unsafe {
            core::ptr::write_volatile(0x20000000 as *mut u32, 0x75);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
