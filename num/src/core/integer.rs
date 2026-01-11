use std::ops::{Neg, Rem, RemAssign};

use super::{Number, Signed};

/// 整数语义
pub trait Integer: Number + Signed + Rem<Output = Self> + RemAssign + Neg<Output = Self> {
    /// 最大公约数
    fn gcd(self, other: Self) -> Self;

    /// 最小公倍数
    fn lcm(self, other: Self) -> Self;
}
