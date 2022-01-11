#![no_std] // don`t link std
#![no_main] // disable rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

use crate::vga_buffer::print_something;


static HELLO: &[u8] = b"Hello world";
static PRINTING_STATIC: &[u8] = b"This is static text and no other functions";

#[no_mangle]// don't mangle the name of this function, as it`s our entry point
pub extern "C" fn _start() -> ! {
    print_something();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

