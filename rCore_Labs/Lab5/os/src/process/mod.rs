mod kernel_stack;
mod config;
#[allow(clippy::module_inception)]
mod thread;
mod process;
mod processor;
use crate::interrupt::*;
use crate::memory::*;
use alloc::{sync::Arc, vec, vec::Vec};
//use alloc::{sync::Arc};
use spin::{Mutex, RwLock};

pub use process::Process;
pub use thread::Thread;
pub use config::*;
pub use kernel_stack::KERNEL_STACK;
pub use processor::PROCESSOR;
