use crate::config::MAX_SYSCALL_NUM;

use super::context::TaskContext;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited
}

#[derive(Debug, Clone, Copy)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub start_time: usize
}