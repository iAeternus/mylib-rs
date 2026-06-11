use std::{
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crossbeam_channel as cb_channel;
use crossbeam_deque::{Injector, Steal, Stealer, Worker};

use crate::thread_pool::{
    builder::ThreadPoolBuilder,
    config::{Priority, RejectionPolicy},
    sleeper::Sleeper,
    stats::{PoolStats, SharedStats},
    worker::worker_loop,
};

pub(crate) type Task = Box<dyn FnOnce() + Send + 'static>;

/// 线程池操作错误。
#[derive(Debug)]
pub enum ThreadPoolError {
    /// 池已关闭，拒绝新任务。
    Closed,
    /// 队列已满（Abort 策略下）。
    QueueFull,
}

/// 异步任务句柄，通过 `join()` 等待结果。
pub struct TaskHandle<T> {
    receiver: cb_channel::Receiver<T>,
}

impl<T> TaskHandle<T> {
    /// 阻塞等待任务完成并获取返回值。
    pub fn join(self) -> Result<T, cb_channel::RecvError> {
        self.receiver.recv()
    }
}

/// Worker 与提交者共享的内部状态。
pub(crate) struct PoolState {
    pub shutdown: AtomicBool,
    pub shutdown_now: AtomicBool,
    /// 已提交未完成的任务数（用于容量控制）。
    pub pending: AtomicUsize,
    /// 容量上限。0 = 无限制。
    pub capacity: usize,
    /// 已退出的 Worker 数。
    pub exited: AtomicUsize,
    pub sleeper: Sleeper,
    pub stats: Arc<SharedStats>,
    /// 用于阻塞提交者的锁（Block 策略 + async wait）。
    pub blocker_lock: Mutex<()>,
    pub blocker_cvar: Condvar,
}

#[allow(dead_code)]
struct PoolInner {
    /// 优先级队列组，索引 0 = 最高优先级。
    injectors: Arc<Vec<Injector<Task>>>,
    /// 跨 Worker 窃取句柄。
    stealers: Arc<Vec<Stealer<Task>>>,
    workers: Vec<JoinHandle<()>>,
    state: Arc<PoolState>,
    priority_levels: usize,
    rejection_policy: RejectionPolicy,
}

impl Drop for PoolInner {
    fn drop(&mut self) {
        self.state.shutdown.store(true, Ordering::Release);
        self.state.shutdown_now.store(true, Ordering::Release);
        self.state.sleeper.unpark_all();
        {
            let _g = self.state.blocker_lock.lock().unwrap();
            self.state.blocker_cvar.notify_all();
        }
        for h in self.workers.drain(..) {
            let _ = h.join();
        }
    }
}

/// 基于工作窃取的多级优先级线程池。
///
/// # 示例
/// ```
/// use concurrency::thread_pool::ThreadPool;
///
/// let pool = ThreadPool::new(4);
/// pool.exec(|| println!("hello")).unwrap();
/// pool.shutdown();
/// ```
pub struct ThreadPool {
    inner: Option<Box<PoolInner>>,
}

impl ThreadPool {
    /// 创建指定 Worker 数的线程池（其余参数取默认值）。
    pub fn new(count: usize) -> Self {
        ThreadPoolBuilder::default().thread_count(count).build()
    }

    /// 返回构建器，可定制各项参数。
    pub fn builder() -> ThreadPoolBuilder {
        ThreadPoolBuilder::default()
    }

    pub(crate) fn with_builder(builder: ThreadPoolBuilder) -> Self {
        let thread_count = builder.thread_count;
        let priority_levels = builder.priority_levels;
        let rejection_policy = builder.rejection_policy;
        let capacity = builder.queue_capacity;

        let injectors: Arc<Vec<Injector<Task>>> =
            Arc::new((0..priority_levels).map(|_| Injector::new()).collect());

        let state = Arc::new(PoolState {
            shutdown: AtomicBool::new(false),
            shutdown_now: AtomicBool::new(false),
            pending: AtomicUsize::new(0),
            capacity,
            exited: AtomicUsize::new(0),
            sleeper: Sleeper::new(),
            stats: Arc::new(SharedStats::new()),
            blocker_lock: Mutex::new(()),
            blocker_cvar: Condvar::new(),
        });

        let workers: Vec<Worker<Task>> = (0..thread_count).map(|_| Worker::new_lifo()).collect();
        let stealers: Arc<Vec<Stealer<Task>>> =
            Arc::new(workers.iter().map(|w| w.stealer()).collect());

        let join_handles: Vec<JoinHandle<()>> = workers
            .into_iter()
            .enumerate()
            .map(|(id, worker)| {
                let st = Arc::clone(&stealers);
                let inj = Arc::clone(&injectors);
                let s = Arc::clone(&state);

                thread::Builder::new()
                    .name(format!("{}-{}", builder.thread_name, id))
                    .spawn(move || {
                        worker_loop(id, worker, st, inj, s);
                    })
                    .expect("spawn worker thread")
            })
            .collect();

        Self {
            inner: Some(Box::new(PoolInner {
                injectors,
                stealers,
                workers: join_handles,
                state,
                priority_levels,
                rejection_policy,
            })),
        }
    }

    fn inner(&self) -> &PoolInner {
        self.inner.as_ref().expect("操作已终止的线程池")
    }

    /// 提交任务（不关心返回值），使用默认优先级。
    ///
    /// # 示例
    /// ```
    /// # use concurrency::thread_pool::ThreadPool;
    /// let pool = ThreadPool::new(2);
    /// pool.exec(|| println!("async task")).unwrap();
    /// pool.shutdown();
    /// ```
    pub fn exec<F>(&self, task: F) -> Result<(), ThreadPoolError>
    where
        F: FnOnce() + Send + 'static,
    {
        self.submit(Priority::default(), Box::new(task), false)
    }

    /// 非阻塞版 `exec`，满队列时不阻塞直接返回 `Err(QueueFull)`。
    pub fn try_exec<F>(&self, task: F) -> Result<(), ThreadPoolError>
    where
        F: FnOnce() + Send + 'static,
    {
        self.submit(Priority::default(), Box::new(task), true)
    }

    /// 提交指定优先级的任务。
    pub fn exec_with_priority<P, F>(&self, priority: P, task: F) -> Result<(), ThreadPoolError>
    where
        P: Into<Priority>,
        F: FnOnce() + Send + 'static,
    {
        self.submit(priority.into(), Box::new(task), false)
    }

    /// 提交任务并通过 `TaskHandle::join()` 获取返回值。
    ///
    /// # 示例
    /// ```
    /// # use concurrency::thread_pool::ThreadPool;
    /// let pool = ThreadPool::new(4);
    /// let handle = pool.spawn(|| 40 + 2).unwrap();
    /// assert_eq!(handle.join().unwrap(), 42);
    /// pool.shutdown();
    /// ```
    pub fn spawn<F, R>(&self, f: F) -> Result<TaskHandle<R>, ThreadPoolError>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        self.spawn_with_priority(Priority::default(), f)
    }

    /// 提交指定优先级的任务并获取返回值。
    pub fn spawn_with_priority<P, F, R>(
        &self,
        priority: P,
        f: F,
    ) -> Result<TaskHandle<R>, ThreadPoolError>
    where
        P: Into<Priority>,
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = cb_channel::bounded(1);
        let task = Box::new(move || {
            let result = f();
            let _ = tx.send(result);
        });
        self.submit(priority.into(), task, false)?;
        Ok(TaskHandle { receiver: rx })
    }

    /// 内部提交逻辑（含拒绝策略分发）。
    fn submit(
        &self,
        priority: Priority,
        task: Task,
        non_blocking: bool,
    ) -> Result<(), ThreadPoolError> {
        let inner = self.inner();
        let state = &inner.state;

        if state.shutdown.load(Ordering::Acquire) || state.shutdown_now.load(Ordering::Acquire) {
            return Err(ThreadPoolError::Closed);
        }

        if state.capacity > 0 {
            let current = state.pending.load(Ordering::Acquire);
            if current >= state.capacity {
                let effective =
                    if non_blocking && matches!(inner.rejection_policy, RejectionPolicy::Block) {
                        RejectionPolicy::Abort
                    } else {
                        inner.rejection_policy
                    };

                match effective {
                    RejectionPolicy::Block => {
                        let mut guard = state.blocker_lock.lock().unwrap();
                        loop {
                            if state.shutdown.load(Ordering::Acquire)
                                || state.shutdown_now.load(Ordering::Acquire)
                            {
                                return Err(ThreadPoolError::Closed);
                            }
                            if state.pending.load(Ordering::Acquire) < state.capacity {
                                break;
                            }
                            guard = state.blocker_cvar.wait(guard).unwrap();
                        }
                    }
                    RejectionPolicy::Abort => {
                        state.stats.task_rejected();
                        return Err(ThreadPoolError::QueueFull);
                    }
                    RejectionPolicy::Discard => {
                        state.stats.task_rejected();
                        return Ok(());
                    }
                    RejectionPolicy::CallerRuns => {
                        state.stats.task_submitted();
                        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(task));
                        if result.is_err() {
                            state.stats.task_panicked();
                        }
                        state.stats.task_completed();
                        return Ok(());
                    }
                    RejectionPolicy::DiscardOldest => {
                        let tmp = Worker::new_lifo();
                        let mut freed = false;
                        for inj in inner.injectors.iter().rev() {
                            loop {
                                match inj.steal_batch_and_pop(&tmp) {
                                    Steal::Success(_) => {
                                        while let Some(t) = tmp.pop() {
                                            inj.push(t);
                                        }
                                        state.pending.fetch_sub(1, Ordering::Release);
                                        freed = true;
                                        break;
                                    }
                                    Steal::Retry => thread::yield_now(),
                                    Steal::Empty => break,
                                }
                            }
                            if freed {
                                break;
                            }
                        }
                        if !freed {
                            state.stats.task_rejected();
                        }
                    }
                }
            }
        }

        if state.shutdown.load(Ordering::Acquire) || state.shutdown_now.load(Ordering::Acquire) {
            return Err(ThreadPoolError::Closed);
        }

        state.stats.task_submitted();
        state.pending.fetch_add(1, Ordering::Release);

        let idx = priority_index(priority, inner.priority_levels);
        inner.injectors[idx].push(task);
        state.sleeper.unpark_one();

        Ok(())
    }

    /// 获取线程池当前统计（近似值）。
    pub fn stats(&self) -> PoolStats {
        let inner = self.inner();
        let s = inner.state.stats.snapshot();
        PoolStats {
            pending: inner.state.pending.load(Ordering::Relaxed),
            ..s
        }
    }

    /// 优雅关闭：不再接受新任务，等待已有任务完成。
    ///
    /// # 示例
    /// ```
    /// # use concurrency::thread_pool::ThreadPool;
    /// let pool = ThreadPool::new(2);
    /// pool.shutdown();
    /// assert!(pool.is_shutdown());
    /// ```
    pub fn shutdown(&self) {
        if let Some(ref inner) = self.inner {
            inner.state.shutdown.store(true, Ordering::Release);
            inner.state.sleeper.unpark_all();
            let _g = inner.state.blocker_lock.lock().unwrap();
            inner.state.blocker_cvar.notify_all();
        }
    }

    /// 立即关闭：丢弃所有未执行任务，强制 Worker 退出。
    pub fn shutdown_now(&self) {
        if let Some(ref inner) = self.inner {
            inner.state.shutdown.store(true, Ordering::Release);
            inner.state.shutdown_now.store(true, Ordering::Release);
            inner.state.sleeper.unpark_all();
            let _g = inner.state.blocker_lock.lock().unwrap();
            inner.state.blocker_cvar.notify_all();
        }
    }

    /// 池是否已关闭（含 `shutdown_now`）。
    pub fn is_shutdown(&self) -> bool {
        self.inner.as_ref().is_none_or(|inner| {
            inner.state.shutdown.load(Ordering::Acquire)
                || inner.state.shutdown_now.load(Ordering::Acquire)
        })
    }

    /// 阻塞等待所有 Worker 退出。
    /// 调用后线程池不可再用。
    pub fn await_termination(&mut self) {
        if let Some(ref inner) = self.inner {
            inner.state.shutdown.store(true, Ordering::Release);
            inner.state.sleeper.unpark_all();
            let _g = inner.state.blocker_lock.lock().unwrap();
            inner.state.blocker_cvar.notify_all();
        }
        self.inner.take();
    }

    /// 带超时的 `await_termination`。超时返回 `Err(())`。
    #[allow(clippy::result_unit_err)]
    pub fn await_termination_timeout(&mut self, timeout: Duration) -> Result<(), ()> {
        let start = Instant::now();
        let target_workers = match self.inner.as_ref() {
            Some(inner) => {
                inner.state.shutdown.store(true, Ordering::Release);
                inner.state.sleeper.unpark_all();
                let _g = inner.state.blocker_lock.lock().unwrap();
                inner.state.blocker_cvar.notify_all();
                inner.workers.len()
            }
            None => return Ok(()),
        };

        let state = &self.inner.as_ref().unwrap().state;
        let mut guard = state.blocker_lock.lock().unwrap();
        loop {
            let exited = state.exited.load(Ordering::Acquire);
            if exited >= target_workers {
                drop(guard);
                self.inner.take();
                return Ok(());
            }
            let elapsed = start.elapsed();
            if elapsed >= timeout {
                return Err(());
            }
            let remaining = timeout.saturating_sub(elapsed);
            let result = state.blocker_cvar.wait_timeout(guard, remaining).unwrap();
            guard = result.0;
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.inner.take();
    }
}

