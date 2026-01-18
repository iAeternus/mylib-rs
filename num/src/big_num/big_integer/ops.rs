use std::{
    cmp::Ordering,
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

use crate::{
    Zero,
    big_num::big_integer::big_integer::{BigInteger, Sign},
    core::one::One,
};

impl<'a> Neg for &'a BigInteger {
    type Output = BigInteger;

    fn neg(self) -> BigInteger {
        if self.is_zero() {
            return self.clone();
        }

        let mut r = self.clone();
        r.sign = match r.sign {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        };
        r
    }
}

impl Neg for BigInteger {
    type Output = BigInteger;

    fn neg(self) -> BigInteger {
        -&self
    }
}

impl<'a, 'b> Add<&'b BigInteger> for &'a BigInteger {
    type Output = BigInteger;

    fn add(self, rhs: &'b BigInteger) -> BigInteger {
        match (self.sign, rhs.sign) {
            (Sign::Positive, Sign::Positive) => {
                let mut r = BigInteger::abs_add(self, rhs);
                r.sign = Sign::Positive;
                r
            }
            (Sign::Negative, Sign::Negative) => {
                let mut r = BigInteger::abs_add(self, rhs);
                r.sign = Sign::Negative;
                r
            }
            _ => self - &(-rhs),
        }
    }
}

impl Add for BigInteger {
    type Output = BigInteger;

    fn add(self, rhs: BigInteger) -> BigInteger {
        &self + &rhs
    }
}

impl<'a> Add<&'a BigInteger> for BigInteger {
    type Output = BigInteger;

    fn add(self, rhs: &'a BigInteger) -> BigInteger {
        &self + rhs
    }
}

impl<'a> Add<BigInteger> for &'a BigInteger {
    type Output = BigInteger;

    fn add(self, rhs: BigInteger) -> BigInteger {
        self + &rhs
    }
}

impl<'a, 'b> Sub<&'b BigInteger> for &'a BigInteger {
    type Output = BigInteger;

    fn sub(self, rhs: &'b BigInteger) -> BigInteger {
        match (self.sign, rhs.sign) {
            (Sign::Positive, Sign::Negative) => self + rhs.abs(),
            (Sign::Negative, Sign::Positive) => -(&self.abs() + rhs),
            _ => match self.abs_cmp(rhs) {
                Ordering::Greater | Ordering::Equal => {
                    let mut r = BigInteger::abs_sub(self, rhs);
                    r.sign = self.sign;
                    r
                }
                Ordering::Less => {
                    let mut r = BigInteger::abs_sub(rhs, self);
                    r.sign = if self.sign == Sign::Positive {
                        Sign::Negative
                    } else {
                        Sign::Positive
                    };
                    r
                }
            },
        }
    }
}

impl Sub for BigInteger {
    type Output = BigInteger;

    fn sub(self, rhs: BigInteger) -> BigInteger {
        &self - &rhs
    }
}

impl<'a> Sub<&'a BigInteger> for BigInteger {
    type Output = BigInteger;

    fn sub(self, rhs: &'a BigInteger) -> BigInteger {
        &self - rhs
    }
}

impl<'a> Sub<BigInteger> for &'a BigInteger {
    type Output = BigInteger;

    fn sub(self, rhs: BigInteger) -> BigInteger {
        self - &rhs
    }
}

impl<'a, 'b> Mul<&'b BigInteger> for &'a BigInteger {
    type Output = BigInteger;

    fn mul(self, _rhs: &'b BigInteger) -> BigInteger {
        todo!("naive / Karatsuba / FFT to be implemented")
    }
}

impl Mul for BigInteger {
    type Output = BigInteger;

    fn mul(self, rhs: BigInteger) -> BigInteger {
        &self * &rhs
    }
}

impl<'a> Mul<&'a BigInteger> for BigInteger {
    type Output = BigInteger;

    fn mul(self, rhs: &'a BigInteger) -> BigInteger {
        &self * rhs
    }
}

impl<'a> Mul<BigInteger> for &'a BigInteger {
    type Output = BigInteger;

    fn mul(self, rhs: BigInteger) -> BigInteger {
        self * &rhs
    }
}

impl<'a, 'b> Div<&'b BigInteger> for &'a BigInteger {
    type Output = BigInteger;

    fn div(self, rhs: &'b BigInteger) -> BigInteger {
        self.div_rem(rhs).unwrap().0
    }
}

impl Div for BigInteger {
    type Output = BigInteger;

    fn div(self, rhs: BigInteger) -> BigInteger {
        &self / &rhs
    }
}

impl<'a> Div<&'a BigInteger> for BigInteger {
    type Output = BigInteger;

    fn div(self, rhs: &'a BigInteger) -> BigInteger {
        &self / rhs
    }
}

impl<'a> Div<BigInteger> for &'a BigInteger {
    type Output = BigInteger;

    fn div(self, rhs: BigInteger) -> BigInteger {
        self / &rhs
    }
}

impl<'a, 'b> Rem<&'b BigInteger> for &'a BigInteger {
    type Output = BigInteger;

    fn rem(self, rhs: &'b BigInteger) -> BigInteger {
        self.div_rem(rhs).unwrap().1
    }
}

impl Rem for BigInteger {
    type Output = BigInteger;

    fn rem(self, rhs: BigInteger) -> BigInteger {
        &self % &rhs
    }
}

impl<'a> Rem<&'a BigInteger> for BigInteger {
    type Output = BigInteger;

    fn rem(self, rhs: &'a BigInteger) -> BigInteger {
        &self % rhs
    }
}

impl<'a> Rem<BigInteger> for &'a BigInteger {
    type Output = BigInteger;

    fn rem(self, rhs: BigInteger) -> BigInteger {
        self % &rhs
    }
}

impl AddAssign<&BigInteger> for BigInteger {
    fn add_assign(&mut self, rhs: &BigInteger) {
        *self = &*self + rhs;
    }
}

impl AddAssign for BigInteger {
    fn add_assign(&mut self, rhs: BigInteger) {
        *self += &rhs;
    }
}

impl SubAssign<&BigInteger> for BigInteger {
    fn sub_assign(&mut self, rhs: &BigInteger) {
        *self = &*self - rhs;
    }
}

impl SubAssign for BigInteger {
    fn sub_assign(&mut self, rhs: BigInteger) {
        *self -= &rhs;
    }
}

impl MulAssign<&BigInteger> for BigInteger {
    fn mul_assign(&mut self, rhs: &BigInteger) {
        *self = &*self * rhs;
    }
}

impl MulAssign for BigInteger {
    fn mul_assign(&mut self, rhs: BigInteger) {
        *self *= &rhs;
    }
}

impl DivAssign<&BigInteger> for BigInteger {
    fn div_assign(&mut self, rhs: &BigInteger) {
        *self = &*self / rhs;
    }
}

impl DivAssign for BigInteger {
    fn div_assign(&mut self, rhs: Self) {
        *self /= &rhs;
    }
}

impl RemAssign<&BigInteger> for BigInteger {
    fn rem_assign(&mut self, rhs: &BigInteger) {
        *self = &*self % rhs;
    }
}

impl RemAssign for BigInteger {
    fn rem_assign(&mut self, rhs: BigInteger) {
        *self %= &rhs;
    }
}

impl Sum for BigInteger {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + b)
    }
}

