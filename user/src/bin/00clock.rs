#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let loop_count = 1000_000;
    let task_info = user_lib::TaskInfo::new();

    for i in 0..loop_count {
        if  i % 100_000 == 0 {
            println!("clock: {}", user_lib::get_time());

            user_lib::task_info(&task_info);
            println!("task info: {:?}", task_info);
        } 
    }
    0
}