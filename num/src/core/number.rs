use crate::core::{
    one::One,
    ops::{Additive, Multiplicative},
    zero::Zero,
};

/// 数字
pub trait Number: Copy + Clone + PartialEq + Zero + One + Additive + Multiplicative {}

impl<T> Number for T where T: Copy + Clone + PartialEq + Zero + One + Additive + Multiplicative {}
