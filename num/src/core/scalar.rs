use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::{Number, One, Zero};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Scalar<T: Number>(pub T);

impl<T: Number> Scalar<T> {
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Number> Zero for Scalar<T> {
    #[inline]
    fn zero() -> Self {
        Scalar(T::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T: Number> One for Scalar<T> {
    #[inline]
    fn one() -> Self {
        Scalar(T::one())
    }

    #[inline]
    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

impl<T: Number> Add for Scalar<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Scalar(self.0 + rhs.0)
    }
}

impl<T: Number> Sub for Scalar<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Scalar(self.0 - rhs.0)
    }
}

impl<T: Number> Mul for Scalar<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Scalar(self.0 * rhs.0)
    }
}

impl<T: Number> Div for Scalar<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Scalar(self.0 / rhs.0)
    }
}

impl<T: Number> From<T> for Scalar<T> {
    #[inline]
    fn from(value: T) -> Self {
        Scalar(value)
    }
}

impl<T: Number> AddAssign for Scalar<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl<T: Number> SubAssign for Scalar<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl<T: Number> MulAssign for Scalar<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl<T: Number> DivAssign for Scalar<T> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}
