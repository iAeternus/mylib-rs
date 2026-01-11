use std::ops::{Add, Mul, Sub};

/// 加法操作
pub trait Additive: Sized + Add<Output = Self> + Sub<Output = Self> {}

impl<T> Additive for T where T: Add<Output = T> + Sub<Output = T> {}

/// 乘法操作
pub trait Multiplicative: Sized + Mul<Output = Self> {}

impl<T> Multiplicative for T where T: Mul<Output = T> {}
