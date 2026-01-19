use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumError {
    /// 除以零
    DivisionByZero,

    /// 非法参数
    InvalidArgument(&'static str),

    /// 溢出
    Overflow,

    /// 数学上未定义的操作
    DomainError(&'static str),

    /// 矩阵大小错误
    MatrixSizeMismatch { expect: usize, actual: usize },

    /// 矩阵维度错误
    MatrixDimensionMismatch {
        expect: (usize, usize),
        actual: (usize, usize),
    },

    /// 下标越界
    IndexOutOfBounds,

    /// 非方阵操作
    NotSquareMatrix { rows: usize, cols: usize },

    /// 高精度整数转换错误
    ParseBigIntError,
}

pub type NumResult<T> = core::result::Result<T, NumError>;

impl std::error::Error for NumError {}

impl fmt::Display for NumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumError::DivisionByZero => write!(f, "division by zero"),
            NumError::InvalidArgument(msg) => write!(f, "invalid argument: {}", msg),
            NumError::Overflow => write!(f, "arithmetic overflow"),
            NumError::DomainError(msg) => write!(f, "domain error: {}", msg),
            NumError::IndexOutOfBounds => write!(f, "index out of bounds"),
            NumError::MatrixSizeMismatch { expect, actual } => {
                write!(
                    f,
                    "matrix size mismatch: expect size {}, actual size {}",
                    expect, actual
                )
            }
            NumError::MatrixDimensionMismatch {
                expect: (e_rows, e_cols),
                actual: (a_rows, a_cols),
            } => {
                write!(
                    f,
                    "matrix dimension mismatch: expect matrix ({}x{}), actual matrix ({}x{})",
                    e_rows, e_cols, a_rows, a_cols
                )
            }
            NumError::NotSquareMatrix { rows, cols } => {
                write!(f, "matrix is not square ({}x{})", rows, cols)
            }
            NumError::ParseBigIntError => write!(f, "parse big int error"),
        }
    }
}
