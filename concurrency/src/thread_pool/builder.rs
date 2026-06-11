use crate::thread_pool::pool::ThreadPool;

pub struct ThreadPoolBuilder {
    pub(crate) thread_count: usize,
    pub(crate) queue_capacity: usize,
    pub(crate) thread_name: String,
}

impl Default for ThreadPoolBuilder {
    fn default() -> Self {
        Self {
            thread_count: 4,
            queue_capacity: 1024,
            thread_name: "worker".into(),
        }
    }
}

impl ThreadPoolBuilder {
    pub fn thread_count(mut self, count: usize) -> Self {
        assert!(count > 0);

        self.thread_count = count;
        self
    }

    pub fn queue_capacity(mut self, cap: usize) -> Self {
        assert!(cap > 0);

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

    pub fn build(self) -> ThreadPool {
        ThreadPool::with_builder(self)
    }
}
