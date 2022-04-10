//! Process management syscalls

use crate::config::{MAX_APP_NUM, MAX_SYSCALL_NUM};
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,TASK_MANAGER};
use crate::timer::get_time_us;

const SYSCALL_TASK_INFO: usize = 410;
const SYSCALL_GET_TIME: usize = 169;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub struct TaskInfo {
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    /// Change the status of current `Running` task into `Exited`.
    // fn mark_current_exited(&self) {
    let inner = TASK_MANAGER.inner.exclusive_access();
    let current = inner.current_task;
    // let current_task_status=inner.tasks[current].task_status;
    let current_task_start_time=inner.tasks[current].start_time;
    let current_task_syscall_times=inner.tasks[current].syscall_times;
    // println!("{}",inner.tasks[current].syscall_times[SYSCALL_TASK_INFO]);
    
    // }
    // println!("{}",current_task_start_time);

    
    unsafe{
        *ti=TaskInfo{
            status:TaskStatus::Running,
            // status:current_status,
            syscall_times: current_task_syscall_times,//[12; MAX_SYSCALL_NUM],//inner.tasks[current].syscall_times,//,//[0; MAX_SYSCALL_NUM],
            time: get_time_us()/1_000-current_task_start_time/1_000,
        };
        // (*ti).syscall_times[SYSCALL_TASK_INFO]=inner.tasks[current].syscall_times[SYSCALL_TASK_INFO];
        // println!("info {}",inner.tasks[current].syscall_times[SYSCALL_TASK_INFO]);
    }
    // println!("info {}",inner.tasks[current].syscall_times[SYSCALL_TASK_INFO]);
    // println!("time {}",inner.tasks[current].syscall_times[SYSCALL_GET_TIME]);
    drop(inner);
    0
}
