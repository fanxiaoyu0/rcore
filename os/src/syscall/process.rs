//! Process management syscalls

use crate::config::MAX_SYSCALL_NUM;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,TASK_MANAGER};
use crate::timer::get_time_us;
use crate::mm::address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use crate::task::current_user_token;
use crate::mm::page_table::{PTEFlags, PageTable};
use crate::mm::frame_alloc;
use crate::config::{PAGE_SIZE, PAGE_SIZE_BITS};

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
    let virtual_address=VirtAddr::from(_ts as usize);
    // println!("time");
    let token=current_user_token();
    // println!("time2");
    let page_table=PageTable::from_token(token);
    let ppn=page_table.translate(virtual_address.floor()).unwrap().ppn().0;
    let physical_address=ppn<<12|virtual_address.page_offset();
    unsafe {
        *(physical_address as *mut TimeVal) = TimeVal {
            sec: _us / 1_000_000,
            usec: _us % 1_000_000,
        };
    }
    // drop(token);
    // drop(ppn);
    // drop(page_table);
    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    // println!("1");
    if _start&0xfff!=0{
        return -1;
    }
    if _port & !0x7 != 0 {
        return -1;
    }
    if _port & 0x7 == 0 {
        return -1;
    }
    // println!("1.2");
    // [start, start + len) 中存在已经被映射的页
    let t=current_user_token();
    // println!("1.3");
    let page_table=PageTable::from_token(t);
    // println!("1.5");
    let pages=(_len+PAGE_SIZE-1)/PAGE_SIZE;
    for i in 0..pages{
        let vpn=VirtAddr::from(_start+i*PAGE_SIZE).floor();
        if !page_table.translate(vpn).is_none(){
            return -1;
        }
    }
    // println!("2");
    for i in 0..pages{
        // 物理内存不足, 注意因为这种情况造成的分配失败，没有回收已经分配过的内存
        let temp=frame_alloc();
        if temp.is_none(){
            return -1;
        }
        let vpn=VirtAddr::from(_start+i*PAGE_SIZE).floor();
        let ppn=temp.unwrap().ppn;
        let mut flags=PTEFlags::U|PTEFlags::V;
        if _port&0x1==1{
            flags=flags|PTEFlags::R;
        }
        if _port&0x2==1{
            flags=flags|PTEFlags::W;
        }
        if _port&0x4==1{
            flags=flags|PTEFlags::X;
        }
        // page_table.map(vpn,ppn,flags);
    }
    drop(page_table);
    0
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    0
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let virtual_address=VirtAddr::from(ti as usize);
    // println!("info1");
    let token=current_user_token();
    // println!("info2");
    let page_table=PageTable::from_token(token);
    let ppn=page_table.translate(virtual_address.floor()).unwrap().ppn().0;
    let physical_address=ppn<<12|virtual_address.page_offset();
    // Change the status of current `Running` task into `Exited`.
    let inner = TASK_MANAGER.inner.exclusive_access();
    let current = inner.current_task;
    unsafe{
        *(physical_address as *mut TaskInfo)=TaskInfo{
            status:TaskStatus::Exited,
            syscall_times: inner.tasks[current].syscall_times,
            time: get_time_us()/1_000-inner.tasks[current].start_time/1_000,
        };
        drop(inner);
    }
    
    drop(token);
    drop(ppn);
    drop(page_table);
    0
}
