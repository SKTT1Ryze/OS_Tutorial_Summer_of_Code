//! 负责分配 / 回收的数据结构

mod segment_tree_allocator;
mod stacked_allocator;
mod free_list_allocator;
mod bitmap_vector_allocator;
/// 分配器：固定容量，每次分配 / 回收一个元素
pub trait Allocator {
    /// 给定容量，创建分配器
    fn new(capacity: usize) -> Self;
    /// 分配一个元素，无法分配则返回 `None`
    fn alloc(&mut self) -> Option<usize>;
    /// 回收一个元素
    fn dealloc(&mut self, index: usize);
}

pub use segment_tree_allocator::SegmentTreeAllocator;
pub use stacked_allocator::StackedAllocator;
pub use free_list_allocator::FreeListAllocator;

/// default Allocator
// pub type AllocatorImpl = FreeListAllocator;
// pub type AllocatorImpl = SegmentTreeAllocator;
pub type AllocatorImpl = StackedAllocator;

pub trait VectorAllocator {
    /// 给定容量，创建分配器
    fn new(capacity: usize) -> Self;
    /// 分配指定长度的空间，无法分配则返回 `None`
    fn alloc(&mut self, size: usize, align: usize) -> Option<usize>;
    /// 回收指定空间（一定是之前分配的）
    fn dealloc(&mut self, start: usize, size: usize, align: usize);
}

pub use bitmap_vector_allocator::BitmapVectorAllocator;

/// default VectorAllocator
pub type VectorAllocatorImpl = BitmapVectorAllocator;