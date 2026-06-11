use crate::thread_pool::config::RejectionPolicy;
use crate::thread_pool::pool::ThreadPool;

/// 线程池构建器。
pub struct ThreadPoolBuilder {
    pub(crate) thread_count: usize,
    /// 全局积压上限。0 表示无限制。
    pub(crate) queue_capacity: usize,
    pub(crate) thread_name: String,
    /// 优先级级数（1~256）。
    pub(crate) priority_levels: usize,
    pub(crate) rejection_policy: RejectionPolicy,
}

impl Default for ThreadPoolBuilder {
    fn default() -> Self {
        Self {
            thread_count: 4,
            queue_capacity: 1024,
            thread_name: "worker".into(),
            priority_levels: 4,
            rejection_policy: RejectionPolicy::default(),
        }
    }
}

impl ThreadPoolBuilder {
    /// 设置 Worker 线程数。
    pub fn thread_count(mut self, count: usize) -> Self {
        assert!(count > 0, "thread_count must be > 0");
        self.thread_count = count;
        self
    }

    /// 设置全局队列容量上限（所有优先级合计）。
    pub fn queue_capacity(mut self, cap: usize) -> Self {
        assert!(cap > 0, "queue_capacity must be > 0");
        self.queue_capacity = cap;
        self
    }

    /// 设置 Worker 线程名称前缀。
    pub fn thread_name<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.thread_name = name.into();
        self
    }

    /// 设置优先级级数（1~256，默认 4）。级数越多粒度越细，但匹配开销略增。
    pub fn priority_levels(mut self, n: usize) -> Self {
        assert!(n > 0 && n <= 256, "priority_levels must be 1..=256");
        self.priority_levels = n;
        self
    }

    /// 设置拒绝策略。
    pub fn rejection_policy(mut self, p: RejectionPolicy) -> Self {
        self.rejection_policy = p;
        self
    }

    /// 构建线程池。
    pub fn build(self) -> ThreadPool {
        ThreadPool::with_builder(self)
    }
}
