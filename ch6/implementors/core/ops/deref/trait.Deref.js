(function() {var implementors = {};
implementors["buddy_system_allocator"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"buddy_system_allocator/struct.LockedFrameAllocator.html\" title=\"struct buddy_system_allocator::LockedFrameAllocator\">LockedFrameAllocator</a>","synthetic":false,"types":["buddy_system_allocator::frame::LockedFrameAllocator"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"buddy_system_allocator/struct.LockedHeap.html\" title=\"struct buddy_system_allocator::LockedHeap\">LockedHeap</a>","synthetic":false,"types":["buddy_system_allocator::LockedHeap"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"buddy_system_allocator/struct.LockedHeapWithRescue.html\" title=\"struct buddy_system_allocator::LockedHeapWithRescue\">LockedHeapWithRescue</a>","synthetic":false,"types":["buddy_system_allocator::LockedHeapWithRescue"]}];
implementors["lock_api"] = [{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawMutex.html\" title=\"trait lock_api::RawMutex\">RawMutex</a> + 'a, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"lock_api/struct.MutexGuard.html\" title=\"struct lock_api::MutexGuard\">MutexGuard</a>&lt;'a, R, T&gt;","synthetic":false,"types":["lock_api::mutex::MutexGuard"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawMutex.html\" title=\"trait lock_api::RawMutex\">RawMutex</a> + 'a, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"lock_api/struct.MappedMutexGuard.html\" title=\"struct lock_api::MappedMutexGuard\">MappedMutexGuard</a>&lt;'a, R, T&gt;","synthetic":false,"types":["lock_api::mutex::MappedMutexGuard"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawMutex.html\" title=\"trait lock_api::RawMutex\">RawMutex</a> + 'a, G:&nbsp;<a class=\"trait\" href=\"lock_api/trait.GetThreadId.html\" title=\"trait lock_api::GetThreadId\">GetThreadId</a> + 'a, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"lock_api/struct.ReentrantMutexGuard.html\" title=\"struct lock_api::ReentrantMutexGuard\">ReentrantMutexGuard</a>&lt;'a, R, G, T&gt;","synthetic":false,"types":["lock_api::remutex::ReentrantMutexGuard"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawMutex.html\" title=\"trait lock_api::RawMutex\">RawMutex</a> + 'a, G:&nbsp;<a class=\"trait\" href=\"lock_api/trait.GetThreadId.html\" title=\"trait lock_api::GetThreadId\">GetThreadId</a> + 'a, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"lock_api/struct.MappedReentrantMutexGuard.html\" title=\"struct lock_api::MappedReentrantMutexGuard\">MappedReentrantMutexGuard</a>&lt;'a, R, G, T&gt;","synthetic":false,"types":["lock_api::remutex::MappedReentrantMutexGuard"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawRwLock.html\" title=\"trait lock_api::RawRwLock\">RawRwLock</a> + 'a, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"lock_api/struct.RwLockReadGuard.html\" title=\"struct lock_api::RwLockReadGuard\">RwLockReadGuard</a>&lt;'a, R, T&gt;","synthetic":false,"types":["lock_api::rwlock::RwLockReadGuard"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawRwLock.html\" title=\"trait lock_api::RawRwLock\">RawRwLock</a> + 'a, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"lock_api/struct.RwLockWriteGuard.html\" title=\"struct lock_api::RwLockWriteGuard\">RwLockWriteGuard</a>&lt;'a, R, T&gt;","synthetic":false,"types":["lock_api::rwlock::RwLockWriteGuard"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawRwLockUpgrade.html\" title=\"trait lock_api::RawRwLockUpgrade\">RawRwLockUpgrade</a> + 'a, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"lock_api/struct.RwLockUpgradableReadGuard.html\" title=\"struct lock_api::RwLockUpgradableReadGuard\">RwLockUpgradableReadGuard</a>&lt;'a, R, T&gt;","synthetic":false,"types":["lock_api::rwlock::RwLockUpgradableReadGuard"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawRwLock.html\" title=\"trait lock_api::RawRwLock\">RawRwLock</a> + 'a, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"lock_api/struct.MappedRwLockReadGuard.html\" title=\"struct lock_api::MappedRwLockReadGuard\">MappedRwLockReadGuard</a>&lt;'a, R, T&gt;","synthetic":false,"types":["lock_api::rwlock::MappedRwLockReadGuard"]},{"text":"impl&lt;'a, R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawRwLock.html\" title=\"trait lock_api::RawRwLock\">RawRwLock</a> + 'a, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"lock_api/struct.MappedRwLockWriteGuard.html\" title=\"struct lock_api::MappedRwLockWriteGuard\">MappedRwLockWriteGuard</a>&lt;'a, R, T&gt;","synthetic":false,"types":["lock_api::rwlock::MappedRwLockWriteGuard"]}];
implementors["os"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"os/mm/frame_allocator/struct.FRAME_ALLOCATOR.html\" title=\"struct os::mm::frame_allocator::FRAME_ALLOCATOR\">FRAME_ALLOCATOR</a>","synthetic":false,"types":["os::mm::frame_allocator::FRAME_ALLOCATOR"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"os/mm/memory_set/struct.KERNEL_SPACE.html\" title=\"struct os::mm::memory_set::KERNEL_SPACE\">KERNEL_SPACE</a>","synthetic":false,"types":["os::mm::memory_set::KERNEL_SPACE"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"os/task/manager/struct.TASK_MANAGER.html\" title=\"struct os::task::manager::TASK_MANAGER\">TASK_MANAGER</a>","synthetic":false,"types":["os::task::manager::TASK_MANAGER"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"os/task/pid/struct.PID_ALLOCATOR.html\" title=\"struct os::task::pid::PID_ALLOCATOR\">PID_ALLOCATOR</a>","synthetic":false,"types":["os::task::pid::PID_ALLOCATOR"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"os/task/processor/struct.PROCESSOR.html\" title=\"struct os::task::processor::PROCESSOR\">PROCESSOR</a>","synthetic":false,"types":["os::task::processor::PROCESSOR"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"os/task/struct.INITPROC.html\" title=\"struct os::task::INITPROC\">INITPROC</a>","synthetic":false,"types":["os::task::INITPROC"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"os/drivers/block/virtio_blk/struct.QUEUE_FRAMES.html\" title=\"struct os::drivers::block::virtio_blk::QUEUE_FRAMES\">QUEUE_FRAMES</a>","synthetic":false,"types":["os::drivers::block::virtio_blk::QUEUE_FRAMES"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"os/drivers/block/struct.BLOCK_DEVICE.html\" title=\"struct os::drivers::block::BLOCK_DEVICE\">BLOCK_DEVICE</a>","synthetic":false,"types":["os::drivers::block::BLOCK_DEVICE"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"os/fs/inode/struct.ROOT_INODE.html\" title=\"struct os::fs::inode::ROOT_INODE\">ROOT_INODE</a>","synthetic":false,"types":["os::fs::inode::ROOT_INODE"]}];
implementors["scopeguard"] = [{"text":"impl&lt;T, F, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"scopeguard/struct.ScopeGuard.html\" title=\"struct scopeguard::ScopeGuard\">ScopeGuard</a>&lt;T, F, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(T),<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"scopeguard/trait.Strategy.html\" title=\"trait scopeguard::Strategy\">Strategy</a>,&nbsp;</span>","synthetic":false,"types":["scopeguard::ScopeGuard"]}];
implementors["spin"] = [{"text":"impl&lt;T, F:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>() -&gt; T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"spin/lazy/struct.Lazy.html\" title=\"struct spin::lazy::Lazy\">Lazy</a>&lt;T, F&gt;","synthetic":false,"types":["spin::lazy::Lazy"]},{"text":"impl&lt;'a, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"spin/mutex/struct.SpinMutexGuard.html\" title=\"struct spin::mutex::SpinMutexGuard\">SpinMutexGuard</a>&lt;'a, T&gt;","synthetic":false,"types":["spin::mutex::spin::SpinMutexGuard"]},{"text":"impl&lt;'a, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"spin/mutex/struct.TicketMutexGuard.html\" title=\"struct spin::mutex::TicketMutexGuard\">TicketMutexGuard</a>&lt;'a, T&gt;","synthetic":false,"types":["spin::mutex::ticket::TicketMutexGuard"]},{"text":"impl&lt;'a, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"spin/mutex/struct.MutexGuard.html\" title=\"struct spin::mutex::MutexGuard\">MutexGuard</a>&lt;'a, T&gt;","synthetic":false,"types":["spin::mutex::MutexGuard"]},{"text":"impl&lt;'rwlock, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"spin/rw_lock/struct.RwLockReadGuard.html\" title=\"struct spin::rw_lock::RwLockReadGuard\">RwLockReadGuard</a>&lt;'rwlock, T&gt;","synthetic":false,"types":["spin::rw_lock::RwLockReadGuard"]},{"text":"impl&lt;'rwlock, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"spin/rw_lock/struct.RwLockUpgradableGuard.html\" title=\"struct spin::rw_lock::RwLockUpgradableGuard\">RwLockUpgradableGuard</a>&lt;'rwlock, T&gt;","synthetic":false,"types":["spin::rw_lock::RwLockUpgradableGuard"]},{"text":"impl&lt;'rwlock, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"spin/rw_lock/struct.RwLockWriteGuard.html\" title=\"struct spin::rw_lock::RwLockWriteGuard\">RwLockWriteGuard</a>&lt;'rwlock, T&gt;","synthetic":false,"types":["spin::rw_lock::RwLockWriteGuard"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()