use std::{fmt::Display, str::FromStr};

use crate::{
    core::{ApproxEq, Float, Norm, Number, One, Signed, Zero},
    error::NumError,
};

/// 复数语义
pub trait ComplexNumber: Number {
    type Scalar: Number;

    /// 获取实部
    fn re(&self) -> Self::Scalar;

    /// 获取虚部
    fn im(&self) -> Self::Scalar;

    /// 返回复数的共轭
    fn conj(self) -> Self;
}

/// 复数
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Complex<T: Number> {
    /// 实部
    pub(crate) re: T,
    /// 虚部
    pub(crate) im: T,
}

impl<T: Number> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Self { re, im }
    }

    fn scale(&self, n: T) -> Self {
        Self::new(self.re * n, self.im * n)
    }

    pub fn norm_sq(&self) -> T {
        self.re * self.re + self.im * self.im
    }
}

impl<T: Float> Complex<T> {
    /// 返回复数的幅角 (radians)
    pub fn arg(&self) -> T {
        self.im.atan2(self.re)
    }

    /// 返回单位化复数
    pub fn unit(&self) -> Self {
        let n = self.norm();
        if n.is_zero() {
            Self::zero()
        } else {
            Self::new(self.re / n, self.im / n)
        }
    }

    /// 返回复数的指数
    pub fn exp(&self) -> Self {
        let exp_re = self.re.exp();
        Self::new(exp_re * self.im.cos(), exp_re * self.im.sin())
    }

    /// 返回复数的自然对数
    pub fn ln(&self) -> Self {
        Self::new(self.norm().ln(), self.arg())
    }

    /// 返回复数的幂
    pub fn powf(&self, n: T) -> Self {
        self.ln().scale(n).exp()
    }
}

impl<T: Number + Signed> ComplexNumber for Complex<T> {
    type Scalar = T;

    fn re(&self) -> Self::Scalar {
        self.re
    }

    fn im(&self) -> Self::Scalar {
        self.im
    }

    fn conj(self) -> Self {
        Self::new(self.re, -self.im)
    }
}

impl<T: Float> Norm for Complex<T> {
    type Output = T;

    fn norm(&self) -> Self::Output {
        self.norm_sq().sqrt()
    }

    fn norm_sq(&self) -> Self::Output {
        self.re * self.re + self.im * self.im
    }
}

impl<T: Number> Zero for Complex<T> {
    fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.re.is_zero() && self.im.is_zero()
    }
}

impl<T: Number> One for Complex<T> {
    fn one() -> Self {
        Self::new(T::one(), T::zero())
    }

    fn is_one(&self) -> bool {
        self.re.is_one() && self.im.is_zero()
    }
}

impl<T: Number> Default for Complex<T> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<T: Number> From<T> for Complex<T> {
    fn from(value: T) -> Self {
        Self::new(value, T::zero())
    }
}

impl<T: Number> From<(T, T)> for Complex<T> {
    fn from((re, im): (T, T)) -> Self {
        Self::new(re, im)
    }
}

impl<T: Float + ApproxEq> ApproxEq for Complex<T> {
    fn approx_eq(&self, rhs: &Self, eps: f64) -> bool {
        self.re.approx_eq(&rhs.re, eps) && self.im.approx_eq(&rhs.im, eps)
    }
}

impl<T: Number> FromStr for Complex<T> {
    type Err = NumError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl<T: Display + Number + Signed> Display for Complex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let re = self.re();
        let im = self.im();

        // 虚部为0
        if im.is_zero() {
            return write!(f, "{}", re);
        }

        // 实部为0
        if re.is_zero() {
            if im.is_one() {
                return write!(f, "i");
            } else {
                return write!(f, "{}i", im);
            }
        }

        // 实部和虚部都不为0
        if im.is_negative() {
            // 虚部为负数
            let abs_im = -im;
            if abs_im.is_one() {
                write!(f, "{}-i", re)
            } else {
                write!(f, "{}-{}i", re, abs_im)
            }
        } else {
            // 虚部为正数
            if im.is_one() {
                write!(f, "{}+i", re)
            } else {
                write!(f, "{}+{}i", re, im)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use core::f64;

    use crate::core::ApproxEq;

    use super::*;

    #[test]
    fn test_conj() {
        let a = Complex::new(3, 4);

        let conj = a.conj();

        assert_eq!(Complex::new(3, -4), conj);
    }

    #[test]
    fn test_norm() {
        let a = Complex::new(3., 4.);

        let norm = a.norm();
        let norm_sq = a.norm_sq();

        assert_eq!(true, 5_f64.approx_eq(&norm, f64::EPSILON));
        assert_eq!(true, 25_f64.approx_eq(&norm_sq, f64::EPSILON));
    }

    #[test]
    fn test_norm_sq() {
        let a = Complex::new(3, 4);
        let b = Complex::new(3., 4.);

        let norm_sq_a = a.norm_sq();
        let norm_sq_b = b.norm_sq();

        assert_eq!(25, norm_sq_a);
        assert_eq!(true, 25_f64.approx_eq(&norm_sq_b, f64::EPSILON));
    }

    #[test]
    fn test_fmt() {
        let a = Complex::new(1, 2);
        let b = Complex::new(1, -2);
        let c = Complex::from(1);
        let d = Complex::new(0, 1);
        let e = Complex::new(0, 2);

        println!("{}", a); // 1+2i
        println!("{}", b); // 1-2i
        println!("{}", c); // 1
        println!("{}", d); // i
        println!("{}", e); // 2i
    }
}
