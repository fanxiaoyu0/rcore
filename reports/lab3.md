<center><h1>lab3</h1></center>

<center><h6>范逍宇 2019013273</h6></center>

### 实验

#### 前向兼容

本章要求兼容之前实现的功能，即 sys_get_time、sys_task_info、sys_mmap、sys_munmap 这四个系统调用，但其实这四个函数几乎没有变化，照搬之前的实现即可，不再赘述。

#### spawn

只需对原来的 fork 函数稍作修改，即可实现 spawn 函数。具体来说，在创建 memory_set 时，不是使用父进程的 memory_set，而是根据 elf 文件创建，应用数据的基地址不再使用父进程的基地址，而是使用从 elf 文件中解析出的基地址，trap 的 entry_point 也不再使用父进程的 entry_point，而是使用从 elf 文件中解析出的 entry_point。这样，在 sys_spawn 函数中调用实现的 spawn 函数即可。

#### stride

要实现 stride 算法，需要向 TaskControlBlockInner 中加入 stride、pass、priority 字段，在 sys_set_priority 函数中根据算法设置 pass 和 priority 字段（为了避免发生反转现象，这里 BIG_STRIDE 取 $10^6$）。在每次 fetch 一个新的任务时，之前的做法是直接从队头取出一个 task，而根据 stride 算法的要求，现在需要找出 stride 最小的任务（按照实验指导书上的提示，这里没有采用堆排序的方法，而是直接遍历所有任务找最小值），然后将它 pop 出去。在启动该任务时，需要更新 stride。最终六个任务的 $runtimes/priority$ 的数值为：3585333, 3602200, 3608080, 3611200, 3601422, 3650120，可以认为满足公平性要求。

-----------

### 问答题

stride 算法原理非常简单，但是有一个比较大的问题。例如两个 pass = 10 的进程，使用 8bit 无符号整形储存 stride， p1.stride = 255, p2.stride = 250，在 p2 执行一个时间片后，理论上下一次应该 p1 执行。

- 实际情况是轮到 p1 执行吗？为什么？

  ```
  不是，因为 p2.stride+p2.pass=260%256=4，因为整数溢出所以 p2.stride 反而变得更小，而且在接下来的时间片内都是 p2.stride 更小，因为 p1.stride=255 是 8bit 无符号整数中最大的数，所以 p2.stride 最多也就是与其相等，这样 p2 的执行时间就远远大于 p1 的执行时间，不满足公平性要求。
  ```

  

我们之前要求进程优先级 >= 2 其实就是为了解决这个问题。可以证明， **在不考虑溢出的情况下** , 在进程优先级全部 >= 2 的情况下，如果严格按照算法执行，那么 STRIDE_MAX – STRIDE_MIN <= BigStride / 2。

- 为什么？尝试简单说明（不要求严格证明）。

  ```
  因为 pass=BigStride/priority，而 priority>=2，所以 pass<=BigStride/2。
  任取两个进程 p1,p2，因为 p1,p2 是任取的，
  只要证明 |p1.stride-p2.stride|<=BigStride/2，就说证明 STRIDE_MAX–STRIDE_MIN <= BigStride/2。
  不妨设 p1.pass<p2.pass，则 p1.pass<p2.pass<=BigStride/2。
  使用数学归纳法，初始时 p1.stride=p2.stride=0，满足 |p1.stride-p2.stride|<=BigStride/2。
  某一时间片轮到 p1 执行，假设此时有 |p1.stride-p2.stride|<=BigStride/2。
  因为是轮到 p1 执行，所以 p1.stride<=p2.stride。
  接下来 p1.new_stride=p1.stride+p1.pass，考虑 p1.new_stride 和 p2.stride 的相对大小，一共有两种情况：
  若 p1.new_stride<=p2.stride，则 
  |p1.new_stride-p2.stride| = p2.stride-p1.new_stride < p2.stride-p1.strid <= BigStride/2 
  且接下来仍然是 p1 先执行，可以重复上述分析。
  若 p1.new_stride>p2.stride，则
  |p1.new_stride-p2.stride| = p1.new_stride-p2.stride 
  = -(p2.stride-p1.strid)+p1.pass < p1.pass <= BigStride/2
  接下来轮到 p2 先执行。
  p2.new_stride=p2.stride+p2.pass，由于 p2.pass>p1.pass，因此 p2.pass+p2.stride>p1.pass+p1.stride，
  即 p2.new_stride>p1.new_stride，这时
  |p1.new_stride-p2.new_stride|=p2.new_stride-p1.new_stride
  = -(p1.new_stride-p2.strid)+p2.pass < p2.pass <= BigStride/2
  接下来轮到 p1 先执行，可以重复上述分析。
  综上，无论状态如何转移，均有 |p1.stride-p2.stride|<=BigStride/2，故：
  STRIDE_MAX–STRIDE_MIN <= BigStride/2 得证。
  ```

- 已知以上结论，**考虑溢出的情况下**，可以为 Stride 设计特别的比较器，让 BinaryHeap<Stride> 的 pop 方法能返回真正最小的 Stride。补全下列代码中的 `partial_cmp` 函数，假设两个 Stride 永远不会相等。

```rust
use core::cmp::Ordering;

struct Stride(u64);

// 在不溢出的情况下，有 STRIDE_MAX – STRIDE_MIN <= BigStride/2，这里设 BigStride=u64::max_value()
// 若两数之差的绝对值小于 BigStride/2，说明没有发生溢出，真实的大小关系应该和现在的大小关系相同 
// 若两数之差的绝对值大于 BigStride/2，说明发生了溢出，真实的大小关系应该和现在的大小关系相反 
impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a=u64::max_value()/2;
        if self.0 < other.0 {
            if other.0-self.0<=a {
                return Some(Ordering::Less);
            } 
            else {
                return Some(Ordering::Greater);
            }
        } 
        else {
            if self.0-other.0<=a {
                return Some(Ordering::Greater);
            } 
            else {
                return Some(Ordering::Less);
            }
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```

TIPS: 使用 8 bits 存储 stride, BigStride = 255, 则: `(125 < 255) == false`, `(129 < 255) == true`.

--------------

### 感想和建议

由于 lab3 的测例 ch5_usertest 是通过 spawn 创建新进程实现的，因此如果仅仅完成了前向兼容的系统调用的移植，而没有实现 spawn 函数，是不能通过之前章节的测例的（当然在 user shell 中单独输入之前的任务名是可以运行测试的）。事实上，我也是偶然间修改了 spawn 函数才发现了这一点，在此之前一直认为是自己对前向兼容的系统调用的移植有问题。虽然这一点可以交由同学们自己发现，但我认为实验指导书中应该说明 ch5_usertest 的这个性质。

