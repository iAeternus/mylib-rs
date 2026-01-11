use crate::core::{Integer, Number, One, Signed};

/// 有理数语义（分数）
pub trait Rational: Number + Signed {
    /// 整数标量类型
    type Int: Integer;

    /// 分子
    fn numer(&self) -> Self::Int;

    /// 分母（保证 > 0）
    fn denom(&self) -> Self::Int;

    /// 返回约分后的自身
    fn reduce(self) -> Self;

    /// 是否为整数
    fn is_integer(&self) -> bool {
        self.denom().is_one()
    }
}
