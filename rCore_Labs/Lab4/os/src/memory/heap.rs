use super::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;
/// Heap space for alloc memory
/// 
/// Size: [`KERNEL_HEAP_SIZE`]
/// This space will be in .bss segment
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0;KERNEL_HEAP_SIZE];

/// Heap allocator
/// 
/// ### `#[global_allocator]`
/// [`LockedHeap`] implements [`alloc::alloc::GlobalAlloc`] trait,
/// Can alloc space when heap is needed. such as: `Box`, `Arc`, etc.
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

/// Initialize OS heap space when running
pub fn init() {
    //use `HEAP_SPACE` as heap
    unsafe {
        HEAP.lock().init(
            HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE
        )
    }
}

/// Alloc space error, panic
#[alloc_error_handler]
fn alloc_error_handler(_: alloc::alloc::Layout) -> ! {
    panic!("Alloc error")
}