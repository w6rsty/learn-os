#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let loop_count = 20;
    let task_info = user_lib::TaskInfo::new();

    let mut i = 0;
    while i < loop_count {
        if  i < loop_count / 2 {
            user_lib::yield_();
        } else {
            println!("clock: {}", user_lib::get_time());
            
            user_lib::task_info(&task_info);
            println!("TaskInfo status:{:?}, times:{}", task_info.status, task_info.times);
        }
        i += 1;
    }
    0
}