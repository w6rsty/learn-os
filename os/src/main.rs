#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod sbi;
mod lang_item;
mod batch;

use core::arch::global_asm;
use crate::sbi::*;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world");
    shutdown(false);
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}