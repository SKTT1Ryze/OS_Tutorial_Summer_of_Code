//! scheduling of thread and process [`Processor`]
#![allow(dead_code)]
use super::*;
use algorithm::*;
use hashbrown::HashSet;
use lazy_static::*;

lazy_static! {
    /// global [`Processor`]
    pub static ref PROCESSOR: Lock<Processor> = Lock::new(Processor::default());
}

lazy_static! {
    /// 空闲线程：当所有线程进入休眠时，切换到这个线程——它什么都不做，只会等待下一次中断
    static ref IDLE_THREAD: Arc<Thread> = Thread::new(
        Process::new_kernel().unwrap(),
        wait_for_interrupt as usize,
        None,
        1,
    ).unwrap();
}

/// 不断让 CPU 进入休眠等待下一次中断
unsafe fn wait_for_interrupt() {
    loop {
        llvm_asm!("wfi" :::: "volatile");
    }
}

/// 线程调度和管理
///
/// 休眠线程会从调度器中移除，单独保存。在它们被唤醒之前，不会被调度器安排。
///
/// # 用例
/// ### 初始化并运行第一个线程
/// ```Rust
/// processor.add_thread(thread);
/// processor.run();
/// unreachable!();
/// ```
///
/// ### 切换线程（在中断中）
/// ```Rust
/// processor.park_current_thread(context);
/// processor.prepare_next_thread()
/// ```
///
/// ### 结束线程（在中断中）
/// ```Rust
/// processor.kill_current_thread();
/// processor.prepare_next_thread()
/// ```
///
/// ### 休眠线程（在中断中）
/// ```Rust
/// processor.park_current_thread(context);
/// processor.sleep_current_thread();
/// processor.prepare_next_thread()
/// ```
///
/// ### 唤醒线程
/// 线程会根据调度器分配执行，不一定会立即执行。
/// ```Rust
/// processor.wake_thread(thread);
/// ```
#[derive(Default)]
pub struct Processor {
    /// current thread
    current_thread: Option<Arc<Thread>>,
    /// thread scheduler
    scheduler: SchedulerImpl<Arc<Thread>>,
    /// save sleeping threads
    sleeping_threads: HashSet<Arc<Thread>>,
}

impl Processor {
    /// get `Arc` reference from current thread
    pub fn current_thread(&self) -> Arc<Thread> {
        self.current_thread.as_ref().unwrap().clone()
    }

    /// 第一次开始运行
    ///
    /// 从 `current_thread` 中取出 [`Context`]，然后直接调用 `interrupt.asm` 中的 `__restore`
    /// 来从 `Context` 中继续执行该线程。
    ///
    /// 注意调用 `run()` 的线程会就此步入虚无，不再被使用
    pub fn run(&mut self) -> ! {
        // __restore from interrupt.asm
        extern "C" {
            fn __restore(context: usize);
        }
        // get Context from current_thread
        if self.current_thread.is_none() {
            panic!("no thread to run, shutting down...");
        }
        let context = self.current_thread().prepare();
        // will not go back
        unsafe {
            __restore(context as usize);
        }
        unreachable!()
    }

    /// activate `Context` of next thread
    pub fn prepare_next_thread(&mut self) -> *mut Context {
        // ask for next thread from scheduler
        if let Some(next_thread) = self.scheduler.get_next() {
            // prepare next thread
            let context = next_thread.prepare();
            self.current_thread = Some(next_thread);
            return context;
        } else {
            // have no active threads
            if self.sleeping_threads.is_empty() {
                // nor the sleeping threads, then panic
                panic!("all threads terminated, shutting down...");
            } else {
                // have sleeping threads, waite for interrupt
                self.current_thread = Some(IDLE_THREAD.clone());
                IDLE_THREAD.prepare()
            }
        }
    }

    /// add a thread
    pub fn add_thread(&mut self, thread: Arc<Thread>) {
        if self.current_thread.is_none() {
            self.current_thread = Some(thread.clone());
        }
        let priority = thread.priority;
        self.scheduler.add_thread(thread, priority);
    }

    /// wake a thread
    pub fn wake_thread(&mut self, thread: Arc<Thread>) {
        thread.inner().sleeping = false;
        self.sleeping_threads.remove(&thread);
        self.scheduler.add_thread(thread, 0);
    }

    /// save `Context` of current thread
    pub fn park_current_thread(&mut self, context: &Context) {
        self.current_thread().park(*context);
    }

    /// make current thread sleep
    pub fn sleep_current_thread(&mut self) {
        // get current thread
        let current_thread = self.current_thread();
        // set to sleeping
        current_thread.inner().sleeping = true;
        // move to sleeping_threads from scheduler
        self.scheduler.remove_thread(&current_thread);
        self.sleeping_threads.insert(current_thread);
    }

    /// kill current thread
    pub fn kill_current_thread(&mut self) {
        // remove from scheduler
        println!("kill here");
        let thread = self.current_thread.take().unwrap();
        self.scheduler.remove_thread(&thread);
    }

    /// fork current thread
    pub fn fork_current_thread(&mut self, context: &Context) {
        let new_thread = self
            .current_thread()
            .fork(*context, self.current_thread().priority)
            .unwrap();
        self.add_thread(new_thread);
    }
}
