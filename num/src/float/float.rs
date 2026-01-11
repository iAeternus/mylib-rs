use crate::core::Number;
use core::ops::Div;

/// 浮点数
pub trait Float: Number + PartialOrd + Div<Output = Self> {
    /// not a number
    fn nan() -> Self;

    /// 正无穷
    fn infinity() -> Self;

    /// 负无穷
    fn neg_infinity() -> Self;

    /// 是否为NaN
    fn is_nan(self) -> bool;

    /// 是否为有限浮点数
    /// 
    /// # Return
    /// - `true`：该值既不是 NaN，也不是正负无穷
    /// - `false`：该值为 NaN 或 正负无穷
    fn is_finite(self) -> bool;

    /// 绝对值
    fn abs(self) -> Self;

    /// 平方根
    fn sqrt(self) -> Self;

    /// 整数幂
    fn powi(self, n: i32) -> Self;
}
