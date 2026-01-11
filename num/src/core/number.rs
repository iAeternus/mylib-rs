use super::{One, Zero};

/// 数值类型
pub trait Number: Zero + One + Copy + Clone + PartialEq + core::fmt::Debug {}

impl<T> Number for T where T: Zero + One + Copy + Clone + PartialEq + core::fmt::Debug {}
