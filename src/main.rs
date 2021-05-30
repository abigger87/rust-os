// main.rs

#![no_std]
#![no_main]

extern crate vga_buffer;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
/// Default entrypoint for Rust. Overwrites crt0, subverting _start_ lang item
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}

/// This function is called on panic.
/// !! Diverging Function !! - returns "never" type *!*
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}