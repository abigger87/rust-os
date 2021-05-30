// main.rs

#![no_std]
#![no_main]

// extern crate vga_buffer;
use vga_buffer;
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
/// Default entrypoint for Rust. Overwrites crt0, subverting _start_ lang item
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    // ** Test our vga_buffer module
    vga_buffer::print_something();

    loop {}
}

/// This function is called on panic.
/// !! Diverging Function !! - returns "never" type *!*
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}