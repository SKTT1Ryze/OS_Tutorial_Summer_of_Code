#![allow(unused)]
use super::timer;
use super::context::Context;
use riscv::register::{
    stvec,
    sie,
    scause::{Exception, Interrupt, Scause, Trap},
};

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
    }
}

/// entry of interrupt handler
/// 
/// `interrupt.asm` save Context, and spread as arguments with scause and stval
/// type of interrupt judged from scause and treat in different ways
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    // println!("{:x?}", context.scause.cause());
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
}

/// handle ebreak interrupt
/// 
/// continue: sepc add 2 to continue
fn breakpoint(context: &mut Context) {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
}
/// handle system call
/// 
/// continue: sepc add 2 to continue
fn syscall_handler(context: &mut Context) {
    println!("system call at 0x{:x}", context.sepc);
    context.sepc += 2;
}

/// handle time interrupt
/// 
/// count in `tick()` and call a ebreak
fn supervisor_timer(_: &Context) {
    timer::tick();
}

/// handle external interrupt
/// 
/// continue: sepc add 2 to continue
fn supervisor_external(context: &mut Context) {
    println!("External interrupt at 0x{:x}", context.sepc);
    context.sepc += 2;
}