impl Product for BigInteger {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), |a, b| a * b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = BigInteger::from(1234i32);
        let b = BigInteger::from(5678i32);
        let result = &a + &b;
        assert_eq!(result.to_string(), "6912");

        let a_neg = BigInteger::from(-1234i32);
        let result_neg = &a_neg + &b;
        assert_eq!(result_neg.to_string(), "4444");
    }

    #[test]
    fn test_sub() {
        let a = BigInteger::from(10000i32);
        let b = BigInteger::from(5678i32);
        let result = &a - &b;
        assert_eq!(result.to_string(), "4322");

        let a_neg = BigInteger::from(-10000i32);
        let result_neg = &a_neg - &b;
        assert_eq!(result_neg.to_string(), "-15678");
    }

    #[test]
    #[ignore = "乘法还未实现"]
    fn test_mul() {
        let a = BigInteger::from(1234i32);
        let b = BigInteger::from(5678i32);
        let result = &a * &b;
        assert_eq!(result.to_string(), "7006652");

        let a_neg = BigInteger::from(-1234i32);
        let result_neg = &a_neg * &b;
        assert_eq!(result_neg.to_string(), "-7006652");
    }

    #[test]
    fn test_div() {
        let a = BigInteger::from(123456789i32);
        let b = BigInteger::from(10000i32);
        let result = &a / &b;
        assert_eq!(result.to_string(), "12345");

        let a_neg = BigInteger::from(-123456789i32);
        let result_neg = &a_neg / &b;
        assert_eq!(result_neg.to_string(), "-12345");
    }

    #[test]
    fn test_rem() {
        let a = BigInteger::from(123456789i32);
        let b = BigInteger::from(10000i32);
        let result = &a % &b;
        assert_eq!(result.to_string(), "6789");

        let a_neg = BigInteger::from(-123456789i32);
        let result_neg = &a_neg % &b;
        assert_eq!(result_neg.to_string(), "-6789");
    }
}
