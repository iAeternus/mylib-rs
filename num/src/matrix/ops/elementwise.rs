use crate::{
    Number,
    error::{NumError, NumResult},
    matrix::{Matrix, MatrixBase},
};

/// 矩阵加法
pub(crate) fn matrix_add<A, B, T>(lhs: &A, rhs: &B) -> NumResult<Matrix<T>>
where
    T: Number,
    A: MatrixBase<T>,
    B: MatrixBase<T>,
{
    elementwise_op(lhs, rhs, |x, y| x + y)
}

/// 矩阵减法
pub(crate) fn matrix_sub<A, B, T>(lhs: &A, rhs: &B) -> NumResult<Matrix<T>>
where
    T: Number,
    A: MatrixBase<T>,
    B: MatrixBase<T>,
{
    elementwise_op(lhs, rhs, |x, y| x - y)
}

fn elementwise_op<A, B, T, F>(lhs: &A, rhs: &B, op: F) -> NumResult<Matrix<T>>
where
    T: Number,
    A: MatrixBase<T>,
    B: MatrixBase<T>,
    F: Fn(T, T) -> T,
{
    if lhs.rows() != rhs.rows() || lhs.cols() != rhs.cols() {
        return Err(NumError::MatrixShapeMismatch {
            expect: (lhs.rows(), lhs.cols()),
            actual: (rhs.rows(), rhs.cols()),
        });
    }

    let rows = lhs.rows();
    let cols = lhs.cols();
    let len = rows * cols;

    let mut data = Vec::with_capacity(len);

    unsafe {
        data.set_len(len);

        let mut idx = 0;
        for i in 0..rows {
            for j in 0..cols {
                *data.get_unchecked_mut(idx) =
                    op(*lhs.get_unchecked(i, j), *rhs.get_unchecked(i, j));
                idx += 1;
            }
        }

        Ok(Matrix::new_unchecked(rows, cols, data))
    }
}

/// 矩阵数乘
#[inline]
pub(crate) fn matrix_scalar_mul<M, T>(matrix: &M, scalar: T) -> Matrix<T>
where
    T: Number,
    M: MatrixBase<T>,
{
    let rows = matrix.rows();
    let cols = matrix.cols();
    let len = rows * cols;

    let mut data = Vec::with_capacity(len);

    unsafe {
        data.set_len(len);

        let mut idx = 0;
        for i in 0..rows {
            for j in 0..cols {
                *data.get_unchecked_mut(idx) = *matrix.get_unchecked(i, j) * scalar;
                idx += 1;
            }
        }

        Matrix::new_unchecked(rows, cols, data)
    }
}

