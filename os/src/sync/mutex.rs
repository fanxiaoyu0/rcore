use super::UPSafeCell;
use crate::task::TaskControlBlock;
use crate::task::{add_task, current_task};
use crate::task::{block_current_and_run_next, suspend_current_and_run_next};
use alloc::{collections::VecDeque, sync::Arc};

pub trait Mutex: Sync + Send {
    fn lock(&self);
    fn unlock(&self);
    fn is_locked(&self)->usize;
    fn update(&self);
}

pub struct MutexSpin {
    inner: UPSafeCell<MutexSpinInner>,
    id: usize,
}
pub struct MutexSpinInner {
    locked: bool,
}

impl MutexSpin {
    pub fn new(id: usize) -> Self {
        Self {
            inner: unsafe {
                UPSafeCell::new(MutexSpinInner {
                    locked: false,
                })
            },
            id: id,
        }
    }
}

impl Mutex for MutexSpin {
    fn lock(&self) {
        loop {
            let mut inner=self.inner.exclusive_access();
            if inner.locked {
                drop(inner);
                suspend_current_and_run_next();
                continue;
            } 
            else {
                let current_task=current_task().unwrap();
                current_task.inner_exclusive_access().mutex_alloc[self.id]=1;
                current_task.inner_exclusive_access().mutex_need[self.id]=0;
                inner.locked = true;
                return;
            }
            
        }
    }
    fn unlock(&self) {
        let mut inner=self.inner.exclusive_access();
        let current_task=current_task().unwrap();
        current_task.inner_exclusive_access().mutex_alloc[self.id]=0;
        inner.locked=false;
    }
    fn is_locked(&self)->usize{
        if self.inner.exclusive_access().locked{
            return 0;
        }
        else{
            return 1;
        }
    }
    fn update(&self){
        let inner=self.inner.exclusive_access();
        let current_task=current_task().unwrap();
        if !inner.locked{
            current_task.inner_exclusive_access().mutex_alloc[self.id]=1;
        }
        else{
            drop(inner);
            current_task.inner_exclusive_access().mutex_need[self.id]=1;
        }
    }
}

pub struct MutexBlocking {
    inner: UPSafeCell<MutexBlockingInner>,
    id: usize,
}

pub struct MutexBlockingInner {
    locked: bool,
    wait_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl MutexBlocking {
    pub fn new(id:usize) -> Self {
        Self {
            inner: unsafe {
                UPSafeCell::new(MutexBlockingInner {
                    locked: false,
                    wait_queue: VecDeque::new(),
                })
            },
            id:id,
        }
    }
}

impl Mutex for MutexBlocking {
    fn lock(&self) {
        let mut mutex_inner = self.inner.exclusive_access();
        if mutex_inner.locked {
            mutex_inner.wait_queue.push_back(current_task().unwrap());
            drop(mutex_inner);
            block_current_and_run_next();
        } else {
            mutex_inner.locked = true;
        }
    }

    fn unlock(&self) {
        let mut mutex_inner = self.inner.exclusive_access();
        assert!(mutex_inner.locked);
        let current_task=current_task().unwrap();
        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
            waking_task.inner_exclusive_access().mutex_need[self.id]=0;
            waking_task.inner_exclusive_access().mutex_alloc[self.id]=1;
            current_task.inner_exclusive_access().mutex_alloc[self.id]=0;
            add_task(waking_task);
        } else {
            mutex_inner.locked = false;
        }
    }

    fn update(&self){
        let inner=self.inner.exclusive_access();
        let current_task=current_task().unwrap();
        if !inner.locked{
            current_task.inner_exclusive_access().mutex_alloc[self.id]=1;
            current_task.inner_exclusive_access().mutex_need[self.id]=0;   
        }
        else{
            drop(inner);
            current_task.inner_exclusive_access().mutex_need[self.id]=1;
        }
    }
    fn is_locked(&self)->usize {
        if self.inner.exclusive_access().locked {
            return 0;
        }
        else {
            return 1;
        }
    }
}
