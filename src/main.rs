#![no_std]
#![no_main]

mod kernel;
mod bsp;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
