#![no_main]
#![no_std]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod config;
mod lang_item;
mod sbi;
mod sync;
pub mod syscall;
pub mod trap;
mod loader;
mod task;
mod timer;
mod stack_trace;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Init trap");
    trap::init();
    println!("[kernel] Loading applications");
    loader::load_apps();
    println!("[kernel] Enable timer interrupt");
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    println!("[kernel] Start executing applications");
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}