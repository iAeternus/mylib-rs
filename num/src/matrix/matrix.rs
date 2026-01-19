use std::ops::{Bound, Index, IndexMut, RangeBounds};

use crate::{
    Number,
    error::{NumError, NumResult},
    matrix::{MatrixBase, MatrixView, MatrixViewMut},
};

/// 二维矩阵
#[derive(Debug, Clone)]
pub struct Matrix<T: Number> {
    /// 行数
    pub(crate) rows: usize,
    /// 列数
    pub(crate) cols: usize,
    /// 数据
    pub(crate) data: Vec<T>,
}

impl<T: Number> Matrix<T> {
    /// 使用一维数组创建矩阵
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> NumResult<Self> {
        if data.len() != rows * cols {
            return Err(NumError::MatrixSizeMismatch {
                expect: rows * cols,
                actual: data.len(),
            });
        }
        unsafe { Ok(Self::new_unchecked(rows, cols, data)) }
    }

    /// 使用一维数组创建矩阵，不做维度检查
    ///
    /// ### Safety
    /// 调用者需保证维度正确
    pub unsafe fn new_unchecked(rows: usize, cols: usize, data: Vec<T>) -> Self {
        Self { rows, cols, data }
    }

    /// 从行迭代器创建矩阵
    ///
    /// ## Param
    /// - rows_iter: 行迭代器，每行是一个可迭代的 T 的集合，可以处理 Vec<Vec<T>>、&[&[T]]、Vec<&[T]> 等多种输入
    pub fn from_rows_iter<I, R>(rows_iter: I) -> NumResult<Self>
    where
        I: IntoIterator<Item = R>,
        R: IntoIterator<Item = T>,
    {
        let mut rows_count = 0;
        let mut cols_count = None;
        let mut data = Vec::new();

        for row in rows_iter.into_iter() {
            let mut row_len = 0;
            for elem in row.into_iter() {
                data.push(elem);
                row_len += 1;
            }

            // 检查列数一致性
            match cols_count {
                Some(expected) if row_len != expected => {
                    return Err(NumError::MatrixDimensionMismatch {
                        expect: (rows_count, expected),
                        actual: (rows_count, row_len),
                    });
                }
                Some(_) => {}
                None => cols_count = Some(row_len),
            }

            rows_count += 1;
        }

        let cols_count = cols_count.unwrap_or(0);

        // 验证数据长度
        if data.len() != rows_count * cols_count {
            return Err(NumError::MatrixSizeMismatch {
                expect: rows_count * cols_count,
                actual: data.len(),
            });
        }

        unsafe { Ok(Self::new_unchecked(rows_count, cols_count, data)) }
    }

    /// 判断是否为方阵
    #[inline]
    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    /// 判断是否形状相同
    #[inline]
    pub fn is_same_shape(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols
    }

    /// 创建零矩阵
    #[inline]
    pub fn zero(rows: usize, cols: usize) -> Self {
        unsafe { Self::new_unchecked(rows, cols, vec![T::zero(); rows * cols]) }
    }

    /// 创建单位矩阵
    #[inline]
    pub fn identity(n: usize) -> Self {
        let mut data = vec![T::zero(); n * n];
        for i in 0..n {
            data[i * n + i] = T::one();
        }
        unsafe { Self::new_unchecked(n, n, data) }
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, i: usize, j: usize) -> &T {
        &self.data[i * self.cols + j]
    }

