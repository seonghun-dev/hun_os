#![no_std]
#![no_main]

use core::panic::PanicInfo;
static HELLO: &[u8] = b"Hello World!";
mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    loop {}
}
