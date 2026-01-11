use super::{Number, Signed};

/// IEEE 754 浮点语义
pub trait Float: Number + Signed {
    /// NaN
    fn nan() -> Self;

    /// 正无穷
    fn infinity() -> Self;

    /// 负无穷
    fn neg_infinity() -> Self;

    /// 是否为 NaN
    fn is_nan(self) -> bool;

    /// 是否为有限值
    fn is_finite(self) -> bool;

    /// 平方根
    fn sqrt(self) -> Self;

    /// 整数幂
    fn powi(self, n: i32) -> Self;
}
