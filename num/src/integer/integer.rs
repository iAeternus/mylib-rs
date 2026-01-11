use std::ops::{Div, Rem};

use crate::core::Number;

/// 整数
pub trait Integer: Number + Eq + Ord + Rem<Output = Self> + Div<Output = Self> {
    /// 绝对值
    fn abs(self) -> Self;

    /// 最大公约数
    fn gcd(self, other: Self) -> Self;

    /// 最小公倍数
    fn lcm(self, other: Self) -> Self {
        self / self.gcd(other) * other
    }
}
