mod fs;
use fs::sys_write;

mod process;
use process::sys_exit;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_GET_TASKINFO: usize = 94;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_GET_TASKINFO => sys_get_taskinfo(args[0], args[1]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}

pub fn sys_get_taskinfo(buf: usize, len: usize) -> isize {
    let slice = unsafe {
        core::slice::from_raw_parts(buf as *const u8, len)
    };
    let str = core::str::from_utf8(slice).unwrap();
    println!("Task info: {}", str);
    len as isize
}