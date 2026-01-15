use crate::core::Number;

/// 向量语义
pub trait Vector: Sized + Clone {
    type Scalar: Number;

    /// 维度
    fn dim(&self) -> usize;

    /// 点积
    fn dot(&self, rhs: &Self) -> Self::Scalar;
}