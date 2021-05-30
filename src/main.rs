// main.rs

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
/// Default entrypoint for Rust. Overwrites crt0, subverting _start_ lang item
pub extern "C" fn _start() -> ! {
    loop {}
}

/// This function is called on panic.
/// !! Diverging Function !! - returns "never" type *!*
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}