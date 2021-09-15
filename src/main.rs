#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;
use crate::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("MelbOs");
    
    #[cfg(test)]
    test_main();
    
    loop {}
}

//Outside test mode
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

//Panic handler for test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[test_case]
fn trivial_assertion_ineq() {
    assert_ne!(1 ,2);
}
