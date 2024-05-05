#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let loop_count = 1000_000;
    for i in 0..loop_count {
        if  i % 100_000 == 0 {
            println!("clock: {}", user_lib::get_time());
        } 
    }
    0
}