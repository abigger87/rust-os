// main.rs

#![no_std]
#![no_main]

extern crate vga_buffer;
use vga_buffer::println;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
/// Default entrypoint for Rust. Overwrites crt0, subverting _start_ lang item
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}

/// This function is called on panic.
/// !! Diverging Function !! - returns "never" type *!*
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}