//! [`SegmentTreeAllocator`]

use super::Allocator;
use alloc::{vec, vec::Vec};
use bit_field::BitArray;

/// Segment Tree Allocator
pub struct SegmentTreeAllocator {
    /// tree
    tree: Vec<u8>,
}

impl SegmentTreeAllocator {
    fn refresh_tree(&mut self, mut index: usize, truth: bool) {
        self.tree.set_bit(index, truth);
        while index > 1 {
            index /= 2;
            match self.tree.get_bit(index * 2) && self.tree.get_bit(index * 2 + 1) {
                true => {
                    self.tree.set_bit(index, true);
                }
                false => {
                    self.tree.set_bit(index, false);
                }
            }
        }
    }
}

impl Allocator for SegmentTreeAllocator {
    fn new(capacity: usize) -> Self {
        // num of leaf
        let leaf_num = capacity.next_power_of_two();
        let mut tree = vec![0u8; leaf_num * 2];
        for i in ((capacity + 7) / 8)..(leaf_num / 8) {
            tree[leaf_num / 8 + i] = 255u8;
        }
        for bit_offset in capacity..(capacity + 8) {
            tree.set_bit(leaf_num + bit_offset, true);
        }
        for bit in (1..leaf_num).rev() {
            match tree.get_bit(bit * 2) && tree.get_bit(bit * 2 + 1) {
                true => {
                    tree.set_bit(bit, true);
                }
                false => {
                    tree.set_bit(bit, false);
                }
            }
        }
        Self { tree }
    }

    fn alloc(&mut self) -> Option<usize> {
        match self.tree.get_bit(1) {
            true => None,
            false => {
                let mut temp_node = 1;
                while temp_node < self.tree.len() / 2 {
                    temp_node = match !self.tree.get_bit(temp_node * 2) {
                        true => temp_node * 2,
                        false => match !self.tree.get_bit(temp_node * 2 + 1) {
                            true => temp_node * 2 + 1,
                            false => panic!("tree is full of damaged"),
                        },
                    };
                }
                // change the tree
                self.refresh_tree(temp_node, true);
                Some(temp_node - self.tree.len() / 2)
            }
        }
    }

    fn dealloc(&mut self, index: usize) {
        let change_node = self.tree.len() / 2 + index;
        self.refresh_tree(change_node, false);
    }
}
