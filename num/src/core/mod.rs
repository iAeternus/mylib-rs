pub mod float;
pub mod integer;
pub mod number;
pub mod one;
pub mod signed;
pub mod unsigned;
pub mod zero;

pub use float::*;
pub use integer::*;
pub use number::*;
pub use one::*;
pub use signed::*;
pub use unsigned::*;
pub use zero::*;

/// 模
pub trait Norm {
    type Output: Number;

    /// 模
    fn norm(&self) -> Self::Output;

    /// 模的平方
    fn norm_sq(&self) -> Self::Output;
}

/// 近似相等比较
pub trait ApproxEq<Rhs = Self> {
    fn approx_eq(&self, rhs: &Rhs, eps: f64) -> bool;
}
