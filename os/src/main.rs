#![no_main]
#![no_std]
#![feature(panic_info_message)]

use log::*;
#[macro_use]
mod console;
pub mod batch;
mod lang_item;
mod logging;
mod sbi;
mod sync;
pub mod syscall;
pub mod trap;

mod stack_trace;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack_lower_bound();
        fn boot_stack_top();
    }
    clear_bss();
    logging::init();
    println!("[kernel] Entering kernel");
    trace!(
        "[kernel] .text [{:#x}, {:#x}",
        stext as usize, 
        etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x}",
        srodata as usize,
        erodata as usize, 
    );
    info!(
        "[kernel] .data [{:#x}, {:#x}",
        sdata as usize,
        edata as usize,
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize,
        boot_stack_lower_bound as usize
    );
    error!(
        "[kernel] .bss [{:#x}, {:#x}",
        sbss as usize,
        ebss as usize,
    );
    trap::init();
    batch::init();
    batch::run_next_app();
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