    #[inline]
    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        if i >= self.rows || j >= self.cols {
            None
        } else {
            unsafe { Some(self.get_mut_unchecked(i, j)) }
        }
    }

    #[inline]
    pub unsafe fn get_mut_unchecked(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.data[i * self.cols + j]
    }

    /// 创建一个矩阵的不可变视图
    ///
    /// 返回指定行范围和列范围的子矩阵视图，不进行数据复制。
    /// 如果指定的范围无效或越界，则返回 `None`。
    ///
    /// ## Param
    ///
    /// - `rows`: 行范围，支持 `Range`、`RangeFrom`、`RangeTo`、`RangeFull` 等类型
    /// - `cols`: 列范围，支持 `Range`、`RangeFrom`、`RangeTo`、`RangeFull` 等类型
    ///
    /// ## Return
    ///
    /// 如果范围有效且不越界，返回 `Some(MatrixView)`；否则返回 `None`。
    ///
    /// ## Examples
    ///
    /// ```
    /// use num::matrix::Matrix;
    /// use crate::num::matrix::MatrixBase;
    ///
    /// let m = Matrix::from([[1, 2, 3, 4],
    ///                       [5, 6, 7, 8],
    ///                       [9, 10, 11, 12]]);
    ///
    /// // 获取 2x2 子矩阵（第1-2行，第2-3列）
    /// let view = m.slice(1..3, 1..3).unwrap();
    /// assert_eq!(view.rows(), 2);
    /// assert_eq!(view.cols(), 2);
    /// assert_eq!(view[(0, 0)], 6);   // 原始矩阵的 (1,1)
    ///
    /// // 使用不同的范围语法
    /// let first_two_rows = m.slice(..2, ..).unwrap();  // 前两行，所有列
    /// let last_two_cols = m.slice(.., 2..).unwrap();   // 所有行，后两列
    /// let middle_rows = m.slice(1..=2, ..).unwrap();   // 第2-3行（包含第3行）
    /// ```
    pub fn slice<R, C>(&self, rows: R, cols: C) -> Option<MatrixView<'_, T>>
    where
        R: RangeBounds<usize>,
        C: RangeBounds<usize>,
    {
        // 转换起始边界
        let row_start = match rows.start_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        };

        // 转换结束边界
        let row_end = match rows.end_bound() {
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => self.rows,
        };

        // 转换起始边界
        let col_start = match cols.start_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        };

        // 转换结束边界
        let col_end = match cols.end_bound() {
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => self.cols,
        };

        // 验证范围有效性
        if row_start <= row_end
            && col_start <= col_end
            && row_end <= self.rows
            && col_end <= self.cols
        {
            Some(MatrixView {
                matrix: self,
                row_range: row_start..row_end,
                col_range: col_start..col_end,
            })
        } else {
            None
        }
    }

    /// 创建一个矩阵的可变视图
    ///
    /// 返回指定行范围和列范围的子矩阵可变视图，允许修改原始矩阵的数据。
    /// 如果指定的范围无效或越界，则返回 `None`。
    ///
    /// ## Params
    ///
    /// - `rows`: 行范围，支持 `Range`、`RangeFrom`、`RangeTo`、`RangeFull` 等类型
    /// - `cols`: 列范围，支持 `Range`、`RangeFrom`、`RangeTo`、`RangeFull` 等类型
    ///
    /// ## Examples
    ///
    /// ```
    /// use num::matrix::Matrix;
    ///
    /// let mut m = Matrix::from([[1, 2, 3],
    ///                           [4, 5, 6],
    ///                           [7, 8, 9]]);
    ///
    /// // 获取左上角 2x2 子矩阵的可变视图
    /// let mut view = m.slice_mut(..2, ..2).unwrap();
    /// view[(0, 0)] = 100;  // 修改原始矩阵的第一个元素
    /// view[(1, 1)] = 200;  // 修改原始矩阵的 (1,1) 元素
    ///
    /// assert_eq!(m[(0, 0)], 100);
    /// assert_eq!(m[(1, 1)], 200);
    /// ```
    pub fn slice_mut<R, C>(&mut self, rows: R, cols: C) -> Option<MatrixViewMut<'_, T>>
    where
        R: RangeBounds<usize>,
        C: RangeBounds<usize>,
    {
        let row_start = match rows.start_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        };

        let row_end = match rows.end_bound() {
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => self.rows,
        };

        let col_start = match cols.start_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        };

        let col_end = match cols.end_bound() {
            Bound::Included(&i) => i + 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => self.cols,
        };

        if row_start <= row_end
            && col_start <= col_end
            && row_end <= self.rows
            && col_end <= self.cols
        {
            Some(MatrixViewMut {
                matrix: self,
                row_range: row_start..row_end,
                col_range: col_start..col_end,
            })
        } else {
            None
        }
    }

    /// 获取行切片
    pub fn row(&self, row: usize) -> Option<&[T]> {
        if row < self.rows {
            let start = row * self.cols;
            Some(&self.data[start..start + self.cols])
        } else {
            None
        }
    }

    /// 获取行切片，可变
    pub fn row_mut(&mut self, row: usize) -> Option<&mut [T]> {
        if row < self.rows {
            let start = row * self.cols;
            Some(&mut self.data[start..start + self.cols])
        } else {
            None
        }
    }

    /// 获取列切片
    pub fn col(&self, col: usize) -> Option<impl Iterator<Item = &T> + '_> {
        if col < self.cols {
            Some(self.data.iter().skip(col).step_by(self.cols))
        } else {
            None
        }
    }

    /// 获取列切片，可变
    pub fn col_mut(&mut self, col: usize) -> Option<impl Iterator<Item = &mut T> + '_> {
        if col < self.cols {
            let cols = self.cols;
            Some(self.data.iter_mut().skip(col).step_by(cols))
        } else {
            None
        }
    }

    /// 使用val填充整个矩阵
    pub fn fill(&mut self, val: T) {
        for elem in &mut self.data {
            *elem = val;
        }
    }
}

