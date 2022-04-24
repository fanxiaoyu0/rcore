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
    let page_table=PageTable::from_token(current_user_token());
    let ppn=page_table.translate(virtual_address.floor()).unwrap().ppn().0;
    let physical_address=ppn<<12|virtual_address.page_offset();
    unsafe {
        *(physical_address as *mut TimeVal) = TimeVal {
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
    let mut page_table=PageTable::from_token(current_user_token());
    let pages=(_len+PAGE_SIZE-1)/PAGE_SIZE;
    for i in 0..pages{
        let vpn=VirtAddr::from(_start+i*PAGE_SIZE).floor();
        let temp=page_table.find_pte(vpn);
        if !temp.is_none(){
            if temp.unwrap().is_valid(){
                return -1;
            }
        }
        // if (!temp.is_none())&&{
            // return -1;
        // }
    }
    // println!("pages:{}",pages);
    for i in 0..pages{
        // 物理内存不足, 注意因为这种情况造成的分配失败，没有回收已经分配过的内存
        let temp=frame_alloc();
        if temp.is_none(){
            return -1;
        }
        let vpn=VirtAddr::from(_start+i*PAGE_SIZE).floor();
        // println!("vpn:{:?}",vpn);
        let ppn=temp.unwrap().ppn;
        // println!("ppn:{:?}",ppn);
        let mut flags=PTEFlags::U;
        // println!("_port:{}",_port);
        if _port&0x1==0x1{
            flags=flags|PTEFlags::R;
        }
        // println!("_port:{}",_port);
        // println!("3&2:{}",0x3&0x2);
        if _port&0x2==0x2{
            flags=flags|PTEFlags::W;
            // println!("Write");
            // println!("flags:{:?}",flags);
        }
        if _port&0x4==0x4{
            flags=flags|PTEFlags::X;
        }
        // println!("flags:{:?}",flags);
        // println!("vpn:{:?}",vpn);
        // println!("ppn:{:?}",ppn);
        page_table.map(vpn,ppn,flags);
    }
    0
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    if _start&0xfff!=0{
        // println!("start");
        return -1;
    }
    let mut page_table=PageTable::from_token(current_user_token());
    let pages=(_len+PAGE_SIZE-1)/PAGE_SIZE;
    // println!("pages:{}",pages);
    for i in 0..pages{
        let vpn=VirtAddr::from(_start+i*PAGE_SIZE).floor();
        let temp=page_table.find_pte(vpn);
        if temp.is_none(){
            // println!("none");
            return -1;
        }
        // println!("{}",temp.unwrap().ppn().0);
        // println!("VPN:{:?}",vpn);
        if !(temp.unwrap().is_valid()){
            // println!("invalid");
            return -1;
        }
    }
    // println!("pages:{}",pages);
    for i in 0..pages{
        let vpn=VirtAddr::from(_start+i*PAGE_SIZE).floor();
        page_table.unmap(vpn);
    }
    0
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let virtual_address=VirtAddr::from(ti as usize);
    let page_table=PageTable::from_token(current_user_token());
    let ppn=page_table.translate(virtual_address.floor()).unwrap().ppn().0;
    let physical_address=ppn<<12|virtual_address.page_offset();
    let inner = TASK_MANAGER.inner.exclusive_access();
    let current = inner.current_task;
    unsafe{
        *(physical_address as *mut TaskInfo)=TaskInfo{
            // Change the status of current `Running` task into `Exited`.
            status:TaskStatus::Exited,
            syscall_times: inner.tasks[current].syscall_times,
            time: get_time_us()/1_000-inner.tasks[current].start_time/1_000,
        };
    }
    0
}
