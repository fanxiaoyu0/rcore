//! Implementation of [`TaskManager`]
//!
//! It is only used to manage processes and schedule process based on ready queue.
//! Other CPU process monitoring functions are in Processor.


use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;

pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

// YOUR JOB: FIFO->Stride
/// A simple FIFO scheduler.
impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    // pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
    //     let mut min_stride_task_index = 0;
    //     let mut min_stride = self.ready_queue[0].inner_exclusive_access().stride;
    //     for i in 0..self.ready_queue.len(){
    //         let task = self.ready_queue[i].inner_exclusive_access();
    //         if task.stride < min_stride {
    //             min_stride = task.stride;
    //             min_stride_task_index = i;
    //         }
    //     }
    //     return self.ready_queue.swap_remove_front(min_stride_task_index);
    // }
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        let mut flag=0;
        let mut max=10000000 as usize;
        for i in 0..self.ready_queue.len(){
            if self.ready_queue.get(i).unwrap().inner_exclusive_access().stride<=max{
                flag=i;
                max=self.ready_queue.get(i).unwrap().inner_exclusive_access().stride;
            }
        }
        let big_stride:isize=999999;
        let prior=self.ready_queue.get(flag).unwrap().inner_exclusive_access().priority as isize;
        self.ready_queue.get(flag).unwrap().inner_exclusive_access().stride+=(big_stride/prior) as usize;
        self.ready_queue.swap(0,flag);
        self.ready_queue.pop_front()
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.exclusive_access().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.exclusive_access().fetch()
}