impl<T: Number> MatrixBase<T> for Matrix<T> {
    #[inline]
    fn rows(&self) -> usize {
        self.rows
    }

    #[inline]
    fn cols(&self) -> usize {
        self.cols
    }

    #[inline]
    fn get(&self, i: usize, j: usize) -> Option<&T> {
        if i >= self.rows || j >= self.cols {
            None
        } else {
            unsafe { Some(self.get_unchecked(i, j)) }
        }
    }
}

impl<T: Number> TryFrom<Vec<Vec<T>>> for Matrix<T> {
    type Error = NumError;

    /// `Vec<Vec<T>>`
    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        Self::from_rows_iter(value)
    }
}

impl<T: Number> TryFrom<&[Vec<T>]> for Matrix<T>
where
    T: Clone,
{
    type Error = NumError;

    /// `&[Vec<T>]`
    fn try_from(value: &[Vec<T>]) -> Result<Self, Self::Error> {
        Self::from_rows_iter(value.iter().map(|row| row.iter().cloned()))
    }
}

impl<T: Number> TryFrom<&[&[T]]> for Matrix<T>
where
    T: Clone,
{
    type Error = NumError;

    /// `&[&[T]]`
    fn try_from(value: &[&[T]]) -> Result<Self, Self::Error> {
        Self::from_rows_iter(value.iter().map(|row| row.iter().cloned()))
    }
}

impl<T: Number> TryFrom<Vec<&[T]>> for Matrix<T>
where
    T: Clone,
{
    type Error = NumError;

    /// `Vec<&[T]>`
    fn try_from(value: Vec<&[T]>) -> Result<Self, Self::Error> {
        Self::from_rows_iter(value.iter().map(|row| row.iter().cloned()))
    }
}

impl<T: Number> TryFrom<Box<[Box<[T]>]>> for Matrix<T>
where
    T: Clone,
{
    type Error = NumError;

    /// `Box<[Box<[T]>]>`
    fn try_from(value: Box<[Box<[T]>]>) -> Result<Self, Self::Error> {
        Self::from_rows_iter(value.iter().map(|row| row.iter().cloned()))
    }
}

impl<T: Number, const R: usize, const C: usize> From<[[T; C]; R]> for Matrix<T> {
    /// 固定大小的数组
    fn from(value: [[T; C]; R]) -> Self {
        Self::from_rows_iter(value).unwrap()
    }
}

impl<T: Number, const C: usize> From<&[[T; C]]> for Matrix<T> {
    /// 固定大小数组的切片
    fn from(value: &[[T; C]]) -> Self {
        Self::from_rows_iter(value.iter().map(|row| row.iter().copied())).unwrap()
    }
}

impl<T: Number, I> TryFrom<(usize, usize, I)> for Matrix<T>
where
    I: IntoIterator<Item = T>,
{
    type Error = NumError;

    /// `(rows, cols, iterator)`
    fn try_from(value: (usize, usize, I)) -> Result<Self, Self::Error> {
        let (rows, cols, iter) = value;
        let data: Vec<T> = iter.into_iter().collect();
        Self::new(rows, cols, data)
    }
}

impl<T: Number> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        if row >= self.rows {
            panic!(
                "Row index {} out of bounds for matrix of size {}x{}",
                row, self.rows, self.cols
            );
        }

        let start = row * self.cols;
        &self.data[start..start + self.cols]
    }
}

impl<T: Number> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        if row >= self.rows {
            panic!(
                "Row index {} out of bounds for matrix of size {}x{}",
                row, self.rows, self.cols
            );
        }

        let start = row * self.cols;
        &mut self.data[start..start + self.cols]
    }
}

impl<T: Number> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        self.get(i, j).unwrap_or_else(|| {
            panic!(
                "Index ({}, {}) out of bounds for matrix of size {}x{}",
                i, j, self.rows, self.cols
            );
        })
    }
}

