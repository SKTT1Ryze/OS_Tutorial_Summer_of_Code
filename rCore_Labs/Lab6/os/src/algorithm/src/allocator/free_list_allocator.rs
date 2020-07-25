//! A frame allocator based on FreeList [`FreeListAllocator`]

use super::Allocator;
use alloc::{vec, vec::Vec};

/// implement frame allocator with FreeList
/// 
/// 使用两个 `Vec` ，一个表示空闲，一个表示已被分配
pub struct FreeListAllocator {
    used_list: Vec<(usize, usize)>,
    free_list: Vec<(usize, usize)>,
}

impl Allocator for FreeListAllocator {
    fn new(capacity: usize) -> Self {
        Self {
            used_list: vec![],
            free_list: vec![(0,capacity)],
        }
    }

    fn alloc(&mut self) -> Option<usize> {
        if let Some((start, end)) = self.free_list.pop() {
            if end - start > 1 {
                self.free_list.push((start + 1, end));
            }
            self.used_list.push((start,start+1));
            Some(start)
        }
        else {
            None
        }
    }

    fn dealloc(&mut self, index: usize) {
        for i in 0..self.used_list.len() {
            let (start, end) = self.used_list[i];
            if start == index {
                self.used_list.remove(i);
                self.free_list.push((start, end));
            }
        }
    }
}