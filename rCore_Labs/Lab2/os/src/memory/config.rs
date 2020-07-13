/*
 * rCore Lab 2: Memory
 * hustccc
 * 2020/7/13
 * Manjaro
 */

//! 定义一些内存相关的常量

use super::address::*;
use lazy_static::*;

/// 页 / 帧大小，必须是 2^n
pub const PAGE_SIZE: usize = 4096;

/// MMIO 设备段内存区域起始地址
pub const DEVICE_START_ADDRESS: PhysicalAddress = PhysicalAddress(0x1000_0000);

/// MMIO 设备段内存区域结束地址
pub const DEVICE_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x1001_0000);

/// 可以访问的内存区域起始地址
pub const MEMORY_START_ADDRESS: PhysicalAddress = PhysicalAddress(0x8000_0000);

/// 可以访问的内存区域结束地址
pub const MEMORY_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x8800_0000);

/// 内核使用线性映射的偏移量
pub const KERNEL_MAP_OFFSET: usize = 0xffff_ffff_0000_0000;

/// The size of heap used for OS allocs memory
pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;

lazy_static! {
    /// The address of end of kernel code, and the address of begin of space used to alloc
    pub static ref KERNEL_END_ADDRESS: PhysicalAddress = PhysicalAddress(kernel_end as usize);
}

extern "C" {
    /// The end of kernel code assigned by `linker.ld`
    /// 
    /// exist as var [`KERNEL_END_ADDRESS`]
    fn kernel_end();
}