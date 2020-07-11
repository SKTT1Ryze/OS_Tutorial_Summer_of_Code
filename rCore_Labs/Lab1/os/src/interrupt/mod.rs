//! interrupt mod
//! 
//! 
#![allow(unused)]
mod handler;
mod context;
mod timer;

use riscv::register::{sie, sstatus};

pub use context::Context;

/// initialize son mod of interrupt
/// 
/// - [`handler::init`]
/// - [`timer::init`]
pub fn init() {
    handler::init();
    timer::init();
    println!("mod interrupt initialized");
}

/// 等待一个外部中断
///
/// 暂时开启中断（不包括时钟中断）并执行 `wfi` 指令
///
/// 会在所有线程都在等待外部信号时调用
pub fn wait_for_interrupt() {
    unsafe {
        sie::clear_stimer();
        sstatus::set_sie();
        llvm_asm!("wfi" :::: "volatile");
        sstatus::clear_sie();
        sie::set_stimer();
    }
}