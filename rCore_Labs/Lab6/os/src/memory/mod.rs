//! memory arrange mod
//!
//! used for alloc space and Virtual address mapping
#![allow(dead_code)]
pub mod heap;
pub mod config;
pub mod address;
pub mod frame;
pub mod range;
pub mod mapping;
pub use {
    address::*,
    config::*,
    frame::FRAME_ALLOCATOR,
    range::Range,
    mapping::{Flags, MapType, MemorySet, Segment},
};

/// short name for some functions in mods
pub type MemoryResult<T> = Result<T, &'static str>;

/// initialize son mod of memory
///
/// - [`heap::init`]
pub fn init() {
    heap::init();
    // 允许内核读写用户态内存
    unsafe { riscv::register::sstatus::set_sum() };

    println!("mod memory initialized");
}