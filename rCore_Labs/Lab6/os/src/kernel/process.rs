//! 进程相关的内核功能

use super::*;

pub(super) fn sys_exit(code: usize) -> SyscallResult {
    println!(
        "thread {} exit with code {}",
        PROCESSOR.lock().current_thread().id,
        code
    );
    SyscallResult::Kill
}

pub(super) fn sys_get_tid() -> SyscallResult {
    let id = PROCESSOR.lock().current_thread().id.clone();
    SyscallResult::Proceed(id)
}

pub(super) fn sys_fork(context: &Context) -> SyscallResult {
    let fork_id = PROCESSOR.lock().current_thread().id.clone();
    PROCESSOR.lock().fork_current_thread(context);
    match PROCESSOR.lock().current_thread().id == fork_id {
        true => {
            SyscallResult::Proceed(fork_id)
        },
        false => {
            SyscallResult::Proceed(0)
        }
    }
}