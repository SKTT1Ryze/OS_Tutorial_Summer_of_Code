//! thread [`Thread`]

use super::*;
use core::hash::{Hash,Hasher};
// ID of thread, `isize`, negetive defines error
pub type ThreadID = isize;
static mut THREAD_COUNTER: ThreadID = 0;

/// TCB
pub struct Thread {
    /// ID
    pub id: ThreadID,
    /// priority
    pub priority: usize,
    /// Stack
    pub stack: Range<VirtualAddress>,
    /// process belonged
    pub process: Arc<RwLock<Process>>,
    /// Some vals
    pub inner: Mutex<ThreadInner>,
}

/// changable part of thread
pub struct ThreadInner {
    /// Context
    pub context: Option<Context>,
    /// is sleep or not
    pub sleeping: bool,
}

impl Thread {
    /// prepare a process
    /// 
    /// activate page table and return Context
    pub fn prepare(&self) -> *mut Context {
        // activate page table
        self.process.write().memory_set.activate();
        // get Context
        let parked_frame = self.inner().context.take().unwrap();
        // push Context in kernel stack
        unsafe { KERNEL_STACK.push_context(parked_frame) }
    }
    pub fn inner(&self) -> spin::MutexGuard<ThreadInner> {
        self.inner.lock()
    }

    /// create a new thread
    pub fn new (
        process: Arc<RwLock<Process>>,
        entry_point: usize,
        arguments: Option<&[usize]>,
        priority: usize,
    ) -> MemoryResult<Arc<Thread>> {
        // 让所属进程分配并映射一段空间，作为线程的栈
        let stack = process
            .write()
            .alloc_page_range(STACK_SIZE, Flags::READABLE | Flags::WRITABLE)?;
        
        // 构建线程的 Context            
        let context = Context::new(
            stack.end.into(),
            entry_point,
            arguments,
            process.read().is_user,
        );

        // 打包成线程
        let thread = Arc::new(Thread {
            id: unsafe {
                THREAD_COUNTER += 1;
                THREAD_COUNTER
            },
            priority: priority,
            stack,
            process,
            inner: Mutex::new(ThreadInner {
                context: Some(context),
                sleeping: false,
            }),
        });
        Ok(thread)
    }
    /// stop thread when time interrupt occur, and save Context
    pub fn park(&self, context: Context) {
        // check context of current thread, should be None
        assert!(self.inner().context.is_none());
        // save Context in thread
        self.inner().context.replace(context);
    }
}   

/// define equal by ID of thread
impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// 通过线程 ID 来判等
///
/// 在 Rust 中，[`PartialEq`] trait 不要求任意对象 `a` 满足 `a == a`。
/// 将类型标注为 [`Eq`]，会沿用 `PartialEq` 中定义的 `eq()` 方法，
/// 同时声明对于任意对象 `a` 满足 `a == a`。
impl Eq for Thread {}

/// hash with ID of thread
impl Hash for Thread {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_isize(self.id);
    }
}

/// 打印线程除了父进程以外的信息
impl core::fmt::Debug for Thread {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("Thread")
            .field("thread_id", &self.id)
            .field("stack", &self.stack)
            .field("context", &self.inner().context)
            .finish()
    }
}
