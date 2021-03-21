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

### 虚拟内存

![img](https://cdn.jsdelivr.net/gh/xmmmmmovo/ResourcesBackup/blog/rcore_memory_layout.png)

#### Sv39

指的是总共`39`位来表示虚拟地址，当然其他可选配地址有:`36, 39, 42, 47`

![img](https://cdn.jsdelivr.net/gh/xmmmmmovo/ResourcesBackup/blog/sv39_address.png)

注意这里的物理页号是27位，虚拟页号27位，最后12位表示页内偏移。

#### 页表项

![img](https://cdn.jsdelivr.net/gh/xmmmmmovo/ResourcesBackup/blog/sv39_pte.jpg)

一个**页表项（PTE，Page Table Entry）**是用来描述一个虚拟页号如何映射到物理页号的。如果一个虚拟页号通过某种手段找到了一个页表项，并通过读取上面的物理页号完成映射，我们称这个虚拟页号通过该页表项完成映射的。

我们可以看到 Sv39 模式里面的一个页表项大小为 64 位（即 8 字节）。其中第 53-10 共 44 位为一个物理页号，表示这个虚拟页号映射到的物理页号。后面的第 9-0 位则描述页的相关状态信息。

- `V` 表示这个页表项是否合法。如果为 0 表示不合法，此时页表项其他位的值都会被忽略。

- `R,W,X` 分别表示是否可读（Readable）、可写（Writable）和可执行（Executable）。

  - 以 `W` 这一位为例，如果为零表示不可写，那么如果一条 `store` 的指令，它通过这个页表项完成了虚拟页号到物理页号的映射，找到了物理地址。但是仍然会报出异常，是因为这个页表项规定如果物理地址是通过它映射得到的，执行的行为和页表描述的状态并不一致。

  - 同时，根据 `R,W,X` 取值的不同，我们还有一些特别表示和约定：

    ![img](https://cdn.jsdelivr.net/gh/xmmmmmovo/ResourcesBackup/blog/sv39_rwx.jpg)

  - 也就是说，如果 `R,W,X` 均为 0，文档上说这表示这个页表项指向下一级页表，我们先暂时记住就好。

- `U` 为 1 表示用户态运行的程序可以通过该页表项完成地址映射。事实上用户态运行的程序也只能够通过 `U` 为 1 的页表项进行虚实地址映射。

  - 然而，我们所处在的 S 态也并不是理所当然的可以访问通过这些 `U` 为 1 的页表项进行映射的用户态内存空间。我们需要将 S 态的状态寄存器 `sstatus` 上的 `SUM (permit Supervisor User Memory access)` 位手动设置为 1 才可以做到这一点。否则 S 态通过的 `load/store` 等指令在访问`U` 为 1 的页表项映射的用合同内存空间时，CPU 会报出异常。

- `A` 表示 Accessed，如果为 1 则表示自从上次 `A` 被清零后，有虚拟地址通过这个页表项进行读写。

- `D` 表示 Dirty，如果为 1 表示自从上次 `D` 被清零后，有虚拟地址通过这个页表项进行写入。

- `RSW` 两位留给 S 态的程序来进行拓展功能实现。

`A`,`D`都是为了更好的进行页面置换算法，这也是最为常见的一种页面置换算法。

#### 页表基址

页表的基址（起始地址）一般会保存在一个特殊的寄存器中。在 RISC-V 中，这个特殊的寄存器就是页表寄存器 satp。

![img](https://cdn.jsdelivr.net/gh/xmmmmmovo/ResourcesBackup/blog/sv39_satp.jpg)

我们使用寄存器 `satp` 来控制 CPU 进行页表映射。

- `MODE` 控制 CPU 使用哪种页表实现，我们只需将 `MODE` 设置为 8 即表示 CPU 使用 Sv39 。
- `ASID` 表示地址空间标识符，这里还没有涉及到进程的概念，我们不需要管这个地方。
- `PPN` 存的是三级页表所在的物理页号。这样，给定一个虚拟页号，CPU 就可以从三级页表开始一步步的将其映射到一个物理页号。

于是，OS 可以在内存中为不同的应用分别建立不同虚实映射的页表，并通过修改寄存器 `satp` 的值指向不同的页表，从而可以修改 CPU 虚实地址映射关系及内存保护的行为。

#### 快表（TLB）

我们知道，物理内存的访问速度要比 CPU 的运行速度慢很多。如果我们按照页表机制循规蹈矩的一步步走，将一个虚拟地址转化为物理地址需要访问 3 次物理内存，得到物理地址后还需要再访问一次物理内存，才能完成访存。这无疑很大程度上降低了效率。

事实上，实践表明虚拟地址的访问具有时间局部性和空间局部性。因此，在 CPU 内部，我们使用**快表（TLB, Translation Lookaside Buffer）**来作为虚拟页号到物理页号的映射的缓存。这部分知识在计算机组成原理课程中有所体现，当我们要做一个映射时，会有很大可能这个映射在近期被完成过，所以我们可以先到 TLB 里面去查一下，如果有的话我们就可以直接完成映射，而不用访问那么多次内存了。

但如果修改了 `satp` 寄存器，说明 OS 切换到了一个与先前映射方式完全不同的页表。此时快表里面存储的映射已经失效了，这种情况下 OS 要在修改 `satp` 的指令后面马上使用 `sfence.vma` 指令刷新整个 TLB。

同样，我们手动修改一个页表项之后，也修改了映射，但 TLB 并不会自动刷新，我们也需要使用 `sfence.vma` 指令刷新 TLB。如果不加参数的，`sfence.vma` 会刷新整个 TLB。你可以在后面加上一个虚拟地址，这样 `sfence.vma` 只会刷新这个虚拟地址的映射。

#### 实现思路

整理完了所有的知识，现在来思考一个问题──在内存初始化之前，内核代码和地址是怎么分配的？

答案就是使用`Fixed map`来进行内存配置，主要是在`mm_init`初始化之前，先将内存分解为固定的内存块，在boot的时候将其直接映射到物理地址上去。

一般来说，虚拟内存都是会分为下面几部分：

![image-20210314195252100](https://cdn.jsdelivr.net/gh/xmmmmmovo/ResourcesBackup/blog/image-20210314195252100.png)

其中fixed就是fixedmap的区域，这一块最好看qemu中的源码，其中用c语言模拟了`fixed_map`(源码里搜索fixed_map)并且也根据fixedmap进行了一些引导。

![1771657-20190831230833457-192209033](https://cdn.jsdelivr.net/gh/xmmmmmovo/ResourcesBackup/blog/1771657-20190831230833457-192209033.png)

上面这是内存布局的详细信息👆🏻

