impl<T: Number> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        let rows = self.rows;
        let cols = self.cols;
        self.get_mut(i, j).unwrap_or_else(|| {
            panic!(
                "Index ({}, {}) out of bounds for matrix of size {}x{}",
                i, j, rows, cols
            );
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_2d_array() {
        // 固定大小数组
        let m: Matrix<i32> = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        assert_eq!(m.rows(), 3);
        assert_eq!(m.cols(), 3);
        assert_eq!(&m.data, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);

        // 空矩阵
        let m: Matrix<i32> = Matrix::from([[]; 0]);
        assert_eq!(m.rows(), 0);
        assert_eq!(m.cols(), 0);
        assert!(m.data.is_empty());
    }

    #[test]
    fn test_from_2d_array_slice() {
        let arr: &[[i32; 3]] = &[[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let m = Matrix::from(arr);
        assert_eq!(m.rows(), 3);
        assert_eq!(m.cols(), 3);
    }

    #[test]
    fn test_try_from_vec_of_vecs() {
        let vecs = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let m: Matrix<i32> = vecs.try_into().unwrap();
        assert_eq!(m.rows(), 3);
        assert_eq!(m.cols(), 3);
        assert_eq!(&m.data, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_try_from_slice_of_vecs() {
        let vecs = vec![vec![1, 2, 3], vec![4, 5, 6]];

        let m: Matrix<i32> = (&vecs[..]).try_into().unwrap();
        assert_eq!(m.rows(), 2);
        assert_eq!(m.cols(), 3);
    }

    #[test]
    fn test_try_from_slice_of_slices() {
        let slices: &[&[i32]] = &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];

        let m: Matrix<i32> = slices.try_into().unwrap();
        assert_eq!(m.rows(), 3);
        assert_eq!(m.cols(), 3);
    }

    #[test]
    fn test_try_from_vec_of_slices() {
        let slices = vec![
            [1, 2, 3].as_slice(),
            [4, 5, 6].as_slice(),
            [7, 8, 9].as_slice(),
        ];

        let m: Matrix<i32> = slices.try_into().unwrap();
        assert_eq!(m.rows(), 3);
        assert_eq!(m.cols(), 3);
    }

    #[test]
    fn test_inconsistent_dimensions() {
        let vecs = vec![
            vec![1, 2, 3],
            vec![4, 5], // 不一致的长度
        ];

        let result: Result<Matrix<i32>, _> = vecs.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn test_tuple_indexing() {
        let mut m = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        // 读取
        assert_eq!(m[(0, 0)], 1);
        assert_eq!(m[(1, 1)], 5);
        assert_eq!(m[(2, 2)], 9);

        // 写入
        m[(1, 2)] = 100;
        assert_eq!(m[(1, 2)], 100);
    }

    #[test]
    fn test_row_slice_indexing() {
        let m = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        // 获取整行切片
        let row0: &[i32] = &m[0];
        assert_eq!(row0, &[1, 2, 3]);

        let row1: &[i32] = &m[1];
        assert_eq!(row1, &[4, 5, 6]);

        // 通过切片访问元素
        assert_eq!(m[0][0], 1);
        assert_eq!(m[1][2], 6);

        // 可变版本
        let mut m = m;
        let row1_mut: &mut [i32] = &mut m[1];
        row1_mut[2] = 100;
        assert_eq!(m[1][2], 100);
        assert_eq!(m[(1, 2)], 100);
    }

    #[test]
    #[should_panic(expected = "Index (3, 2) out of bounds")]
    fn test_index_out_of_bounds_panic() {
        let m = Matrix::from([[1, 2, 3], [4, 5, 6]]);
        let _ = m[(3, 2)];
    }

    #[test]
    #[should_panic(expected = "Row index 3 out of bounds")]
    fn test_row_index_out_of_bounds_panic() {
        let m = Matrix::from([[1, 2, 3], [4, 5, 6]]);
        let _ = m[3];
    }

    #[test]
    fn test_safe_get() {
        let m = Matrix::from([[1, 2, 3], [4, 5, 6]]);

        // 安全的 get 方法返回 Option
        assert_eq!(m.get(0, 0), Some(&1));
        assert_eq!(m.get(2, 0), None); // 行越界
        assert_eq!(m.get(0, 3), None); // 列越界

        // 使用 get 避免 panic
        if let Some(value) = m.get(1, 2) {
            assert_eq!(*value, 6);
        }
    }

    #[test]
    fn test_safe_get_mut() {
        let mut m = Matrix::from([[1, 2, 3], [4, 5, 6]]);

        m.get_mut(0, 0).map(|val| *val = 100);
        assert_eq!(m[0][0], 100);
    }

    #[test]
    fn test_row_col_slice() {
        let mut m = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        let row2 = m.row(1).unwrap();
        assert_eq!(row2, &[4, 5, 6]);

        let col3: Vec<_> = m.col(2).unwrap().copied().collect();
        assert_eq!(col3, vec![3, 6, 9]);

        let row1_mut = m.row_mut(0).unwrap();
        row1_mut[1] = 20;
        assert_eq!(m[(0, 1)], 20);
    }
}
