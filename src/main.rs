#![no_std]      // disable rust standard library
#![no_main]     // disable all rust-level entry points
                //
use core::panic::PanicInfo;

/// Panic function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]    // don't mangle the name of this fuction
pub extern "C" fn _start() -> ! {
    // This function is the entrypoint
    // named `_start` by default
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
            unsafe {
                    *vga_buffer.offset(i as isize * 2) = byte;
                    *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
            }
    }

    loop {}
}

