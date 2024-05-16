use crate::{
    config::MAX_SYSCALL_NUM,
    task::{task::TaskStatus, exit_current_and_run_next, suspend_current_and_run_next},
    timer::get_time_ms,
};

#[allow(dead_code)]
pub struct TaskInfo {
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    times: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application extied with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

pub fn sys_get_task_info(ti: *mut TaskInfo) -> isize {
    let tcb = crate::task::get_current_task_tcb();
    unsafe {
        (*ti) = TaskInfo {
            status: tcb.task_status,
            syscall_times: tcb.syscall_times,
            times: get_time_ms() - tcb.start_time,
        };
    } 
    0
}