//! [`StrideScheduler`]
//pub const MAX_STRIDE: usize = 2usize.pow(32) - 1;
pub const MAX_STRIDE: usize = 4_294_967_295;
use super::Scheduler;
//use alloc::collections::LinkedList;
use alloc::vec::Vec;

pub struct ThreadBlock<ThreadType: Clone + Eq> {
    thread: ThreadType,
    pub priority: usize,
    pub stride: usize,
}

impl<ThreadType: Clone + Eq> ThreadBlock<ThreadType> {
    fn new(thread: ThreadType, priority: usize, stride: usize) -> Self {
        Self {
            thread: thread,
            priority: priority,
            stride: stride,
        }
    }
    fn update_stride(&mut self) {
        if self.priority == 0 {
            self.stride = MAX_STRIDE;
        } else {
            self.stride += MAX_STRIDE / self.priority;
        }
    }
    fn set_priority(&mut self, priority: usize) {
        self.priority = priority;
    }
}

/// thread scheduler base on stride scheduling
pub struct StrideScheduler<ThreadType: Clone + Eq> {
    pool: Vec<ThreadBlock<ThreadType>>,
}

/// `Default` create a empty scheduler
impl<ThreadType: Clone + Eq> Default for StrideScheduler<ThreadType> {
    fn default() -> Self {
        Self { pool: Vec::new() }
    }
}

impl<ThreadType: Clone + Eq> StrideScheduler<ThreadType> {
    fn get_min_stride_thread_index(&mut self) -> Option<usize> {
        if self.pool.is_empty() {
            return None;
        }
        let mut min_stride_thread_index = 0;
        for i in 0..self.pool.len() {
            if self.pool[i].stride < self.pool[min_stride_thread_index].stride {
                min_stride_thread_index = i;
            }
        }
        Some(min_stride_thread_index)
    }
}

impl<ThreadType: Clone + Eq> Scheduler<ThreadType> for StrideScheduler<ThreadType> {
    fn add_thread(&mut self, thread: ThreadType, priority: usize) {
        self.pool.push(ThreadBlock::new(thread, priority, 0))
    }

    fn get_next(&mut self) -> Option<ThreadType> {
        if let Some(index) = self.get_min_stride_thread_index() {
            let mut threadblock = self.pool.remove(index);
            threadblock.update_stride();
            let next_thread = threadblock.thread.clone();
            self.pool.push(threadblock);
            Some(next_thread)
        } else {
            None
        }
    }

    fn remove_thread(&mut self, thread: &ThreadType) {
        let mut removed = self.pool.drain_filter(|t| &(t.thread) == thread);
        assert!(removed.next().is_some() && removed.next().is_none());
    }

    fn set_priority(&mut self, thread: ThreadType, priority: usize) {
        for threadblock in self.pool.iter_mut() {
            if threadblock.thread == thread {
                threadblock.set_priority(priority);
            }
        }
    }
}
