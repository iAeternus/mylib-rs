use crate::thread_pool::config::RejectionPolicy;
use crate::thread_pool::pool::ThreadPool;

pub struct ThreadPoolBuilder {
    pub(crate) thread_count: usize,
    pub(crate) queue_capacity: usize,
    pub(crate) thread_name: String,
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
    pub fn thread_count(mut self, count: usize) -> Self {
        assert!(count > 0, "thread_count must be > 0");
        self.thread_count = count;
        self
    }

    pub fn queue_capacity(mut self, cap: usize) -> Self {
        assert!(cap > 0, "queue_capacity must be > 0");
        self.queue_capacity = cap;
        self
    }

    pub fn thread_name<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.thread_name = name.into();
        self
    }

    pub fn priority_levels(mut self, n: usize) -> Self {
        assert!(n > 0 && n <= 256, "priority_levels must be 1..=256");
        self.priority_levels = n;
        self
    }

    pub fn rejection_policy(mut self, p: RejectionPolicy) -> Self {
        self.rejection_policy = p;
        self
    }

    pub fn build(self) -> ThreadPool {
        ThreadPool::with_builder(self)
    }
}
