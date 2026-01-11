use std::ops::Neg;

use crate::integer::integer::Integer;

/// 有符号数
pub trait SignedInteger: Integer + Neg<Output = Self> {
    fn is_negative(self) -> bool;
}