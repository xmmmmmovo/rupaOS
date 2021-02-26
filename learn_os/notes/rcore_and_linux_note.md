## 清华大学rcore & Linux内核笔记

### 内存

#### 内存空间

1. `MMU`：完成的工作就是虚拟地址到物理地址的转换，可以让系统中的多个程序跑在自己独立的虚拟地址空间中，相互不会影响。程序可以对底层的物理内存一无所知，物理地址可以是不连续的，但是不妨碍映射连续的虚拟地址空间。
2. `TLB`：`MMU`工作的过程就是查询页表的过程，页表放置在内存中时查询开销太大，因此专门有一小片访问更快的区域用于存放`地址转换条目`，用于提高查找效率。当页表内容有变化的时候，需要清除`TLB`，以防止地址映射出错。
3. `Cache`：处理器和存储器之间的缓存机制，用于提高访问速率，在ARMv8上会存在多级Cache，其中`L1 Cache`分为`指令Cache`和`数据Cache`，在`CPU Core`的内部，支持虚拟地址寻址；`L2 Cache`容量更大，同时存储指令和数据，为多个`CPU Core`共用，这多个`CPU Core`也就组成了一个`Cluster`。

下图浅黄色部分描述的就是一个地址转换的过程。
![img](https://cdn.jsdelivr.net/gh/xmmmmmovo/ResourcesBackup/blog/1771657-20190825003339798-684155442-20210225103904611.png)

由于上图没有体现出`L1和L2 Cache`和`MMU`的关系，所以再来一张图吧：
![img](https://cdn.jsdelivr.net/gh/xmmmmmovo/ResourcesBackup/blog/1771657-20190825003354426-1552689730-20210225103917902.png)

那具体是怎么访问的呢？再来一张图：
![img](https://cdn.jsdelivr.net/gh/xmmmmmovo/ResourcesBackup/blog/1771657-20190825003407960-1065344550.png)

![img](https://cdn.jsdelivr.net/gh/xmmmmmovo/ResourcesBackup/blog/1771657-20190825003429333-168689583.png)

#### 内存模型

分为`PFN(physical frame number)`和`NUMA`，其中`PFN`比较简单，以帧为基本进行分配，就是常说的分页式管理，而`NUMA`则是非一致性内存访问，是为了解决多核处理器一致性访问冲突时内存带宽变为瓶颈的解决方案。

PFN:

![img](https://cdn.jsdelivr.net/gh/xmmmmmovo/ResourcesBackup/blog/1771657-20190915182104342-747734013.png)

NUMA:

![img](https://cdn.jsdelivr.net/gh/xmmmmmovo/ResourcesBackup/blog/1771657-20190915182130622-1496868483.png)

同时Linux提供了三种内存模型：

![img](https://cdn.jsdelivr.net/gh/xmmmmmovo/ResourcesBackup/blog/1771657-20190915182145277-611308778.png)





