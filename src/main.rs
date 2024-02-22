#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo, ptr::write_volatile};

mod boot {
    use core::arch::global_asm;

    global_asm!(".section .start._start");
}

const GPFSEL2: *mut u32 = 0x3F20_0008 as *mut u32;
const GPSET0: *mut u32 = 0x3F20_001C as *mut u32;
const GPCLR0: *mut u32 = 0x3F20_0028 as *mut u32;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        write_volatile(GPFSEL2, 1 << 18);
    }
    loop {
        unsafe {
            write_volatile(GPSET0, 1 << 26);
        }
        for _ in 0..5_000_000 {
            unsafe {
                asm!("nop");
            }
        }
        unsafe {
            write_volatile(GPCLR0, 1 << 26);
        }
        for _ in 0..5_000_000 {
            unsafe {
                asm!("nop");
            }
        }
    }   
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}
