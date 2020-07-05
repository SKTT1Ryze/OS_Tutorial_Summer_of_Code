/*
 * rCore Labs: Lab 0
 * 2020/7/5
 * hustccc
 * Manjaro
 */
//! # global
#![no_std]
#![no_main]
//#![warn(missing_docs)]
//insert assemble file
#![feature(asm)]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod panic;
mod sbi;

//entry
global_asm!(include_str!("asm/entry.asm"));

/*
use core::panic::PanicInfo;
//use inserted assemble for print a char
pub fn console_putchar(ch: u8) {
    let _ret: usize;
    let arg0: usize = ch as usize;
    let arg1: usize = 0;
    let arg2: usize = 0;
    let which: usize = 1;
    unsafe {
        llvm_asm!("ecall"
        //asm!("ecall"
             : "={x10}" (_ret)
             : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
             : "memory"
             : "volatile"
        );
    }
}
//entry for Rust
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    console_putchar(b'R');
    console_putchar(b'u');
    console_putchar(b's');
    console_putchar(b't');
    console_putchar(b'_');
    console_putchar(b'O');
    console_putchar(b'S');
    console_putchar(b'\n');
    loop {}
}
*/
// the first function to be called after _start
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Hello, rCore-Tutorial!");
    println!("I have done Lab 0");
    panic!("Hi,panic here...")
}




