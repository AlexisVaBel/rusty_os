#![no_std] // don`t link std
#![no_main] // disable rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;


#[no_mangle]// don't mangle the name of this function, as it`s our entry point
pub extern "C" fn _start() -> ! {
    println!("R_OS {}", "Small osya");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

