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
mod drivers;
mod fs;
mod kernel;

#[allow(unused_imports)]
use crate::memory::PhysicalAddress;
use process::*;
use xmas_elf::ElfFile;
use fs::*;

extern crate alloc;

//entry
global_asm!(include_str!("asm/entry.asm"));

// the first function to be called after _start
#[no_mangle]
pub extern "C" fn rust_main(_hart_id: usize, dtb_pa: PhysicalAddress ) -> ! {
    println!("Hello, rCore-Tutorial!");
    println!("I have done Lab 6");
    //panic!("Hi,panic here...")
    
    memory::init();
    interrupt::init();
    drivers::init(dtb_pa);
    fs::init();
    /*
    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
    */
    //unreachable!();
    //loop{};
    
    
    
    // test for alloc space
    /*
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);
    {
        let mut vec = Vec::new();
        for i in 0..10 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 10);
        for (i, value) in vec.into_iter().enumerate() {
            assert_eq!(value, i);
        }
        println!("head test passed");
    }
    */
    // test
    //println!("{}", *memory::config::KERNEL_END_ADDRESS);
    // test
    /*
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
    */
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
    for message in 0..10 {
        let thread = Thread::new(
            process.clone(),
        sample_process as usize,
        Some(&[message]),
        message,
        ).unwrap();
        PROCESSOR.get().add_thread(thread);
    }
    drop(process);
    PROCESSOR.get().run();
    */

    // test
    /*
    let process = Process::new_kernel().unwrap();

    PROCESSOR
        .get()
        .add_thread(Thread::new(process.clone(), simple as usize, Some(&[0]), 1).unwrap());

    // 把多余的 process 引用丢弃掉
    drop(process);

    PROCESSOR.get().run()
    */
    
    start_kernel_thread();
    start_kernel_thread();
    start_user_thread("hello_world");
    start_user_thread("notebook");
    PROCESSOR.get().run()
    
}


/*
fn sample_process(message: usize) {
    for i in 0..1000000 {
        if i % 200000 == 0 {
            println!("thread {}", message);
        }
    }
}*/
/*
/// 测试任何内核线程都可以操作文件系统和驱动
fn simple(id: usize) {
    println!("hello from thread id {}", id);
    // 新建一个目录
    fs::ROOT_INODE
        .create("tmp", rcore_fs::vfs::FileType::Dir, 0o666)
        .expect("failed to mkdir /tmp");
    // 输出根文件目录内容
    fs::ls("/");

    loop {}
}
*/

fn start_kernel_thread() {
    let process = Process::new_kernel().unwrap();
    let thread = Thread::new(process, test as usize, None, 0).unwrap();
    PROCESSOR.get().add_thread(thread);
}

fn test() {
    println!("hello");
}

fn start_user_thread(name: &str) {
    // 从文件系统中找到程序
    let app = fs::ROOT_INODE.find(name).unwrap();
    // 读取数据
    let data = app.readall().unwrap();
    // 解析 ELF 文件
    let elf = ElfFile::new(data.as_slice()).unwrap();
    // 利用 ELF 文件创建线程，映射空间并加载数据
    let process = Process::from_elf(&elf, true).unwrap();
    // 再从 ELF 中读出程序入口地址
    let thread = Thread::new(process, elf.header.pt2.entry_point() as usize, None, 0).unwrap();
    // 添加线程
    PROCESSOR.get().add_thread(thread);
}

