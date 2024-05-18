#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let run_times = 1000_00;
    let task_info = user_lib::TaskInfo::new();
    let mut i = 0;
    while i < run_times {
        if (i % 1000) == 0 {
            println!("Hello, world! from user mode");
            user_lib::task_info(&task_info);
            println!("TaskInfo status:{:?}, times:{}", task_info.status, task_info.times);
        }
        user_lib::yield_();
        i += 1;
    }

    0
}