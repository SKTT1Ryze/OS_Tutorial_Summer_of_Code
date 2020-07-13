/*
 * rCore Lab 2: Memory
 * hustccc
 * 2020/7/13
 * Manjaro
 */

//! memory arrange mod
//!
//! used for alloc space and Virtual address mapping
#![allow(dead_code)]
pub mod heap;
pub mod config;
pub mod address;

/// initialize son mod of memory
///
/// - [`heap::init`]
pub fn init() {
    heap::init();
    // 允许内核读写用户态内存
    unsafe { riscv::register::sstatus::set_sum() };

    println!("mod memory initialized");
}