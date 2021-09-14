#![no_std]
#![no_main]
#![allow(non_snake_case)]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("MelbOs").unwrap();
    write!(vga_buffer::WRITER.lock(), "ver-{}year-{}", 010, 2021);

    loop{}
}
