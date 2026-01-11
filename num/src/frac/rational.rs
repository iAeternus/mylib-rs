use crate::core::{Number, Signed};

/// 有理数（分数）
pub trait Rational: Number + Signed {
    /// 分子
    fn numer(&self) -> Self;

    /// 分母（永远 > 0）
    fn denom(&self) -> Self;

    /// 约分
    fn reduce(self) -> Self;
}