/// 将 Priority(0..255) 映射到 Injector 数组索引。
fn priority_index(p: Priority, levels: usize) -> usize {
    let idx = (p.into_inner() as usize) * levels / 256;
    idx.min(levels - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(miri, ignore)]
    fn spawn_and_join() -> Result<(), ThreadPoolError> {
        let pool = ThreadPool::builder()
            .thread_count(8)
            .queue_capacity(5000)
            .thread_name("test-worker")
            .build();

        let handle = pool.spawn(|| 100 + 200)?;
        assert_eq!(handle.join().unwrap(), 300);
        Ok(())
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn exec_many() -> Result<(), ThreadPoolError> {
        let pool = ThreadPool::new(4);
        let n = 1000;
        let counter = Arc::new(AtomicUsize::new(0));
        for _ in 0..n {
            let c = Arc::clone(&counter);
            pool.exec(move || {
                c.fetch_add(1, Ordering::SeqCst);
            })?;
        }
        pool.shutdown();
        thread::sleep(Duration::from_millis(500));
        assert_eq!(counter.load(Ordering::SeqCst), n);
        Ok(())
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn priority_order() -> Result<(), ThreadPoolError> {
        let pool = ThreadPool::builder()
            .thread_count(1)
            .queue_capacity(100)
            .priority_levels(3)
            .build();

        let results = Arc::new(Mutex::new(Vec::new()));
        for i in 0..5 {
            let r = Arc::clone(&results);
            pool.exec_with_priority(Priority::LOWEST, move || {
                r.lock().unwrap().push(i);
            })?;
        }
        for i in 5..10 {
            let r = Arc::clone(&results);
            pool.exec_with_priority(Priority::HIGHEST, move || {
                r.lock().unwrap().push(i);
            })?;
        }
        pool.shutdown();
        thread::sleep(Duration::from_millis(500));
        let results = results.lock().unwrap();
        let last_high = results.iter().rposition(|&x| x >= 5).unwrap();
        let first_low = results.iter().position(|&x| x < 5).unwrap();
        assert!(last_high < first_low);
        assert_eq!(results.len(), 10);
        Ok(())
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn try_exec_queue_full() {
        let pool = ThreadPool::builder()
            .thread_count(1)
            .queue_capacity(1)
            .rejection_policy(RejectionPolicy::Abort)
            .build();

        let barrier = Arc::new(std::sync::Barrier::new(2));
        let b = Arc::clone(&barrier);
        pool.exec(move || {
            barrier.wait();
            thread::sleep(Duration::from_secs(1));
        })
        .unwrap();

        b.wait();
        thread::sleep(Duration::from_millis(20));

        for _ in 0..5 {
            let result = pool.try_exec(|| {});
            assert!(result.is_err());
        }

        pool.shutdown();
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn abort_rejection() {
        let pool = ThreadPool::builder()
            .thread_count(2)
            .queue_capacity(5)
            .rejection_policy(RejectionPolicy::Abort)
            .build();

        for i in 0..5 {
            let result = pool.exec(move || {
                thread::sleep(Duration::from_millis(10 * i));
            });
            assert!(result.is_ok(), "task {} should be accepted", i);
        }

        pool.shutdown();
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn caller_runs_policy() -> Result<(), ThreadPoolError> {
        let pool = ThreadPool::builder()
            .thread_count(1)
            .queue_capacity(1)
            .rejection_policy(RejectionPolicy::CallerRuns)
            .build();

        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let p = Arc::clone(&pair);
        pool.exec(move || {
            let (lock, cvar) = &*p;
            let mut guard = lock.lock().unwrap();
            *guard = true;
            cvar.notify_all();
            while *guard {
                guard = cvar.wait(guard).unwrap();
            }
        })?;

        let (lock, cvar) = &*pair;
        let mut guard = lock.lock().unwrap();
        while !*guard {
            guard = cvar.wait(guard).unwrap();
        }
        drop(guard);

        let flag = Arc::new(AtomicBool::new(false));
        let f = Arc::clone(&flag);
        pool.exec(move || {
            f.store(true, Ordering::SeqCst);
        })?;

        assert!(flag.load(Ordering::SeqCst));

        let (lock, cvar) = &*pair;
        let mut guard = lock.lock().unwrap();
        *guard = false;
        cvar.notify_all();
        drop(guard);

        pool.shutdown();
        Ok(())
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn shutdown_and_termination() {
        let pool = ThreadPool::new(4);
        pool.shutdown();
        assert!(pool.is_shutdown());
        let result = pool.exec(|| {});
        assert!(result.is_err());
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn stats_collection() -> Result<(), ThreadPoolError> {
        let pool = ThreadPool::new(2);
        pool.exec(|| {})?;
        pool.exec(|| {})?;
        pool.shutdown();
        thread::sleep(Duration::from_millis(300));
        let stats = pool.stats();
        assert_eq!(stats.submitted, 2);
        assert_eq!(stats.completed, 2);
        Ok(())
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn task_handle_panic() -> Result<(), ThreadPoolError> {
        let pool = ThreadPool::new(1);
        let handle = pool.spawn(|| -> i32 {
            panic!("intentional panic");
        })?;
        let result = handle.join();
        assert!(result.is_err());
        thread::sleep(Duration::from_millis(50));
        let stats = pool.stats();
        assert_eq!(stats.panicked, 1);
        Ok(())
    }
}
