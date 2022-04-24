//! Process management syscalls

use crate::config::MAX_SYSCALL_NUM;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,TASK_MANAGER};
use crate::timer::get_time_us;
use crate::mm::address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use crate::task::current_user_token;
use crate::mm::page_table::{PTEFlags, PageTable};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

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

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let _us = get_time_us();
    let s=VirtAddr::from(_ts as usize);
    let pt=PageTable::from_token(current_user_token());
    let ppn=pt.translate(s.floor()).unwrap().ppn().0;
    let t=ppn<<12|s.page_offset();
    unsafe {
        *(t as *mut TimeVal) = TimeVal {
            sec: _us / 1_000_000,
            usec: _us % 1_000_000,
        };
    }
    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    -1
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    -1
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let s=VirtAddr::from(ti as usize);
    let pt=PageTable::from_token(current_user_token());
    let ppn=pt.translate(s.floor()).unwrap().ppn().0;
    let t=ppn<<12|s.page_offset();
    // Change the status of current `Running` task into `Exited`.
    let inner = TASK_MANAGER.inner.exclusive_access();
    let current = inner.current_task;
    unsafe{
        *(t as *mut TaskInfo)=TaskInfo{
            status:inner.tasks[current].task_status,
            syscall_times: inner.tasks[current].syscall_times,
            time: get_time_us()/1_000-inner.tasks[current].start_time/1_000,
        };
    }
    drop(inner);
    0
}
