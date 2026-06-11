/// 任务优先级。0 最高，255 最低，默认 128。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Priority(u8);

impl Priority {
    pub const HIGHEST: Priority = Priority(0);
    pub const LOWEST: Priority = Priority(255);
    pub const NORMAL: Priority = Priority(128);

    pub const fn new(val: u8) -> Self {
        Self(val)
    }

    pub fn into_inner(self) -> u8 {
        self.0
    }
}

impl Default for Priority {
    fn default() -> Self {
        Self::NORMAL
    }
}

impl From<u8> for Priority {
    fn from(val: u8) -> Self {
        Self(val)
    }
}

/// 队列拒绝策略（容量满时生效）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RejectionPolicy {
    /// 调用者阻塞直到有空位。
    #[default]
    Block,
    /// 立即返回 `Err(QueueFull)`。
    Abort,
    /// 静默丢弃新任务。
    Discard,
    /// 从最低优先级队列移除一个最旧任务，然后提交新任务。
    DiscardOldest,
    /// 由调用者线程同步执行任务。
    CallerRuns,
}
