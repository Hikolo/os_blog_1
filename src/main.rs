// main.rs

#![no_std] // don't link the Rust standard lib, it asumes many things not present
           // and must therefore be disabled.
#![no_main] // disable all rust-level entry point, start doesn't point to main
            // so there's no reason to have it.

use core::panic::PanicInfo;

#[no_mangle] // don't mangle this function name, keeps it as defined
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
