//! interrupt handle function
use super::context::Context;
use super::timer;
use super::handler;
use crate::process::PROCESSOR;
use riscv::register::{
    scause::{Exception, Interrupt, Scause, Trap},
    sie, stvec,
};

//extern crate alloc;
//use alloc::boxed::Box;
/*
pub struct HandleFunctionTable {
    pub handle_function: [Box<dyn FnMut(&mut Context)>; 4],
}

impl HandleFunctionTable {
    pub fn init() -> Self {
        HandleFunctionTable {
            handle_function: [
                Box::new(|context: &mut Context| {
                    println!("Breakpoint at 0x{:x}", context.sepc);
                    context.sepc += 2;
                }),// breakpoint interrupt closure
                Box::new(|context: &mut Context| {
                    println!("system call at 0x{:x}", context.sepc);
                    context.sepc += 2;
                }),//system call closure
                Box::new(|context: &mut Context| {
                    timer::tick();
                }),//time interrupt closure
                Box::new(|context: &mut Context| {
                    println!("External interrupt at 0x{:x}", context.sepc);
                    context.sepc += 2;
                }),
            ]
        }
    }
}
*/
pub union Vector {
    pub handler: unsafe fn(context: &mut Context) -> *mut Context,
}
#[no_mangle]
pub static __INTERRUPTS: [Vector; 4] = [
    Vector {handler: breakpoint,},
    Vector {handler: syscall_handler,},
    Vector {handler: supervisor_timer,},
    Vector {handler: supervisor_external,}
];

pub fn get_handle_function(index: usize, context: &mut Context) {
    let handle_function_table = [
        |context: &mut Context| {
            println!("Breakpoint at 0x{:x}", context.sepc);
            context.sepc += 2;
        },// breakpoint interrupt closure
        |context: &mut Context| {
            println!("system call at 0x{:x}", context.sepc);
            context.sepc += 2;
        },//system call closure
        |context: &mut Context| {
            timer::tick();
        },//time interrupt closure
        |context: &mut Context| {
            println!("External interrupt at 0x{:x}", context.sepc);
            context.sepc += 2;
        },
    ];   
    handle_function_table[index](context);
}


/// handle ebreak interrupt
///
/// continue: sepc add 2 to continue
pub fn breakpoint(context: &mut Context) -> *mut Context{
    println!("Breakpoint at 0x{:x}", context.sepc);
    /*
    println!("Another breakpoint interrupt start");
    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
    println!("Another breakpoint interrupt end");
    */
    context.sepc += 2;
    //println!("breakpoint interrupt return");
    context
}
/// handle system call
///
/// continue: sepc add 2 to continue
pub fn syscall_handler(context: &mut Context) -> *mut Context{
    println!("system call at 0x{:x}", context.sepc);
    context.sepc += 2;
    context
}

/// handle time interrupt
///
/// count in `tick()` and call a ebreak
//pub fn supervisor_timer(_: &Context) {
pub fn supervisor_timer(context: &mut Context) -> *mut Context{
    timer::tick();
    PROCESSOR.get().park_current_thread(context);
    //println!("timer interrupt return");
    PROCESSOR.get().prepare_next_thread()
}

/// handle external interrupt
///
/// continue: sepc add 2 to continue
pub fn supervisor_external(context: &mut Context) -> *mut Context{
    println!("External interrupt at 0x{:x}", context.sepc);
    context.sepc += 2;
    context
}

/// 出现未能解决的异常，终止当前线程
pub fn fault(_context: &mut Context, scause: Scause, stval: usize) -> *mut Context {
    println!(
        "{:x?} terminated with {:x?}",
        PROCESSOR.get().current_thread(),
        scause.cause()
    );
    println!("stval: {:x}", stval);
    PROCESSOR.get().kill_current_thread();
    // 跳转到 PROCESSOR 调度的下一个线程
    PROCESSOR.get().prepare_next_thread()
}