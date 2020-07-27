//! struct [`Context`] to save status

use core::fmt;
use core::mem::zeroed;
use riscv::register::sstatus::{self, Sstatus, SPP::*};

/// need to be saved registers
///
/// x*
/// - `sstatus`:saved status
/// - `sepc`：address of interrupt occurs
///
/// ### `#[repr(C)]`
/// arrange the memory like C
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Context {
    /// x0~x31
    pub x: [usize; 32],
    /// sstatus
    pub sstatus: Sstatus,
    /// sepc
    pub sepc: usize,
}

/// create a context initialized with 0
///
/// use [`core::mem::zeroed()`] to initialized with 0
impl Default for Context {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

/// format print
///
/// # Example
///
/// ```rust
/// println!("{:x?}", Context);   // print with 0x....
/// ```
impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Context")
            .field("registers", &self.x)
            .field("sstatus", &self.sstatus)
            .field("sepc", &self.sepc)
            .finish()
    }
}

#[allow(unused)]
impl Context {
    /// get sp
    pub fn sp(&self) -> usize {
        self.x[2]
    }

    /// set sp
    pub fn set_sp(&mut self, value: usize) -> &mut Self {
        self.x[2] = value;
        self
    }

    /// get return address
    pub fn ra(&self) -> usize {
        self.x[1]
    }

    /// set return address
    pub fn set_ra(&mut self, value: usize) -> &mut Self {
        self.x[1] = value;
        self
    }

    /// set arguments with function specification
    ///
    /// don't work for more than 8 arguments and struct spread
    pub fn set_arguments(&mut self, arguments: &[usize]) -> &mut Self {
        assert!(arguments.len() <= 8);
        self.x[10..(10 + arguments.len())].copy_from_slice(arguments);
        self
    }

    /// creat initialized `Context` for thread
    pub fn new(
        stack_top: usize,
        entry_point: usize,
        arguments: Option<&[usize]>,
        is_user: bool,
    ) -> Self {
        let mut context = Self::default();

        // set sp
        context.set_sp(stack_top).set_ra(-1isize as usize);
        // set arguments
        if let Some(args) = arguments {
            context.set_arguments(args);
        }
        // set entry point
        context.sepc = entry_point;

        // set sstatus
        context.sstatus = sstatus::read();
        if is_user {
            context.sstatus.set_spp(User);
        } else {
            context.sstatus.set_spp(Supervisor);
        }
        // 这样设置 SPIE 位，使得替换 sstatus 后关闭中断，
        // 而在 sret 到用户线程时开启中断。详见 SPIE 和 SIE 的定义
        context.sstatus.set_spie(true);

        context
    }
}
