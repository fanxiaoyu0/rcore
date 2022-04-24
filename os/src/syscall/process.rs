//! Process management syscalls

use crate::config::MAX_SYSCALL_NUM;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,TASK_MANAGER};
use crate::timer::get_time_us;
use crate::mm::address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use crate::task::current_user_token;
use crate::mm::page_table::{PTEFlags, PageTable};
use crate::mm::frame_alloc;

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
    if _start&0xfff!=0{
        return -1;
    }
    if _port & !0x7 != 0 {
        return -1;
    }
    if _port & 0x7 == 0 {
        return -1;
    }
    let ppn=frame_alloc().unwrap().ppn;
    // let pte_flags = ;
    let mut t=_len/4096;
    for i in 1..(t.floor()){
        page_table.map(vpn, ppn, pte_flags);
    }
    
    // let frame = frame_alloc().unwrap();
// 11        PageTable {
// 12            root_ppn: frame.ppn,
// 13            frames: vec![frame],
// 14        }
    
    // fn frame_dealloc(ppn: PhysPageNum) {
        // FRAME_ALLOCATOR.exclusive_access().dealloc(ppn);
    // }
        // [start, start + len) 中存在已经被映射的页
// 
// 物理内存不足
    //     5        let ppn: PhysPageNum;
//     6        match self.map_type {
//     7            MapType::Identical => {
//     8                ppn = PhysPageNum(vpn.0);
//     9            }
//    10            MapType::Framed => {
//    11                let frame = frame_alloc().unwrap();
//    12                ppn = frame.ppn;
//    13                self.data_frames.insert(vpn, frame);
//    14            }
//    15        }
//    16        
// pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
//     let pte = self.find_pte_create(vpn).unwrap();
//     assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);
//     *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
// }
// pub fn unmap(&mut self, vpn: VirtPageNum) {
//     let pte = self.find_pte_create(vpn).unwrap();
//     assert!(pte.is_valid(), "vpn {:?} is invalid before unmapping", vpn);
//     *pte = PageTableEntry::empty();
// }

// pub fn map_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
//     5        let ppn: PhysPageNum;
//     6        match self.map_type {
//     7            MapType::Identical => {
//     8                ppn = PhysPageNum(vpn.0);
//     9            }
//    10            MapType::Framed => {
//    11                let frame = frame_alloc().unwrap();
//    12                ppn = frame.ppn;
//    13                self.data_frames.insert(vpn, frame);
//    14            }
//    15        }
//    16        let pte_flags = PTEFlags::from_bits(self.map_perm.bits).unwrap();
//    17        page_table.map(vpn, ppn, pte_flags);
//    18    }
//    19    pub fn unmap_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
//    20        match self.map_type {
//    21            MapType::Framed => {
//    22                self.data_frames.remove(&vpn);
//    23            }
//    24            _ => {}
//    25        }
//    26        page_table.unmap(vpn);
//    27    }

    0
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    0
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let virtual_address=VirtAddr::from(ti as usize);
    let page_table=PageTable::from_token(current_user_token());
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
    }
    drop(inner);
    0
}
