#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(custom_test_frameworks)]
#![test_runner(melb_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use melb_os::{println, print, task::{Task, executor::Executor, simple_executor::SimpleExecutor, keyboard}};

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use melb_os::allocator;
    use melb_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("MelbOS | ver:{:?}", VERSION);
    melb_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {memory::init(phys_mem_offset)};
    let mut frame_allocator = unsafe {BootInfoFrameAllocator::init(&boot_info.memory_map)};

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    
    #[cfg(test)]
    test_main();
    
    println!("It did not crash!");
    
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

async fn async_num() -> u32 {
    42
}

async fn example_task() {
    let number = async_num().await;
    println!("Async Number: {}", number);
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    melb_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    melb_os::test_panic_handler(info)
}