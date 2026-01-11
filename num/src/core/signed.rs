use std::ops::Neg;

/// 有符号数值
pub trait Signed: Neg<Output = Self> {
    /// 绝对值
    fn abs(self) -> Self;

    /// 是否为负
    fn is_negative(self) -> bool;
}
