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
#![feature(asm)]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#[macro_use]
mod console;
mod panic;
mod sbi;
mod interrupt;
mod memory;

extern crate alloc;

//entry
global_asm!(include_str!("asm/entry.asm"));

// the first function to be called after _start
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Hello, rCore-Tutorial!");
    println!("I have done Lab 2");
    //panic!("Hi,panic here...")
    interrupt::init();
    /*
    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
    */
    //unreachable!();
    //loop{};
    
    memory::init();

    // test for alloc space
    /*
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);
    let mut vec = Vec::new();
    for i in 0..10000 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10000);
    for (i, value) in vec.into_iter().enumerate() {
        assert_eq!(value, i);
    }
    println!("head test passed");
    panic!()
    */
    // test
    /*
    println!("{}", *memory::config::KERNEL_END_ADDRESS);
    panic!()
    */
    // test
    
    for _ in 0..2 {
        let frame_0 = match memory::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}",err)
        };
        let frame_1 = match memory::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}",err)
        };
        println!("{} and {}", frame_0.page_number(), frame_1.page_number());
        println!("{} and {}", frame_0.address(), frame_1.address());
    }    
    panic!()
    
}




