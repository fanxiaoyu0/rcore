//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_TASK_INFO: usize = 410;

mod fs;
mod process;

use fs::*;
use process::*;
use crate::task::{TASK_MANAGER};

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    // LAB1: You may need to update syscall info here.
    // match syscall_id {
    //     _ => panic!("Unsupported syscall_id: {}", syscall_id),
    // }
    let mut inner = TASK_MANAGER.inner.exclusive_access();
    let current = inner.current_task;
    
    // let current_task_status=inner.tasks[current].task_status;
    // let current_task_start_time=inner.tasks[current].start_time;

    match syscall_id {
        SYSCALL_WRITE => {inner.tasks[current].syscall_times[SYSCALL_WRITE]+=1;drop(inner);sys_write(args[0], args[1] as *const u8, args[2])},
        SYSCALL_EXIT => {inner.tasks[current].syscall_times[SYSCALL_EXIT]+=1;drop(inner);sys_exit(args[0] as i32)},
        SYSCALL_YIELD => {inner.tasks[current].syscall_times[SYSCALL_YIELD]+=1;drop(inner);sys_yield()},
        SYSCALL_GET_TIME => {inner.tasks[current].syscall_times[SYSCALL_GET_TIME]+=1;drop(inner);sys_get_time(args[0] as *mut TimeVal, args[1])},
        SYSCALL_TASK_INFO => {inner.tasks[current].syscall_times[SYSCALL_TASK_INFO]+=1;drop(inner);sys_task_info(args[0] as *mut TaskInfo)},
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
