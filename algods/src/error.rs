use core::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlgodsError {
    /// 使用了无效或过期的 NodeId
    InvalidNodeId,

    /// 对根节点执行了非法操作（如删除）
    CannotRemoveRoot,
}

pub type AlgodsResult<T> = core::result::Result<T, AlgodsError>;

impl std::error::Error for AlgodsError {}

impl Display for AlgodsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            AlgodsError::InvalidNodeId => write!(f, "invalid or stale NodeId"),
            AlgodsError::CannotRemoveRoot => write!(f, "cannot remove root node"),
        }
    }
}