/// 矩阵乘法
#[inline]
pub(crate) fn matrix_mul<A, B, T>(lhs: &A, rhs: &B) -> NumResult<Matrix<T>>
where
    T: Number,
    A: MatrixBase<T>,
    B: MatrixBase<T>,
{
    let lhs_rows = lhs.rows();
    let lhs_cols = lhs.cols();
    let rhs_cols = rhs.cols();

    if lhs_cols != rhs.rows() {
        return Err(NumError::MatrixCannotMul {
            lhs_col: lhs_cols,
            rhs_row: rhs.rows(),
        });
    }

    let mut data = vec![T::zero(); lhs_rows * rhs_cols];

    unsafe {
        for i in 0..lhs_rows {
            for k in 0..lhs_cols {
                let a = *lhs.get_unchecked(i, k);
                for j in 0..rhs_cols {
                    let b = *rhs.get_unchecked(k, j);
                    *data.get_unchecked_mut(i * rhs_cols + j) += a * b;
                }
            }
        }

        Ok(Matrix::new_unchecked(lhs_rows, rhs_cols, data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::Matrix;

    #[test]
    fn test_matrix_add_ok() {
        let a = mat_i32(2, 2, &[1, 2, 3, 4]);
        let b = mat_i32(2, 2, &[5, 6, 7, 8]);

        let c = matrix_add(&a, &b).unwrap();

        assert_eq!(c.rows(), 2);
        assert_eq!(c.cols(), 2);

        unsafe {
            assert_eq!(*c.get_unchecked(0, 0), 6);
            assert_eq!(*c.get_unchecked(0, 1), 8);
            assert_eq!(*c.get_unchecked(1, 0), 10);
            assert_eq!(*c.get_unchecked(1, 1), 12);
        }
    }

    #[test]
    fn test_matrix_add_shape_mismatch() {
        let a = mat_i32(2, 2, &[1, 2, 3, 4]);
        let b = mat_i32(2, 3, &[1, 2, 3, 4, 5, 6]);

        let err = matrix_add(&a, &b).unwrap_err();

        match err {
            NumError::MatrixShapeMismatch { expect, actual } => {
                assert_eq!(expect, (2, 2));
                assert_eq!(actual, (2, 3));
            }
            _ => panic!("unexpected error type"),
        }
    }

    #[test]
    fn test_matrix_sub_ok() {
        let a = mat_i32(2, 2, &[5, 6, 7, 8]);
        let b = mat_i32(2, 2, &[1, 2, 3, 4]);

        let c = matrix_sub(&a, &b).unwrap();

        unsafe {
            assert_eq!(*c.get_unchecked(0, 0), 4);
            assert_eq!(*c.get_unchecked(0, 1), 4);
            assert_eq!(*c.get_unchecked(1, 0), 4);
            assert_eq!(*c.get_unchecked(1, 1), 4);
        }
    }

    #[test]
    fn test_matrix_scalar_mul() {
        let a = mat_i32(2, 3, &[1, 2, 3, 4, 5, 6]);

        let b = matrix_scalar_mul(&a, 3);

        assert_eq!(b.rows(), 2);
        assert_eq!(b.cols(), 3);

        unsafe {
            for i in 0..2 {
                for j in 0..3 {
                    assert_eq!(*b.get_unchecked(i, j), *a.get_unchecked(i, j) * 3);
                }
            }
        }
    }

    #[test]
    fn test_matrix_mul_ok() {
        // 2x3 * 3x2
        let a = mat_i32(2, 3, &[1, 2, 3, 4, 5, 6]);
        let b = mat_i32(3, 2, &[7, 8, 9, 10, 11, 12]);

        let c = matrix_mul(&a, &b).unwrap();

        assert_eq!(c.rows(), 2);
        assert_eq!(c.cols(), 2);

        unsafe {
            assert_eq!(*c.get_unchecked(0, 0), 58); // 1*7 + 2*9 + 3*11
            assert_eq!(*c.get_unchecked(0, 1), 64); // 1*8 + 2*10 + 3*12
            assert_eq!(*c.get_unchecked(1, 0), 139); // 4*7 + 5*9 + 6*11
            assert_eq!(*c.get_unchecked(1, 1), 154);
        }
    }

    #[test]
    fn test_matrix_mul_invalid_shape() {
        let a = mat_i32(2, 3, &[1, 2, 3, 4, 5, 6]);
        let b = mat_i32(4, 2, &[1, 2, 3, 4, 5, 6, 7, 8]);

        let err = matrix_mul(&a, &b).unwrap_err();

        match err {
            NumError::MatrixCannotMul { lhs_col, rhs_row } => {
                assert_eq!(lhs_col, 3);
                assert_eq!(rhs_row, 4);
            }
            _ => panic!("unexpected error type"),
        }
    }

    #[test]
    fn test_matrix_mul_f64() {
        let a = mat_f64(1, 2, &[1.5, 2.0]);
        let b = mat_f64(2, 1, &[2.0, 4.0]);

        let c = matrix_mul(&a, &b).unwrap();

        unsafe {
            assert!((*c.get_unchecked(0, 0) - 11.0).abs() < 1e-12);
        }
    }

    fn mat_i32(rows: usize, cols: usize, data: &[i32]) -> Matrix<i32> {
        assert_eq!(rows * cols, data.len());
        unsafe { Matrix::new_unchecked(rows, cols, data.to_vec()) }
    }

    fn mat_f64(rows: usize, cols: usize, data: &[f64]) -> Matrix<f64> {
        assert_eq!(rows * cols, data.len());
        unsafe { Matrix::new_unchecked(rows, cols, data.to_vec()) }
    }
}
