#![allow(unused)]
#![feature(asm)]
#![feature(llvm_asm)]
use super::context::Context;
use super::timer;
use super::idt;
use super::handle_function;
use super::handle_function::__INTERRUPTS;
use crate::kernel::syscall_handler;
use crate::memory::*;
use riscv::register::{
    scause::{Exception, Interrupt, Scause, Trap},
    sie, stvec,
};
use crate::process::PROCESSOR;

global_asm!(include_str!("./interrupt.asm"));

/// initialize interrupt handler
///
/// set `__interrupt` to `stvec`, and enable interrupt
pub fn init() {
    unsafe {
        extern "C" {
            /// interrupt entry point from `interrupt.asm`
            fn __interrupt();
        }
        // use Direct mode ，and set interrupt entry to __interrupt
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);

        // enable external interrupt
        sie::set_sext();
        
        // 在 OpenSBI 中开启外部中断
        *PhysicalAddress(0x0c00_2080).deref_kernel() = 1u32 << 10;
        // 在 OpenSBI 中开启串口
        *PhysicalAddress(0x1000_0004).deref_kernel() = 0x0bu8;
        *PhysicalAddress(0x1000_0001).deref_kernel() = 0x01u8;
        // 其他一些外部中断相关魔数
        *PhysicalAddress(0x0C00_0028).deref_kernel() = 0x07u32;
        *PhysicalAddress(0x0C20_1000).deref_kernel() = 0u32;
    }
}
/*
/// entry of interrupt handler
///
/// `interrupt.asm` save Context, and spread as arguments with scause and stval
/// type of interrupt judged from scause and treat in different ways
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    // println!("stval: {}", stval);
    match scause.cause() {
        // breakpoint interrupt（ebreak）
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        // system call
        Trap::Exception(Exception::UserEnvCall) => syscall_handler(context),
        // time interrupt
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        // External interrupt
        Trap::Interrupt(Interrupt::SupervisorExternal) => supervisor_external(context),
        // others unimplemented
        _ => unimplemented!("{:?}: {:x?}, stval: 0x{:x}", scause.cause(), context, stval),
    }
}*/

/// entry of interrupt handler
///
/// `interrupt.asm` save Context, and spread as arguments with scause and stval
/// type of interrupt judged from scause and treat in different ways
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) -> *mut Context{
    // println!("stval: {}", stval);
    let temp_idt = idt::IDT::new();
    let idt_id = match scause.cause() {
        // breakpoint interrupt（ebreak）
        Trap::Exception(Exception::Breakpoint) => 0,
        // system call
        Trap::Exception(Exception::UserEnvCall) => 1,
        // time interrupt
        Trap::Interrupt(Interrupt::SupervisorTimer) => 2,
        // External interrupt
        Trap::Interrupt(Interrupt::SupervisorExternal) => 3,
        // others unimplemented
        //_ => unimplemented!("{:?}: {:x?}, stval: 0x{:x}", scause.cause(), context, stval),
        _ => 4,
    };

    if idt_id == 4 {
        handle_function::fault(context, scause, stval)
    }
    else {
        let (base, offset) = (temp_idt.gates[idt_id].base, temp_idt.gates[idt_id].offset);
        //handle_function::get_handle_function(offset, context);
        unsafe {
            (&__INTERRUPTS[offset].handler)(context)
        }
    }
}




