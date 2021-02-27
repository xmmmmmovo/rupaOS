use crate::memory::PAGE_SIZE;

use super::PageTableEntry;

/// 存有 512 个页表项的页表
///
/// 注意我们不会使用常规的 Rust 语法来创建 `PageTable`。相反，我们会分配一个物理页，
/// 其对应了一段物理内存，然后直接把其当做页表进行读写。我们会在操作系统中用一个「指针」
/// [`PageTableTracker`] 来记录这个页表。
#[repr(C)]
pub struct PageTable {
    pub entries: [PageTableEntry; PAGE_SIZE / 8],
}
