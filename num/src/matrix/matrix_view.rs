use std::ops::{Index, IndexMut, Range};

use crate::{
    Number,
    matrix::{Matrix, MatrixBase},
};

/// 矩阵视图
#[derive(Debug, Clone)]
pub struct MatrixView<'a, T: Number> {
    pub(crate) matrix: &'a Matrix<T>,
    pub(crate) row_range: Range<usize>,
    pub(crate) col_range: Range<usize>,
}

/// 矩阵视图，可变
#[derive(Debug)]
pub struct MatrixViewMut<'a, T: Number> {
    pub(crate) matrix: &'a mut Matrix<T>,
    pub(crate) row_range: Range<usize>,
    pub(crate) col_range: Range<usize>,
}

impl<'a, T: Number> MatrixView<'a, T> {
    #[inline]
    unsafe fn get_unchecked(&self, i: usize, j: usize) -> &T {
        unsafe {
            self.matrix
                .get_unchecked(self.row_range.start + i, self.col_range.start + j)
        }
    }

    /// 转换为矩阵，需要克隆数据
    pub fn to_matrix(&self) -> Matrix<T> {
        let rows = self.rows();
        let cols = self.cols();
        let mut data = Vec::with_capacity(rows * cols);

        for i in self.row_range.clone() {
            for j in self.col_range.clone() {
                unsafe {
                    let elem = self.matrix.get_unchecked(i, j);
                    data.push(elem.clone());
                }
            }
        }

        unsafe { Matrix::new_unchecked(rows, cols, data) }
    }
}

impl<'a, T: Number> MatrixViewMut<'a, T> {
    #[inline]
    pub unsafe fn get_unchecked(&self, i: usize, j: usize) -> &T {
        unsafe {
            self.matrix
                .get_unchecked(self.row_range.start + i, self.col_range.start + j)
        }
    }

    #[inline]
    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        if i >= self.rows() || j >= self.cols() {
            None
        } else {
            unsafe { Some(self.get_mut_unchecked(i, j)) }
        }
    }

    #[inline]
    pub unsafe fn get_mut_unchecked(&mut self, i: usize, j: usize) -> &mut T {
        unsafe {
            self.matrix
                .get_mut_unchecked(self.row_range.start + i, self.col_range.start + j)
        }
    }
}

impl<'a, T: Number> MatrixBase<T> for MatrixView<'a, T> {
    #[inline]
    fn rows(&self) -> usize {
        self.row_range.len()
    }

    #[inline]
    fn cols(&self) -> usize {
        self.col_range.len()
    }

    #[inline]
    fn get(&self, i: usize, j: usize) -> Option<&T> {
        if i >= self.rows() || j >= self.cols() {
            None
        } else {
            unsafe { Some(self.get_unchecked(i, j)) }
        }
    }
}

impl<'a, T: Number> MatrixBase<T> for MatrixViewMut<'a, T> {
    #[inline]
    fn rows(&self) -> usize {
        self.row_range.len()
    }

    #[inline]
    fn cols(&self) -> usize {
        self.col_range.len()
    }

    #[inline]
    fn get(&self, i: usize, j: usize) -> Option<&T> {
        if i >= self.rows() || j >= self.cols() {
            None
        } else {
            unsafe { Some(self.get_unchecked(i, j)) }
        }
    }
}

impl<'a, T: Number> Index<(usize, usize)> for MatrixView<'a, T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        self.get(i, j).unwrap_or_else(|| {
            panic!(
                "Index ({}, {}) out of bounds for submatrix of size {}x{}",
                i,
                j,
                self.rows(),
                self.cols()
            );
        })
    }
}

impl<'a, T: Number> Index<(usize, usize)> for MatrixViewMut<'a, T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        self.get(i, j).unwrap_or_else(|| {
            panic!(
                "Index ({}, {}) out of bounds for submatrix of size {}x{}",
                i,
                j,
                self.rows(),
                self.cols()
            );
        })
    }
}

impl<'a, T: Number> IndexMut<(usize, usize)> for MatrixViewMut<'a, T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        let rows = self.rows();
        let cols = self.cols();
        self.get_mut(i, j).unwrap_or_else(|| {
            panic!(
                "Index ({}, {}) out of bounds for submatrix of size {}x{}",
                i, j, rows, cols
            );
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_view() {
        let m = Matrix::from([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);

        // 获取 2x2 子矩阵 (从 (1,1) 开始)
        let sub = m.slice(1..3, 1..3).unwrap();

        assert_eq!(sub.rows(), 2);
        assert_eq!(sub.cols(), 2);
        assert_eq!(sub[(0, 0)], 6); // m[1,1]
        assert_eq!(sub[(0, 1)], 7); // m[1,2]
        assert_eq!(sub[(1, 0)], 10); // m[2,1]
        assert_eq!(sub[(1, 1)], 11); // m[2,2]

        // 转换为独立矩阵
        let sub_matrix = sub.to_matrix();
        assert_eq!(sub_matrix[(0, 0)], 6);
        assert_eq!(sub_matrix[(1, 1)], 11);
    }

    #[test]
    fn test_matrix_view_mut() {
        let mut m = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        {
            let mut sub = m.slice_mut(0..2, 1..3).unwrap();
            sub[(0, 0)] = 100; // 修改 m[0,1]
            sub[(1, 1)] = 200; // 修改 m[1,2]
        }

        assert_eq!(m[(0, 1)], 100);
        assert_eq!(m[(1, 2)], 200);
    }
}
