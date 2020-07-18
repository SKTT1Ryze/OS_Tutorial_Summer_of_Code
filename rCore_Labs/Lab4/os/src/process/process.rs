//! process [`Process`]
#![allow(unused_imports)]
use super::*;
use xmas_elf::ElfFile;

/// PCB
pub struct Process {
    /// is user mode or not
    pub is_user: bool,
    /// page table, memory mapping
    pub memory_set: MemorySet,
}

#[allow(unused)]
impl Process {
    /// create a kernel process
    pub fn new_kernel() -> MemoryResult<Arc<RwLock<Self>>> {
        Ok(Arc::new(RwLock::new(Self {
            is_user: false,
            memory_set: MemorySet::new_kernel()?,
        })))
    }
    /// 分配一定数量的连续虚拟空间
    ///
    /// 从 `memory_set` 中找到一段给定长度的未占用虚拟地址空间，分配物理页面并建立映射。返回对应的页面区间。
    ///
    /// `flags` 只需包括 rwx 权限，user 位会根据进程而定。
    pub fn alloc_page_range(
        &mut self,
        size: usize,
        flags: Flags,
    ) -> MemoryResult<Range<VirtualAddress>> {
        // memory_set 只能按页分配，所以让 size 向上取整页
        let alloc_size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        // 给 memory_set 添加一段不会发生重叠的空间
        let mut range = Range::<VirtualAddress>::from(0x1000000..0x1000000 + alloc_size);
        while self.memory_set.overlap_with(range.into()) {
            range.start += alloc_size;
            range.end += alloc_size;
        }
        // alloc physical page and build mapping
        self.memory_set.add_segment(
            Segment {
                map_type: MapType::Framed,
                range,
                flags: flags | Flags::user(self.is_user),
            },
            None,
        )?;
        Ok(Range::from(range.start..(range.start + size)))
    }
}

