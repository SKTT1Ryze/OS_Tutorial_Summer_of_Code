/*
 * Learn Rust The Hard Way: LeetCode 01
 * 2020/7/6
 * hustccc
 * Manjaro
 */
//https://leetcode-cn.com/problems/insert-delete-getrandom-o1/


extern crate rand;
use std::collections::HashMap;
use std::collections::HashSet;
use rand::Rng;
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unreachable_patterns)]
struct  set {
    map: HashMap<i32, usize>,
    vector: Vec<i32>,
}

impl set {
    fn new() -> set {
        set {
            map: HashMap::new(),
            vector: Vec::new(),
        }
    }

    pub fn insert(&mut self, value: i32) -> bool {
        if self.map.contains_key(&value) {
            println!("value has existed");
            return false;
        }
        let len = self.map.len();
        match (self.vector.len() > len) {
            true => {
                self.vector[len] = value;
            },
            false => {
                self.vector.push(value);
            }
        }
        self.map.insert(value, len);
        return true;
    }

    pub fn remove(&mut self, value: i32) -> bool {
        match self.map.contains_key(&value) {
            false => {
                return false;
            },
            true => {},
        }
        let len = self.map.len();
        let index = *self.map.get(&value).expect("Ok");
        match index+1 {
            len => {},
            _ => {
                self.vector[index] = self.vector[self.map.len() - 1];
                self.map.insert(self.vector[index], index);
            },    
        }
        self.map.remove(&value);
        return true;
    }
    pub fn get_random(&self) -> i32 {
        let mut seed = rand::thread_rng();
        let mut temp: usize = seed.gen();
        temp = temp % self.map.len();
        return self.vector[temp];
    }
}

