use std::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{
    complex::Complex,
    core::{Float, Norm, one::One, zero::Zero},
};

impl<T: Float> Neg for Complex<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.re, -self.im)
    }
}

impl<T: Float> Add for Complex<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl<T: Float> Sub for Complex<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl<T: Float> Mul for Complex<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl<T: Float> Div for Complex<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let d = rhs.norm_sq();
        assert!(!d.is_zero(), "division by zero");
        Self::new(
            (self.re * rhs.re + self.im * rhs.im) / d,
            (self.im * rhs.re - self.re * rhs.im) / d,
        )
    }
}

impl<T: Float> AddAssign for Complex<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T: Float> SubAssign for Complex<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

impl<T: Float> MulAssign for Complex<T> {
    fn mul_assign(&mut self, rhs: Self) {
        let re = self.re * rhs.re - self.im * rhs.im;
        let im = self.re * rhs.im + self.im * rhs.re;
        self.re = re;
        self.im = im;
    }
}

impl<T: Float> DivAssign for Complex<T> {
    fn div_assign(&mut self, rhs: Self) {
        let d = rhs.norm();
        assert!(!d.is_zero(), "division by zero");
        let re = (self.re * rhs.re + self.im * rhs.im) / d;
        let im = (self.im * rhs.re - self.re * rhs.im) / d;
        self.re = re;
        self.im = im;
    }
}

impl<T: Float> Sum for Complex<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + b)
    }
}

impl<T: Float> Product for Complex<T> {
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
        let z = Complex::new(1.0, -2.0);
        let w = -z;
        assert_eq!(w.re, -1.0);
        assert_eq!(w.im, 2.0);
    }

    #[test]
    fn test_add() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, -1.0);
        let c = a + b;
        assert_eq!(c.re, 4.0);
        assert_eq!(c.im, 1.0);
    }

    #[test]
    fn test_sub() {
        let a = Complex::new(5.0, 3.0);
        let b = Complex::new(2.0, 1.0);
        let c = a - b;
        assert_eq!(c.re, 3.0);
        assert_eq!(c.im, 2.0);
    }

    #[test]
    fn test_mul() {
        // (1 + 2i)(3 + 4i) = -5 + 10i
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = a * b;
        assert_eq!(c.re, -5.0);
        assert_eq!(c.im, 10.0);
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
        let z = Complex::new(3.0, -4.0);
        assert_eq!(z + Complex::zero(), z);
        assert_eq!(Complex::zero() + z, z);
    }

    #[test]
    fn one_is_multiplicative_identity() {
        let z = Complex::new(-2.0, 5.0);
        assert_eq!(z * Complex::one(), z);
        assert_eq!(Complex::one() * z, z);
    }

    #[test]
    fn zero_mul_is_zero() {
        let z = Complex::new(7.0, -3.0);
        assert_eq!(z * Complex::zero(), Complex::zero());
    }

    #[test]
    fn test_add_assign() {
        let mut z = Complex::new(1.0, 1.0);
        z += Complex::new(2.0, -3.0);
        assert_eq!(z.re, 3.0);
        assert_eq!(z.im, -2.0);
    }

    #[test]
    fn test_sub_assign() {
        let mut z = Complex::new(5.0, 4.0);
        z -= Complex::new(1.0, 2.0);
        assert_eq!(z.re, 4.0);
        assert_eq!(z.im, 2.0);
    }

    #[test]
    fn test_test_mul_assign() {
        let mut z = Complex::new(1.0, 2.0);
        z *= Complex::new(3.0, 4.0);
        let expected = Complex::new(-5.0, 10.0);
        assert!(z.approx_eq(&expected, f64::EPSILON));
    }

    #[test]
    fn test_sum() {
        let v = vec![
            Complex::new(1.0, 1.0),
            Complex::new(2.0, -1.0),
            Complex::new(-3.0, 0.0),
        ];
        let s: Complex<f64> = v.into_iter().sum();
        assert_eq!(s.re, 0.0);
        assert_eq!(s.im, 0.0);
    }

    #[test]
    fn test_product() {
        let v = vec![Complex::new(1.0, 1.0), Complex::new(1.0, -1.0)];
        let p: Complex<f64> = v.into_iter().product();
        // (1+i)(1-i) = 2
        assert_eq!(p.re, 2.0);
        assert_eq!(p.im, 0.0);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn div_by_zero_panics() {
        let _ = Complex::new(1.0, 1.0) / Complex::zero();
    }
}
