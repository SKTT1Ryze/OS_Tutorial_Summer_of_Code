//! 负责分配 / 回收的数据结构

mod segment_tree_allocator;
mod stacked_allocator;
mod free_list_allocator;
mod bitmap_vector_allocator;
mod buddy_system_vector_allocator;
/// 分配器：固定容量，每次分配 / 回收一个元素
pub trait Allocator {
    /// create allocator with capacity
    fn new(capacity: usize) -> Self;
    /// alloc a item. error return `None`
    fn alloc(&mut self) -> Option<usize>;
    /// dealloc a item
    fn dealloc(&mut self, index: usize);
}

pub use segment_tree_allocator::SegmentTreeAllocator;
pub use stacked_allocator::StackedAllocator;
pub use free_list_allocator::FreeListAllocator;

/// default Allocator
// pub type AllocatorImpl = FreeListAllocator;
// pub type AllocatorImpl = SegmentTreeAllocator;
pub type AllocatorImpl = StackedAllocator;

/// 分配器：固定容量，每次分配 / 回收指定大小的元素
pub trait VectorAllocator {
    /// create allocator with capacity
    fn new(capacity: usize) -> Self;
    /// alloc space with size of `size`. error return `None`
    fn alloc(&mut self, size: usize, align: usize) -> Option<usize>;
    /// dealloc space with `start` and `size`
    fn dealloc(&mut self, start: usize, size: usize, align: usize);
}

pub use bitmap_vector_allocator::BitmapVectorAllocator;
pub use buddy_system_vector_allocator::BuddySystemVectorAllocator;
/// default VectorAllocator
//pub type VectorAllocatorImpl = BitmapVectorAllocator;
pub type VectorAllocatorImpl = BuddySystemVectorAllocator;