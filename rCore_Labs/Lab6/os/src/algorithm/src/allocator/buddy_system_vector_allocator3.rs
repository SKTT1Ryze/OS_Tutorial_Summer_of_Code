//! [`BuddySystemVectorAllocator`]

use super::VectorAllocator;
//use alloc::collections::LinkedList;
//use alloc::{vec,vec::Vec};
use core::cmp::min;
const MAX_LIST_SIZE: usize = 10;

const MAX_BLOCK_LIST_SIZE: usize = 512;
/// block of buddy system
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Block {
    start: u8,
    size: u8,
    is_exist: bool,
}
/// Buddy System Vector Allocator
pub struct BuddySystemVectorAllocator {
    capacity: usize,
    list_size: usize,
    list: [[Block; MAX_BLOCK_LIST_SIZE]; MAX_LIST_SIZE],
}

impl Block {
    fn new () -> Self{
        Self {
            start: 0,
            size: 0,
            is_exist: false,
        }
    }
}

impl BuddySystemVectorAllocator {    
    fn is_empty_at (&self, index: usize) -> bool {
        for block in self.list[index].iter() {
            match block.is_exist {
                false => {},
                true => {
                    return false;
                }
            }
        }
        true
    }
    fn pop_back_at (&mut self, index: usize) -> Option<Block> {
        let mut i = 0;
        for block in self.list[index].iter() {
            match block.is_exist {
                false => { i += 1;},
                true => {
                    //let new_block = block.take();
                    let new_block = Block {
                        start: block.start,
                        size: block.size,
                        is_exist: true,
                    };
                    self.list[index][i].is_exist = false;
                    return Some(new_block);
                }
            }
        }
        None
    }

    fn push_back_at (&mut self, index: usize, new_block: Block) {
        for i in 0..MAX_BLOCK_LIST_SIZE {
            match self.list[index][i].is_exist {
                false => {
                    self.list[index][i].start = new_block.start;
                    self.list[index][i].size = new_block.size;
                    self.list[index][i].is_exist = true;
                    return;
                },
                true => {},
            }
        }
    }
    fn update (&mut self, new_block: Block, index: usize) {
        if self.is_empty_at(index) {
            self.push_back_at(index, new_block);
        }
        else if index == self.list_size - 1 {
            return;
        }
        else {
            let mut temp_array = [Block::new(); MAX_BLOCK_LIST_SIZE];
            let mut count = 0;
            while let Some(temp_block) = self.pop_back_at(index) {
                if ((temp_block.start - new_block.start) as isize).abs() as u8 == new_block.size {
                    if temp_block.start % (new_block.size*2) == 0 {
                        let new_big_block = Block {
                            start: temp_block.start,
                            size: temp_block.size*2,
                            is_exist: true,
                        };
                        self.update(new_big_block, index+1);
                    }
                    else if new_block.start % (new_block.size*2) == 0 {
                        let new_big_block = Block {
                            start: new_block.start,
                            size: new_block.size*2,
                            is_exist: true,
                        };
                        self.update(new_big_block, index+1);
                    }
                }
                //temp_array[count] = temp_block;
                temp_array[count].start = temp_block.start;
                temp_array[count].size = temp_block.size;
                temp_array[count].is_exist = true;
                count += 1;
            }
            /*
            while let Some(back_block) = temp_vec.pop() {
                self.list[index].push_back(back_block);
            }*/
            for i in 0..count {
                self.push_back_at(index, temp_array[i]);
            }
            self.push_back_at(index, new_block);
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

}


impl VectorAllocator for BuddySystemVectorAllocator {
    fn new(capacity: usize) -> Self {
        let total_size = capacity.next_power_of_two();
        let list_size = min(Self::log_two(total_size) + 1, MAX_LIST_SIZE);
        let mut new_list = [[
            Block {
                start: 0,
                size: 0,
                is_exist: false,
            };
            MAX_BLOCK_LIST_SIZE]; 
            MAX_LIST_SIZE];
        
        new_list[list_size-1][0].start = 0;
        new_list[list_size-1][0].size = total_size as u8;
        new_list[list_size-1][0].is_exist = true;
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
        let get_index = Self::log_two(size.next_power_of_two());
        //let get_index = log_two(size);
        let mut find_index =get_index;
        while self.is_empty_at(find_index)  {
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
                let temp_block = self.pop_back_at(find_index - i);
                match temp_block {
                    None => panic!("the linkedlist is empty...from buddy system"),
                    Some(tblock) => {
                        self.push_back_at(
                            find_index - i - 1,
                            Block {
                                start: tblock.start,
                                size: tblock.size / 2,
                                is_exist: true,
                            }
                        );
                        self.push_back_at(
                            find_index - i - 1,
                            Block {
                                start: tblock.start + tblock.size / 2,
                                size: tblock.size / 2,
                                is_exist: true,
                            }
                        );
                    }
                }
            }
            let alloc_block = self.pop_back_at(get_index);
            match alloc_block {
                None => panic!("get block error...from buddy system"),
                Some(get_block) => {
                    //assert_eq!(get_block.size, size.next_power_of_two());
                    return Some(get_block.start as usize);
                }
            }
        }
    }

    fn dealloc(&mut self, start: usize, size: usize, _align: usize) {
        if !size.is_power_of_two() {
            panic!("dealloc size is not the power fo two");
        }
        let get_index = Self::log_two(size);
        let new_block = Block {
            start: start as u8,
            size: size as u8,
            is_exist: true,
        };
        self.update(new_block, get_index);
    }
}

