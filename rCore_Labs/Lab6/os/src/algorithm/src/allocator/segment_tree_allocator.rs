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
    fn refresh_tree(&mut self, mut index: usize, value: bool) {
        self.tree.set_bit(index, value);
        while index > 1 {
            index /= 2;
            let truth = self.tree.get_bit(index * 2) && self.tree.get_bit(index * 2 + 1);
            self.tree.set_bit(index, truth);
        }
    }
}

impl Allocator for SegmentTreeAllocator {
    fn new(capacity: usize) -> Self {
        // num of leaf
        let leaf_num = capacity.next_power_of_two();
        let mut tree = vec![0u8; leaf_num*2];
        for i in ((capacity + 7) / 8)..(leaf_num / 8) {
            tree[leaf_num / 8 + i] = 255u8;
        }
        for i in capacity..(capacity + 8) {
            tree.set_bit(leaf_num + i, true);
        }
        for i in (1..leaf_num).rev() {
            let v = tree.get_bit(i * 2) && tree.get_bit(i * 2 + 1);
            tree.set_bit(i, v);
        }
        Self { 
            tree
        }
    }

    fn alloc(&mut self) -> Option<usize> {
        match self.tree.get_bit(1) {
            true => None,
            false => {
                let mut temp_node = 1;
                while temp_node < self.tree.len() / 2 {
                    temp_node = match !self.tree.get_bit(temp_node*2) {
                        true => temp_node*2,
                        false => {
                            match !self.tree.get_bit(temp_node*2+1) {
                                true => temp_node*2+1,
                                false => panic!("tree is full of damaged")
                            }
                        }
                    };
                }
                // change the tree
                self.refresh_tree(temp_node, true);
                Some(temp_node - self.tree.len() / 2)
            }
        }
    }

    fn dealloc(&mut self, index: usize) {
        let change_node = index + self.tree.len() / 2;
        self.refresh_tree(change_node, false);
    }
}

