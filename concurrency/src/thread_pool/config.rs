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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RejectionPolicy {
    #[default]
    Block,
    Abort,
    Discard,
    DiscardOldest,
    CallerRuns,
}
