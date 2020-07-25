//! [`BuddySystemVectorAllocator`]

use super::VectorAllocator;
use alloc::collections::LinkedList;
use alloc::{vec,vec::Vec};
//use core::cmp::min;
const MAX_LIST_SIZE: usize = 12;
const MAX_BLOCK_LIST_SIZE: usize = 4096;
/// block of buddy system
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Block {
    start: usize,
    size: usize,
}
/// Buddy System Vector Allocator
pub struct BuddySystemVectorAllocator {
    capacity: usize,
    list_size: usize,
    list: Vec<Vec<Block>>,
}


impl BuddySystemVectorAllocator {
    fn update (&mut self, new_block: Block, index: usize) {
        if self.list[index].is_empty() {
            self.list[index].push(new_block);
        }
        else {
            let mut temp_vec = Vec::new();
            while let Some(temp_block) = self.list[index].pop() {
                if ((temp_block.start - new_block.start) as isize).abs() as usize  == new_block.size {
                    if temp_block.start % (new_block.size*2) == 0 {
                        let new_big_block = Block {
                            start: temp_block.start,
                            size: temp_block.size*2,
                        };
                        self.update(new_big_block, index+1);
                    }
                    else if new_block.start % (new_block.size*2) == 0 {
                        let new_big_block = Block {
                            start: new_block.start,
                            size: new_block.size*2,
                        };
                        self.update(new_big_block, index+1);
                    }
                }
                temp_vec.push(temp_block);
            }
            while let Some(back_block) = temp_vec.pop() {
                self.list[index].push(back_block);
            }
            self.list[index].push(new_block);
        }
    }
}


impl VectorAllocator for BuddySystemVectorAllocator {
    fn new(capacity: usize) -> Self {
        let total_size = capacity.next_power_of_two()/2;
        let list_size = log_two(total_size) + 1;
        let mut new_list = vec![Vec::new(); list_size];
        let first_block = Block {
            start: 0,
            size: total_size,
        };
        new_list[list_size-1].push(first_block);
        Self {
            capacity: total_size,
            list_size: list_size,
            list: new_list,
        }
    }

    fn alloc(&mut self, size: usize, _align: usize) -> Option<usize> {
        if size > self.capacity {
            return None;
        }
        let get_index = log_two(size.next_power_of_two());
        let mut find_index =get_index;
        while self.list[find_index].is_empty()  {
            find_index += 1;
            if find_index > self.list_size -1 {
                break;
            }
        }
        if find_index > self.list_size - 1 {
            panic!("BuddySystemAllocator has nothing");
        }
        else {
            for i in 0.. (find_index- get_index) {
                let temp_block = self.list[find_index - i].pop();
                match temp_block {
                    None => panic!("the linkedlist is empty...from buddy system"),
                    Some(tblock) => {
                        self.list[find_index - i - 1].push(
                            Block {
                                start: tblock.start,
                                size: tblock.size / 2,
                            }
                        );
                        self.list[find_index - i - 1].push(
                            Block {
                                start: tblock.start + tblock.size / 2,
                                size: tblock.size / 2,
                            }
                        );
                    }
                }
            }
            let alloc_block = self.list[get_index].pop();
            match alloc_block {
                None => panic!("get block error...from buddy system"),
                Some(get_block) => {
                    //assert_eq!(get_block.size, size.next_power_of_two());
                    return Some(get_block.start);
                }
            }
        }
    }

    fn dealloc(&mut self, start: usize, size: usize, _align: usize) {
        if !size.is_power_of_two() {
            panic!("dealloc size is not the power fo two");
        }
        let get_index = log_two(size);
        let new_block = Block {
            start: start,
            size: size,
        };
        self.update(new_block, get_index);
    }
}

pub fn log_two(num: usize) -> usize {
    if num % 2 != 0 {
        panic!("not the power of two");
    }
    else {
        for i in 0..num {
            if 2usize.pow(i as u32) == num {
                return i;
            }
        }
        panic!("temp to get log2 of 0");
    }
}