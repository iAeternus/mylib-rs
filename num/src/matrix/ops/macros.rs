use std::ops::{Add, Mul, Sub};

use crate::matrix::elementwise::{matrix_add, matrix_mul, matrix_scalar_mul, matrix_sub};
use crate::{
    Number, Scalar,
    error::NumResult,
    matrix::{Matrix, MatrixView, MatrixViewMut},
};

macro_rules! impl_bin_op_matrix_base {
    ($trait:ident, $method:ident, $func:ident, $lhs:ty, $rhs:ty) => {
        impl<T: Number> $trait<&$rhs> for &$lhs {
            type Output = Matrix<T>;

            fn $method(self, rhs: &$rhs) -> Self::Output {
                $func(self, rhs).unwrap_or_else(|e| panic!("{}", e))
            }
        }

        impl<T: Number> $trait<$rhs> for &$lhs {
            type Output = Matrix<T>;

            fn $method(self, rhs: $rhs) -> Self::Output {
                self.$method(&rhs)
            }
        }

        impl<T: Number> $trait<&$rhs> for $lhs {
            type Output = Matrix<T>;

            fn $method(self, rhs: &$rhs) -> Self::Output {
                (&self).$method(rhs)
            }
        }

        impl<T: Number> $trait<$rhs> for $lhs {
            type Output = Matrix<T>;

            fn $method(self, rhs: $rhs) -> Self::Output {
                (&self).$method(&rhs)
            }
        }
    };
}

macro_rules! impl_bin_op_matrix_all {
    ($trait:ident, $method:ident, $func:ident) => {
        impl_bin_op_matrix_base!($trait, $method, $func, Matrix<T>, Matrix<T>);
        impl_bin_op_matrix_base!($trait, $method, $func, Matrix<T>, MatrixView<'_, T>);
        impl_bin_op_matrix_base!($trait, $method, $func, Matrix<T>, MatrixViewMut<'_, T>);

        impl_bin_op_matrix_base!($trait, $method, $func, MatrixView<'_, T>, Matrix<T>);
        impl_bin_op_matrix_base!($trait, $method, $func, MatrixView<'_, T>, MatrixView<'_, T>);
        impl_bin_op_matrix_base!(
            $trait,
            $method,
            $func,
            MatrixView<'_, T>,
            MatrixViewMut<'_, T>
        );

        impl_bin_op_matrix_base!($trait, $method, $func, MatrixViewMut<'_, T>, Matrix<T>);
        impl_bin_op_matrix_base!(
            $trait,
            $method,
            $func,
            MatrixViewMut<'_, T>,
            MatrixView<'_, T>
        );
        impl_bin_op_matrix_base!(
            $trait,
            $method,
            $func,
            MatrixViewMut<'_, T>,
            MatrixViewMut<'_, T>
        );
    };
}

impl_bin_op_matrix_all!(Add, add, matrix_add);
impl_bin_op_matrix_all!(Sub, sub, matrix_sub);

macro_rules! impl_matrix_scalar_mul {
    ($Mat:ty) => {
        impl<T: Number> Mul<T> for &$Mat {
            type Output = Matrix<T>;

            #[inline]
            fn mul(self, rhs: T) -> Self::Output {
                matrix_scalar_mul(self, rhs)
            }
        }

        impl<T: Number> Mul<T> for $Mat {
            type Output = Matrix<T>;

            #[inline]
            fn mul(self, rhs: T) -> Self::Output {
                matrix_scalar_mul(&self, rhs)
            }
        }

        impl<T: Number> Mul<&$Mat> for Scalar<T> {
            type Output = Matrix<T>;

            #[inline]
            fn mul(self, rhs: &$Mat) -> Self::Output {
                matrix_scalar_mul(rhs, self.0)
            }
        }

        impl<T: Number> Mul<$Mat> for Scalar<T> {
            type Output = Matrix<T>;

            #[inline]
            fn mul(self, rhs: $Mat) -> Self::Output {
                matrix_scalar_mul(&rhs, self.0)
            }
        }
    };
}

impl_matrix_scalar_mul!(Matrix<T>);
impl_matrix_scalar_mul!(MatrixView<'_, T>);
impl_matrix_scalar_mul!(MatrixViewMut<'_, T>);

macro_rules! impl_matrix_mul {
    ($Lhs:ty, $Rhs:ty) => {
        impl<T: Number> Mul<&$Rhs> for &$Lhs {
            type Output = NumResult<Matrix<T>>;

            #[inline]
            fn mul(self, rhs: &$Rhs) -> Self::Output {
                matrix_mul(self, rhs)
            }
        }

        impl<T: Number> Mul<$Rhs> for &$Lhs {
            type Output = NumResult<Matrix<T>>;

            #[inline]
            fn mul(self, rhs: $Rhs) -> Self::Output {
                matrix_mul(self, &rhs)
            }
        }

        impl<T: Number> Mul<&$Rhs> for $Lhs {
            type Output = NumResult<Matrix<T>>;

            #[inline]
            fn mul(self, rhs: &$Rhs) -> Self::Output {
                matrix_mul(&self, rhs)
            }
        }

        impl<T: Number> Mul<$Rhs> for $Lhs {
            type Output = NumResult<Matrix<T>>;

            #[inline]
            fn mul(self, rhs: $Rhs) -> Self::Output {
                matrix_mul(&self, &rhs)
            }
        }
    };
}

impl_matrix_mul!(Matrix<T>, Matrix<T>);
impl_matrix_mul!(Matrix<T>, MatrixView<'_, T>);
impl_matrix_mul!(Matrix<T>, MatrixViewMut<'_, T>);

impl_matrix_mul!(MatrixView<'_, T>, Matrix<T>);
impl_matrix_mul!(MatrixView<'_, T>, MatrixView<'_, T>);
impl_matrix_mul!(MatrixView<'_, T>, MatrixViewMut<'_, T>);

impl_matrix_mul!(MatrixViewMut<'_, T>, Matrix<T>);
impl_matrix_mul!(MatrixViewMut<'_, T>, MatrixView<'_, T>);
impl_matrix_mul!(MatrixViewMut<'_, T>, MatrixViewMut<'_, T>);
