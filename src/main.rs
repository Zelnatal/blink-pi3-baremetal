#![no_std]
#![no_main]

use core::{
    panic::PanicInfo,
    ptr::{read_volatile, write_volatile},
};

const GPFSEL2: *mut u32 = 0x3F20_0008 as *mut u32;
const GPSET0: *mut u32 = 0x3F20_001C as *mut u32;
const GPCLR0: *mut u32 = 0x3F20_0028 as *mut u32;

const CLO: *mut u32 = 0x3F00_3004 as *mut u32;
const CHI: *mut u32 = 0x3F00_3008 as *mut u32;

#[no_mangle]
fn _start() -> ! {
    unsafe {
        write_volatile(GPFSEL2, 1 << 18);
    }
    loop {
        unsafe {
            write_volatile(GPSET0, 1 << 26);
        }
        delay(1000);
        unsafe {
            write_volatile(GPCLR0, 1 << 26);
        }
        delay(1000);
    }
}

fn delay(t: u32) {
    let target = time() + (t * 1000) as u64;
    while time() < target {}
}

fn time() -> u64 {
    unsafe { (read_volatile(CHI) as u64) << 32 | (read_volatile(CLO) as u64) }
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}
