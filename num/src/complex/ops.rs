use std::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{
    complex::Complex,
    core::{Number, Signed, one::One, zero::Zero},
};

impl<T: Number + Signed> Neg for Complex<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.re, -self.im)
    }
}

impl<T: Number> Add for Complex<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl<T: Number> Sub for Complex<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl<T: Number> Mul for Complex<T> {
    type Output = Self;

    /// 复数乘复数
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl<T: Number> Mul<T> for Complex<T> {
    type Output = Self;

    /// 复数乘标量
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.re * rhs, self.im * rhs)
    }
}

// macro_rules! impl_mul {
//     ($($t:ty),+ $(,)?) => {
//         $(
//             impl<T: Number> Mul<Complex<T>> for $t {
//                 type Output = Complex<$t>;

//                 fn mul(self, rhs: Complex<T>) -> Self::Output {
//                     Complex::new(self * rhs.re, self * rhs.im)
//                 }
//             }
//         )+
//     };
// }

// impl_mul!(i8, i16, i32, i64, i128);

impl<T: Number> Div for Complex<T> {
    type Output = Self;

    /// 不推荐对整数类型调用除法，可能发生截断
    fn div(self, rhs: Self) -> Self::Output {
        let d = rhs.norm_sq();
        assert!(!d.is_zero(), "division by zero");
        Self::new(
            (self.re * rhs.re + self.im * rhs.im) / d,
            (self.im * rhs.re - self.re * rhs.im) / d,
        )
    }
}

impl<T: Number> AddAssign for Complex<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T: Number> SubAssign for Complex<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

impl<T: Number> MulAssign for Complex<T> {
    /// 复数乘复数
    fn mul_assign(&mut self, rhs: Self) {
        let re = self.re * rhs.re - self.im * rhs.im;
        let im = self.re * rhs.im + self.im * rhs.re;
        self.re = re;
        self.im = im;
    }
}

impl<T: Number> MulAssign<T> for Complex<T> {
    /// 复数乘标量
    fn mul_assign(&mut self, rhs: T) {
        self.re *= rhs;
        self.im *= rhs;
    }
}

impl<T: Number> DivAssign for Complex<T> {
    /// 不推荐对整数类型调用除法，可能发生截断
    fn div_assign(&mut self, rhs: Self) {
        let d = rhs.norm_sq();
        assert!(!d.is_zero(), "division by zero");
        let re = (self.re * rhs.re + self.im * rhs.im) / d;
        let im = (self.im * rhs.re - self.re * rhs.im) / d;
        self.re = re;
        self.im = im;
    }
}

impl<T: Number> Sum for Complex<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + b)
    }
}

impl<T: Number> Product for Complex<T> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), |a, b| a * b)
    }
}

#[cfg(test)]
mod tests {
    use core::f64;

    use super::*;
    use crate::{complex::Complex, core::ApproxEq};

    #[test]
    fn test_neg() {
        let z = Complex::new(1, -2);
        let w = -z;
        assert_eq!(w.re, -1);
        assert_eq!(w.im, 2);
    }

    #[test]
    fn test_add() {
        let a = Complex::new(1, 2);
        let b = Complex::new(3, -1);
        let c = a + b;
        assert_eq!(c.re, 4);
        assert_eq!(c.im, 1);
    }

    #[test]
    fn test_sub() {
        let a = Complex::new(5, 3);
        let b = Complex::new(2, 1);
        let c = a - b;
        assert_eq!(c.re, 3);
        assert_eq!(c.im, 2);
    }

    #[test]
    fn test_mul() {
        // (1 + 2i)(3 + 4i) = -5 + 10i
        let a = Complex::new(1, 2);
        let b = Complex::new(3, 4);
        let c = a * b;
        assert_eq!(c.re, -5);
        assert_eq!(c.im, 10);
    }

    #[test]
    fn test_div() {
        let a = Complex::new(1.0, 1.0);
        let b = Complex::new(1.0, -1.0);
        let c = a / b;
        println!("{}", c);
        let expected = Complex::new(0.0, 1.0);
        assert!(c.approx_eq(&expected, f64::EPSILON));
    }

    #[test]
    fn zero_is_additive_identity() {
        let z = Complex::new(3, -4);
        assert_eq!(z + Complex::zero(), z);
        assert_eq!(Complex::zero() + z, z);
    }

    #[test]
    fn one_is_multiplicative_identity() {
        let z = Complex::new(-2, 5);
        assert_eq!(z * Complex::one(), z);
        assert_eq!(Complex::one() as Complex<i32> * z, z);
    }

    #[test]
    fn zero_mul_is_zero() {
        let z = Complex::new(7, -3);
        assert_eq!(z * Complex::zero(), Complex::zero());
    }

    #[test]
    fn test_add_assign() {
        let mut z = Complex::new(1, 1);
        z += Complex::new(2, -3);
        assert_eq!(z.re, 3);
        assert_eq!(z.im, -2);
    }

    #[test]
    fn test_sub_assign() {
        let mut z = Complex::new(5, 4);
        z -= Complex::new(1, 2);
        assert_eq!(z.re, 4);
        assert_eq!(z.im, 2);
    }

    #[test]
    fn test_test_mul_assign() {
        let mut z = Complex::new(1, 2);
        z *= Complex::new(3, 4);
        assert_eq!(Complex::new(-5, 10), z);
    }

    #[test]
    fn test_sum() {
        let v = vec![Complex::new(1, 1), Complex::new(2, -1), Complex::new(-3, 0)];
        let s: Complex<i32> = v.into_iter().sum();
        assert_eq!(s.re, 0);
        assert_eq!(s.im, 0);
    }

    #[test]
    fn test_product() {
        let v = vec![Complex::new(1, 1), Complex::new(1, -1)];
        let p: Complex<i32> = v.into_iter().product();
        // (1+i)(1-i) = 2
        assert_eq!(p.re, 2);
        assert_eq!(p.im, 0);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn div_by_zero_panics() {
        let _ = Complex::new(1.0, 1.0) / Complex::zero();
    }
}
