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
#![feature(slice_fill)]
#![feature(naked_functions)]

#[macro_use]
mod console;
mod panic;
mod sbi;
mod interrupt;
mod memory;
mod process;
#[allow(unused_imports)]
use crate::memory::PhysicalAddress;
use process::*;
//use xmas_elf::ElfFile;

extern crate alloc;

//entry
global_asm!(include_str!("asm/entry.asm"));

// the first function to be called after _start
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Hello, rCore-Tutorial!");
    println!("I have done Lab 4");
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
    
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);
    let mut vec = Vec::new();
    for i in 0..100 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 100);
    for (i, value) in vec.into_iter().enumerate() {
        assert_eq!(value, i);
    }
    println!("head test passed");
    
    // test
    //println!("{}", *memory::config::KERNEL_END_ADDRESS);
    // test
    
    for index in 0..2 {
        let frame_0 = match memory::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}",err)
        };
        let frame_1 = match memory::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}",err)
        };
        println!("index: {}, {} and {}", index, frame_0.page_number(), frame_1.page_number());
        //println!("index: {}, {} and {}", index, frame_0.address(), frame_1.address());
    }
    panic!("end of rust_main");
    // test
    /*
    let remap = memory::mapping::MemorySet::new_kernel().unwrap();
    remap.activate();
    println!("kernel has remapped");
    panic!()
    */

    // test 
    /*
    let process = Process::new_kernel().unwrap();
    for message in 0..8 {
        let thread = Thread::new(
            process.clone(),
        sample_process as usize,
        Some(&[message]),
        ).unwrap();
        PROCESSOR.get().add_thread(thread);
    }
    drop(process);
    PROCESSOR.get().run();
    */
}

fn sample_process(message: usize) {
    for i in 0..1000000 {
        if i % 200000 == 0 {
            println!("thread {}", message);
        }
    }
}



