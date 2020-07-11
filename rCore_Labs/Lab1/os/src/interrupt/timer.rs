//! set next time interrupt and handle time interrupt
#![feature(asm)]
#![feature(llvm_asm)]
#![feature(global_asm)]
use crate::sbi::set_timer;
use riscv::register::{sie, sstatus, time};

/// interval of time interrupt, per CPU instruction
static INTERVAL: usize = 100000;
/// count for time interrupt
pub static mut TICKS: usize = 0;

/// initialize time interrupt
///
/// enable time interrupt, and set the first time interrupt
pub fn init() {
    unsafe {
        // set STIE, enable the time interrupt
        sie::set_stimer();
        // set sstatus.SIE
        sstatus::set_sie();
    }
    // set next time interrupt
    set_next_timeout();
}

/// set next time interrupt
///
/// get current time, plus interval, and call SBI to set next time interrupt
fn set_next_timeout() {
    set_timer(time::read() + INTERVAL);
}

/// call this function when time interrupt occurs
///
/// set next time interrupt, and count +1
pub fn tick() {
    set_next_timeout();
    unsafe {
        TICKS += 1;
        if TICKS % 100 == 0 {
            println!("{} tick", TICKS);
            unsafe {
                llvm_asm!("ebreak"::::"volatile");
            };
        }
    }
}
