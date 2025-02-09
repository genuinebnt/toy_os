#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(toy_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::{arch::asm, panic::PanicInfo};

use toy_os::interrupts;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    interrupts::init();

    unsafe { *(0x80 as *mut u64) = 42 };
    //divide_by_zero();
    //unsafe { asm!("ud2") };
    println!("Hello, World!");

    #[cfg(test)]
    test_main();

    loop {}
}

fn divide_by_zero() {
    unsafe { asm!("mov dx, 0; div dx") }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    toy_os::test_panic_handler(info)
}
