use crate::sync::{Condvar, Mutex, MutexBlocking, MutexSpin, Semaphore};
use crate::task::{block_current_and_run_next, current_process, current_task};
use crate::timer::{add_timer, get_time_ms};
use alloc::sync::Arc;
use alloc::vec::Vec;

pub fn sys_sleep(ms: usize) -> isize {
    let expire_ms = get_time_ms() + ms;
    let task = current_task().unwrap();
    add_timer(expire_ms, task);
    block_current_and_run_next();
    0
}

// LAB5 HINT: you might need to maintain data structures used for deadlock detection
// during sys_mutex_* and sys_semaphore_* syscalls
pub fn sys_mutex_create(blocking: bool) -> isize {
    let process = current_process();
    let mut inner=process.inner_exclusive_access();
    if let Some(id)=inner
        .mutex_list
        .iter()
        .enumerate()
        .find(|(_, item)| item.is_none())
        .map(|(id, _)| id)
    {
        let mutex: Option<Arc<dyn Mutex>> = if !blocking {
            Some(Arc::new(MutexSpin::new(id)))
        } 
        else {
            Some(Arc::new(MutexBlocking::new(id)))
        };
        inner.mutex_list[id]=mutex;
        return id as isize;
    } 
    else {
        let mutex: Option<Arc<dyn Mutex>> = if !blocking {
            Some(Arc::new(MutexSpin::new(inner.mutex_list.len())))
        } 
        else {
            Some(Arc::new(MutexBlocking::new(inner.mutex_list.len())))
        };
        inner.mutex_list.push(mutex);
        return inner.mutex_list.len() as isize-1;
    }
}
pub fn is_dead_sem(detect:usize)->bool{
    if detect==0 {
        return true;
    }
    if detect!=1 {
        return false;
    }
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    let task_count=inner.tasks.len();
    let mut work: Vec<usize> = Vec::new();
    let mut finish: Vec<bool> = Vec::new();
    for _i in 0..task_count {
        finish.push(false);
    }
    for i in 0..inner.semaphore_list.len(){
        if let Some(sem) = &mut inner.semaphore_list[i]{
            if sem.inner.exclusive_access().count>0{
                work.push(sem.inner.exclusive_access().count as usize);
                continue;
            }
        }
        work.push(0);
    }
    loop{
        let mut temp=false;
        let inner_task=&mut inner.tasks;
        for i in 0..task_count{
            if finish[i]{
                continue;
            }
            let mut f=false;
            if let Some(task)=&mut inner_task[i]{
                {
                    for j in 0..work.len(){
                        if task.inner_exclusive_access().sem_need[j]>work[j] as isize{
                            f=true;
                            break;
                        }
                    }
                    if f {
                        continue;
                    }
                    temp=true;
                    finish[i]=true;
                    for j in 0..work.len(){
                        work[j]+=task.inner_exclusive_access().sem_alloc[j];
                    }
                    
                }
            }
        }
        if !temp{
            break;
        }
    }
    for i in 0..task_count{
        if !finish[i]{
            return false;
        }
    }
    return true;
}

pub fn is_dead_mutex(detect:usize)->bool{
    if detect==0{
        return true;
    }
    if detect!=1{
        return false;
    }
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    let task_count=inner.tasks.len();
    let mut work: Vec<usize> = Vec::new();
    let mut finish: Vec<bool> = Vec::new();
    for _i in 0..task_count {
        finish.push(false);
    }
    for i in 0..inner.mutex_list.len(){
        if let Some(mtx) = &mut inner.mutex_list[i]{
            if mtx.is_locked()==1{
                work.push(1);
            }
        }
        work.push(0);
    }
    loop {
        let mut temp = false;
        let current_task=&mut inner.tasks;
        for i in 0..task_count{
            if finish[i]{
                continue;
            }
            if let Some(task)=&mut current_task[i]{
                let mut f=false;
                {
                    for j in 0..work.len(){
                        if task.inner_exclusive_access().mutex_need[j]>work[j]{
                            f=true;
                            break;
                        }
                    }
                }
                if f {
                    continue;
                }
                temp=true;
                finish[i]=true;
                for j in 0..work.len(){
                    work[j]+=task.inner_exclusive_access().mutex_alloc[j]
                }
            }
        }
        if !temp{
            break;
        }
    }
    for i in 0..task_count{
        if !finish[i]{
            return false;
        }
    }
    return true;
    
}
// LAB5 HINT: Return -0xDEAD if deadlock is detected
pub fn sys_mutex_lock(mutex_id: usize) -> isize {
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    let tem=process_inner.detect;
    drop(process_inner);
    drop(process);
    mutex.update();
    if !is_dead_mutex(tem){
        return -0xDEAD;
    }
    mutex.lock();
    return 0;
}

pub fn sys_mutex_unlock(mutex_id: usize) -> isize {
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    drop(process_inner);
    drop(process);
    mutex.unlock();
    0
}

pub fn sys_semaphore_create(res_count: usize) -> isize {
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    let id = if let Some(id) = process_inner
        .semaphore_list
        .iter()
        .enumerate()
        .find(|(_, item)| item.is_none())
        .map(|(id, _)| id)
    {
        process_inner.semaphore_list[id] = Some(Arc::new(Semaphore::new(res_count,id)));
        id
    } else {
        let tem=process_inner.semaphore_list.len();
        process_inner
            .semaphore_list
            .push(Some(Arc::new(Semaphore::new(res_count,tem))));
        process_inner.semaphore_list.len() - 1
    };
    id as isize
}

pub fn sys_semaphore_up(sem_id: usize) -> isize {
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let sem = Arc::clone(process_inner.semaphore_list[sem_id].as_ref().unwrap());
    drop(process_inner);
    sem.up();
    0
}

// LAB5 HINT: Return -0xDEAD if deadlock is detected
pub fn sys_semaphore_down(sem_id: usize) -> isize {
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let sem = Arc::clone(process_inner.semaphore_list[sem_id].as_ref().unwrap());
    let temp = process_inner.detect;
    drop(process_inner);
    sem.update();
    if !is_dead_sem(temp){
        return -0xDEAD;
    }
    sem.down();
    0
}

pub fn sys_condvar_create(_arg: usize) -> isize {
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    let id = if let Some(id) = process_inner
        .condvar_list
        .iter()
        .enumerate()
        .find(|(_, item)| item.is_none())
        .map(|(id, _)| id)
    {
        process_inner.condvar_list[id] = Some(Arc::new(Condvar::new()));
        id
    } else {
        process_inner
            .condvar_list
            .push(Some(Arc::new(Condvar::new())));
        process_inner.condvar_list.len() - 1
    };
    id as isize
}

pub fn sys_condvar_signal(condvar_id: usize) -> isize {
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let condvar = Arc::clone(process_inner.condvar_list[condvar_id].as_ref().unwrap());
    drop(process_inner);
    condvar.signal();
    0
}

pub fn sys_condvar_wait(condvar_id: usize, mutex_id: usize) -> isize {
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let condvar = Arc::clone(process_inner.condvar_list[condvar_id].as_ref().unwrap());
    let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    drop(process_inner);
    condvar.wait(mutex);
    0
}

// LAB5 YOUR JOB: Implement deadlock detection, but might not all in this syscall
pub fn sys_enable_deadlock_detect(_enabled: usize) -> isize {
    if _enabled==0 ||_enabled==1{
        let process=current_process();
        let mut _inner=process.inner_exclusive_access();
        _inner.detect=_enabled;
        return 1;
    }
    return -1;
}
