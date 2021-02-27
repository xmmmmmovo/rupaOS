use crate::VectorAllocator;

/// 伙伴系统分配器的简单实现，每字节用一位表示
pub struct BuddySystemAllocator {}

impl VectorAllocator for BuddySystemAllocator {
    fn new(capacity: usize) -> Self {
        todo!()
    }

    fn alloc(&mut self, size: usize, align: usize) -> Option<usize> {
        todo!()
    }

    fn dealloc(&mut self, start: usize, size: usize, align: usize) {
        todo!()
    }
}
