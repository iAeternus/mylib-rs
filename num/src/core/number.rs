use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::{One, Zero};

/// 数值类型
pub trait Number:
    Zero
    + One
    + Copy
    + Clone
    + PartialEq
    + core::fmt::Debug
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{
}

impl<T> Number for T where
    T: Zero
        + One
        + Copy
        + Clone
        + PartialEq
        + core::fmt::Debug
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
{
}
