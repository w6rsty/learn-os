mod context;     
mod switch;
pub mod task;

use crate::{
    config::{MAX_APP_NUM, MAX_SYSCALL_NUM},
    loader::{get_num_app, init_app_cx},
    sbi::shutdown,
    sync::UPSafeCell,
};
use task::{TaskControlBlock, TaskStatus};
use lazy_static::*;
pub use context::TaskContext;
use switch::__switch;

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

impl TaskManager {
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1) // Search in a ring
            .map(|id| id % self.num_app)
            .find(|id| {
                // Find first ready task
                inner.tasks[*id].task_status == TaskStatus::Ready
            })
    }

    // Run the first task in list
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);
        let mut unused = TaskContext::zero_init();
        unsafe {
            // Switch from an empty task to first task
            __switch(&mut unused as *mut TaskContext, next_task_cx_ptr);
        }
        panic!("Unreachable in run_first_task!");
    }

    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[current].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            println!("All applications completed!");
            shutdown(false);
        }
    }

    fn get_current_task_tcb(&self) -> TaskControlBlock {
        let inner = self.inner.exclusive_access();
        inner.tasks[inner.current_task]
    }

    fn increate_current_task_syscall_times(&self, syscall_id: usize) {
        if syscall_id >= MAX_SYSCALL_NUM {
            return;
        }
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].syscall_times[syscall_id] += 1;
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        // Create TaskControlBlocks
        let mut tasks = [
            TaskControlBlock {
                task_cx: TaskContext::zero_init(),
                task_status: TaskStatus::UnInit,
                syscall_times: [0; MAX_SYSCALL_NUM],
                start_time: 0
            }; MAX_APP_NUM
        ];
        // Setup TaskContext
        for (i, task) in tasks.iter_mut().enumerate() {
            task.task_cx = TaskContext::goto_restore(init_app_cx(i));
            task.task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: unsafe { UPSafeCell::new(TaskManagerInner {
                tasks,
                current_task: 0
            })},
        }
    };
}

pub fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

pub fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

pub fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn get_current_task_tcb() -> TaskControlBlock {
    TASK_MANAGER.get_current_task_tcb()
}

pub fn increase_current_task_syscall_times(syscall_id: usize) {
    TASK_MANAGER.increate_current_task_syscall_times(syscall_id);
}