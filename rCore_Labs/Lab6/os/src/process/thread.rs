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
    pub process: Arc<Process>,
    /// Some vals
    pub inner: Mutex<ThreadInner>,
}

/// changable part of thread
pub struct ThreadInner {
    /// Context
    pub context: Option<Context>,
    /// is sleep or not
    pub sleeping: bool,
    /// is dead or not
    pub dead: bool,
}

impl Thread {
    /// prepare a process
    /// 
    /// activate page table and return Context
    pub fn prepare(&self) -> *mut Context {
        // activate page table
        self.process.inner().memory_set.activate();
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
        process: Arc<Process>,
        entry_point: usize,
        arguments: Option<&[usize]>,
        priority: usize,
    ) -> MemoryResult<Arc<Thread>> {
        // 让所属进程分配并映射一段空间，作为线程的栈
        let stack = process
            .alloc_page_range(STACK_SIZE, Flags::READABLE | Flags::WRITABLE)?;
        
        // 构建线程的 Context            
        let context = Context::new(
            stack.end.into(),
            entry_point,
            arguments,
            process.is_user,
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
                dead: false,
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

    /// fork a thread
    pub fn fork(&self, cur_context: Context, priority: usize) -> MemoryResult<Arc<Thread>> {
        println!("fork here");
        let stack = self.process
            .alloc_page_range(STACK_SIZE, Flags::READABLE | Flags::WRITABLE)?;
        for i in 0..STACK_SIZE {
            *VirtualAddress(stack.start.0 + i).deref::<u8>() = *VirtualAddress(self.stack.start.0 + i).deref::<u8>()
        }
        let mut new_context = cur_context.clone();
        new_context.set_sp( usize::from(stack.start) -  usize::from(self.stack.start) + cur_context.sp()  );
        let thread = Arc::new(Thread {
            id: unsafe {
                THREAD_COUNTER += 1;
                THREAD_COUNTER
            },
            priority: priority,
            stack,
            process: Arc::clone(&self.process),
            inner: Mutex::new(ThreadInner {
                context: Some(new_context),
                sleeping: false,
                dead: false,
            }),
        });
        Ok(thread)
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
