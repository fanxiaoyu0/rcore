<center><h1>lab4</h1></center>

<center><h6>范逍宇 2019013273</h6></center>

### 一、实验。

#### 0.前向兼容。

本章要求兼容之前实现的功能，即 sys_get_time、sys_task_info、sys_mmap、sys_munmap、sys_spawn 这几个系统调用，但其实这几个函数几乎没有变化，照搬之前的实现即可，不再赘述。

#### 1.sys_linkat。

先比较文件名字，若文件名相同，则报错。遍历找到旧文件路径的 inode number，结合新的文件路径，创建一个新的 Direntry，写入到 root inode。

#### 2.sys unlinkat。

遍历找到目标文件路径的 inode number，在 root inode 将相应位置置为 Direntry: empty，这样就完成了删除。注意如果只剩一个链接，则应删除文件内容。

#### 3.sys stat。

注意首先需要通过进行地址的转换（这一点和实现 task_info 时相似）。然后对 stat 的各个变量赋值，具体地说，通过判断 disk_node.type 实现对 mode 的赋值，通过计算 block id 和 offset 实现对 ino 的赋值。

### 二、问答题。

在我们的easy-fs中, root inode起着什么作用?如果 root inode中的内容损坏了,会发生什么?

```
root inode 是根目录的 inode，用于查找根目录下的文件和文件索引。
如果 root inode 损坏，则无法再找到根目录下的文件，
```



