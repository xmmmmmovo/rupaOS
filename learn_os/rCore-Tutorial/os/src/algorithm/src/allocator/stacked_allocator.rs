use super::Allocator;

use alloc::{vec, vec::Vec};

/// 使用栈结构实现分配器
///
/// 在 `Vec` 末尾进行加入 / 删除。
/// 每个元素 tuple `(start, end)` 表示 [start, end) 区间为可用。
pub struct StackedAllocator {
    list: Vec<(usize, usize)>,
}

impl Allocator for StackedAllocator {
    fn new(capacity: usize) -> Self {
        Self {
            list: vec![(0, capacity)],
        }
    }

    fn alloc(&mut self) -> Option<usize> {
        if let Some((start, end)) = self.list.pop() {
            // 这里start+1说明空闲空间-1，分配一个
            if end - start > 1 {
                self.list.push((start + 1, end));
            }
            Some(start)
        } else {
            None
        }
    }

    fn dealloc(&mut self, index: usize) {
        // 尾部扩容
        // 这样是存在内存碎片但是不会出现内存重叠问题
        self.list.push((index, index + 1));
    }
}
