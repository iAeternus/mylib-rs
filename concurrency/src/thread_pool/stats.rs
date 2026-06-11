use std::sync::atomic::{AtomicUsize, Ordering};

/// 线程池快照统计。
#[derive(Debug, Clone, Copy)]
pub struct PoolStats {
    /// 累计提交数
    pub submitted: usize,
    /// 当前执行中
    pub running: usize,
    /// 累计完成数
    pub completed: usize,
    /// 被拒绝数
    pub rejected: usize,
    /// 崩溃任务数
    pub panicked: usize,
    /// 尚未完成数（= submitted - completed）
    pub pending: usize,
}

/// 线程安全的共享计数器（原子操作）。
pub(crate) struct SharedStats {
    submitted: AtomicUsize,
    running: AtomicUsize,
    completed: AtomicUsize,
    rejected: AtomicUsize,
    panicked: AtomicUsize,
}

impl SharedStats {
    pub(crate) fn new() -> Self {
        Self {
            submitted: AtomicUsize::new(0),
            running: AtomicUsize::new(0),
            completed: AtomicUsize::new(0),
            rejected: AtomicUsize::new(0),
            panicked: AtomicUsize::new(0),
        }
    }

    pub(crate) fn task_submitted(&self) {
        self.submitted.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn task_started(&self) {
        self.running.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn task_completed(&self) {
        self.running.fetch_sub(1, Ordering::Relaxed);
        self.completed.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn task_rejected(&self) {
        self.rejected.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn task_panicked(&self) {
        self.panicked.fetch_add(1, Ordering::Relaxed);
    }

    /// 取快照（Relaxed 顺序，各计数器间可能不一致）。
    pub fn snapshot(&self) -> PoolStats {
        PoolStats {
            submitted: self.submitted.load(Ordering::Relaxed),
            running: self.running.load(Ordering::Relaxed),
            completed: self.completed.load(Ordering::Relaxed),
            rejected: self.rejected.load(Ordering::Relaxed),
            panicked: self.panicked.load(Ordering::Relaxed),
            pending: self.submitted.load(Ordering::Relaxed)
                - self.completed.load(Ordering::Relaxed),
        }
    }
}
