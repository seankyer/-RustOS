#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Compute stack top based on SRAM base address and size
// These values cannot be taken from linker script because Rust compiler
// will complain that constants aren't evaluated until link time
const SRAM_BASE: u32 = 0x20000000;  // Base address of SRAM
const SRAM_SIZE: u32 = 0x00010000;  // SRAM size (64KB)
const STACK_TOP: u32 = SRAM_BASE + SRAM_SIZE;  // Stack top address

union VectorEntry {
    pointer: u32,                    // Type for stack pointer
    handler: unsafe extern "C" fn(), // Handler pointer
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".vector_table")]
static VECTOR_TABLE: [VectorEntry; 16] = [
    VectorEntry { pointer: STACK_TOP },
    VectorEntry { handler: reset_handler },
    VectorEntry { handler: nmi_handler },
    VectorEntry { handler: hardfault_handler },
    VectorEntry { pointer: 0 },
    VectorEntry { pointer: 0 },
    VectorEntry { pointer: 0 },
    VectorEntry { pointer: 0 },
    VectorEntry { handler: svc_handler },
    VectorEntry { handler: debug_monitor_handler },
    VectorEntry { pointer: 0 },
    VectorEntry { handler: pend_sv_handler },
    VectorEntry { handler: sys_tick_handler },
    VectorEntry { pointer: 0 },
    VectorEntry { pointer: 0 },
    VectorEntry { pointer: 0 },
];

#[unsafe(no_mangle)]
pub extern "C" fn reset_handler() {
    _rust_start();
}

#[unsafe(no_mangle)]
pub extern "C" fn nmi_handler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn hardfault_handler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn svc_handler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn debug_monitor_handler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn pend_sv_handler() {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn sys_tick_handler() {
    loop {}
}

// Get linker symbols
unsafe extern "C" {
    static mut _data_start: u32;
    static mut _data_end: u32;
    static mut _bss_start: u32;
    static mut _bss_end: u32;
    static _text_end: u32; // Starting LMA for .data
}

#[unsafe(no_mangle)]
pub extern "C" fn _rust_start() -> ! {
    unsafe {
        let sp: u32 = STACK_TOP;
        core::arch::asm!(
            "mov sp, {}",
            in(reg) sp
        );

        // Zero .bss section
        zero_bss();

        // Copy .data section
        copy_data();
    }

    // Transfer control to main program
    main();
}

// Copy data from source in binary to RAM
unsafe fn copy_data() {
    unsafe {
        let mut src = &_text_end as *const u32;
        let mut dest = &raw mut _data_start as *mut u32;

        while dest < &raw mut _data_end as *mut u32 {
            *dest = *src;
            dest = dest.offset(1);
            src = src.offset(1);
        }
    }
}

unsafe fn zero_bss() {
    unsafe {
        let mut dest = &raw mut _bss_start as *mut u32;
        while dest < &raw mut _bss_end as *mut u32 {
                *dest = 0;
                dest = dest.offset(1);
        }
    }
}

// Exercise .bss section
#[unsafe(no_mangle)]
pub static mut BOOT_STATUS: u32 = 0;

// Exercise .data section
#[unsafe(no_mangle)]
pub static mut BOOT_SUCCESS: u32 = 0xC0FFEE;

// Main application logic
fn main() -> !{

    let msg = b"Hello world!\0";

    for (i, &b) in msg.iter().enumerate() {
        let ptr = (0x20000100 as *mut u8).wrapping_add(i);
        unsafe {
            core::ptr::write_volatile(ptr, b);
        }
    }

    // Report a successful boot
    unsafe {
        core::ptr::write_volatile(&raw mut BOOT_STATUS, BOOT_SUCCESS);
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
