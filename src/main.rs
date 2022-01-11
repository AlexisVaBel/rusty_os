#![no_std] // don`t link std
#![no_main] // disable rust-level entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello world";

#[no_mangle]// don't mangle the name of this function, as it`s our entry point
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
