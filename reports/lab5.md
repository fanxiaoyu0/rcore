<center><h1>lab5</h1></center>

<center><h6>范逍宇 2019013273</h6></center>

### 一、实验。

使用银行家算法实现检测死锁的功能。

修改 sys_enable_deadlock_detect，根据 _enabled 决定是否启用检测死锁的功能。修改 sys_mutex_lock，当检测到死锁时返回 -0xDEAD。增加 is_dead_mutex 函数，判断是否出现死锁，算法的具体过程在实验指导书中已经介绍。修改 sys_mutex_create 函数，根据 id 维护 mutex_list。

在 Mutex 中增加 is_locked 和 update 函数，分别用于返回和更新 mutex_alloc 的状态。

在 Semaphore 中增加 update 函数，用于更新 sem_alloc 和 sem_need。

在 ProcessControlBlockInner 中，加入 detect 变量。在 SemaphoreInner 中，加入 id 变量。在 TaskControlBlockInner 中，加入 mutex_alloc，mutex_need，sem_alloc，sem_need 等变量，用于支持上述算法，注意按要求对这些变量进行初始化。 

本次实验大约用时 20h 。

### 二、问答题。

1.在我们的多线程实现中，当主线程 (即 0 号线程) 退出时，视为整个进程退出， 此时需要结束该进程管理的所有线程并回收其资源。 - 需要回收的资源有哪些？ - 其他线程的 TaskControlBlock 可能在哪些位置被引用，分别是否需要回收，为什么？

需要回收线程的用户态栈、用于系统调用和异常处理的跳板页、内核栈等资源。其他线程的 TaskControlBlock 在锁机制、信号量机制、条件变量机制的实现时可能被引用。需要回收。

2.对比以下两种 `Mutex.unlock` 的实现，二者有什么区别？这些区别可能会导致什么问题？

```rust
impl Mutex for Mutex1 {
    fn unlock(&self) {
        let mut mutex_inner = self.inner.exclusive_access();
        assert!(mutex_inner.locked);
        mutex_inner.locked = false;
        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
            add_task(waking_task);
        }
    }
}

impl Mutex for Mutex2 {
    fn unlock(&self) {
        let mut mutex_inner = self.inner.exclusive_access();
        assert!(mutex_inner.locked);
        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
            add_task(waking_task);
        } else {
            mutex_inner.locked = false;
        }
    }
}

```

Mutex1 先解除互斥锁，再唤醒等待线程。Mutex2 先唤醒等候线程，再解除互斥锁。前者可能导致互斥锁解除后被其他线程使用产生混乱。后者可能导致等候线程被唤醒后互斥锁仍未解除。