pub mod matrix;
pub mod matrix_view;
pub mod ops;

pub use matrix::*;
pub use matrix_view::*;
pub use ops::*;

use crate::Number;

/// 矩阵基础接口
pub trait MatrixBase<T: Number> {
    /// 获取行数
    fn rows(&self) -> usize;

    /// 获取列数
    fn cols(&self) -> usize;

    /// 索引访问
    fn get(&self, i: usize, j: usize) -> Option<&T>;

    ///
    unsafe fn get_unchecked(&self, i: usize, j: usize) -> &T;
}
