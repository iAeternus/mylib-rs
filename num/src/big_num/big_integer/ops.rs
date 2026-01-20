use std::{
    cmp::Ordering,
    iter::{Product, Sum},
    ops::{
        Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
    },
};

use crate::{
    Zero,
    big_num::big_integer::{
        big_integer::{BigInteger, Sign},
        mul::{BigIntMul, FFTMul, NaiveMul},
    },
    core::one::One,
};

impl BitXor for Sign {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => Sign::Positive,
            _ => Sign::Negative,
        }
    }
}

impl Neg for &BigInteger {
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
    type Output = Self;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl Add<&BigInteger> for &BigInteger {
    type Output = BigInteger;

    fn add(self, rhs: &BigInteger) -> BigInteger {
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
    type Output = Self;

    fn add(self, rhs: BigInteger) -> Self::Output {
        &self + &rhs
    }
}

impl Add<&BigInteger> for BigInteger {
    type Output = BigInteger;

    fn add(self, rhs: &BigInteger) -> BigInteger {
        &self + rhs
    }
}

impl Add<BigInteger> for &BigInteger {
    type Output = BigInteger;

    fn add(self, rhs: BigInteger) -> BigInteger {
        self + &rhs
    }
}

impl Sub<&BigInteger> for &BigInteger {
    type Output = BigInteger;

    fn sub(self, rhs: &BigInteger) -> BigInteger {
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
    type Output = Self;

    fn sub(self, rhs: BigInteger) -> Self::Output {
        &self - &rhs
    }
}

impl Sub<&BigInteger> for BigInteger {
    type Output = BigInteger;

    fn sub(self, rhs: &BigInteger) -> BigInteger {
        &self - rhs
    }
}

impl Sub<BigInteger> for &BigInteger {
    type Output = BigInteger;

    fn sub(self, rhs: BigInteger) -> BigInteger {
        self - &rhs
    }
}

impl Mul<u32> for &BigInteger {
    type Output = BigInteger;

    fn mul(self, rhs: u32) -> BigInteger {
        self.mul_u32(rhs)
    }
}

impl Mul<u32> for BigInteger {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        self.mul_u32(rhs)
    }
}

impl Mul<&BigInteger> for u32 {
    type Output = BigInteger;

    fn mul(self, rhs: &BigInteger) -> Self::Output {
        rhs.mul_u32(self)
    }
}

impl Mul<BigInteger> for u32 {
    type Output = BigInteger;

    fn mul(self, rhs: BigInteger) -> Self::Output {
        rhs.mul_u32(self)
    }
}

impl Mul<&BigInteger> for &BigInteger {
    type Output = BigInteger;

    fn mul(self, rhs: &BigInteger) -> BigInteger {
        if self.is_zero() || rhs.is_zero() {
            return BigInteger::zero();
        }

        let n = self.digits.len().max(rhs.digits.len());
        if n <= NaiveMul::limit() {
            NaiveMul::mul(self, rhs)
        } else if n <= FFTMul::limit() {
            FFTMul::mul(self, rhs)
        } else {
            panic!(
                "Number too large! lhs size: {}, rhs size: {}",
                self.size(),
                rhs.size()
            )
        }
    }
}

impl Mul for BigInteger {
    type Output = Self;

    fn mul(self, rhs: BigInteger) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&BigInteger> for BigInteger {
    type Output = BigInteger;

    fn mul(self, rhs: &BigInteger) -> BigInteger {
        &self * rhs
    }
}

impl Mul<BigInteger> for &BigInteger {
    type Output = BigInteger;

    fn mul(self, rhs: BigInteger) -> BigInteger {
        self * &rhs
    }
}

impl Div<&BigInteger> for &BigInteger {
    type Output = BigInteger;

    fn div(self, rhs: &BigInteger) -> BigInteger {
        self.div_rem(rhs)
            .unwrap_or_else(|err| {
                panic!("{}", err);
            })
            .0
    }
}

impl Div for BigInteger {
    type Output = Self;

    fn div(self, rhs: BigInteger) -> Self::Output {
        &self / &rhs
    }
}

impl Div<&BigInteger> for BigInteger {
    type Output = BigInteger;

    fn div(self, rhs: &BigInteger) -> BigInteger {
        &self / rhs
    }
}

impl Div<BigInteger> for &BigInteger {
    type Output = BigInteger;

    fn div(self, rhs: BigInteger) -> BigInteger {
        self / &rhs
    }
}

impl Rem<&BigInteger> for &BigInteger {
    type Output = BigInteger;

    fn rem(self, rhs: &BigInteger) -> BigInteger {
        self.div_rem(rhs)
            .unwrap_or_else(|err| {
                panic!("{}", err);
            })
            .1
    }
}

impl Rem for BigInteger {
    type Output = Self;

    fn rem(self, rhs: BigInteger) -> Self::Output {
        &self % &rhs
    }
}

impl Rem<&BigInteger> for BigInteger {
    type Output = BigInteger;

    fn rem(self, rhs: &BigInteger) -> BigInteger {
        &self % rhs
    }
}

impl Rem<BigInteger> for &BigInteger {
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
    fn test_mul_basic() {
        // 短数字块 -> NaiveMul
        let a = BigInteger::from(1234i32);
        let b = BigInteger::from(5678i32);
        let result = &a * &b;
        assert_eq!(result.to_string(), "7006652");

        // 负数测试
        let a_neg = BigInteger::from(-1234i32);
        let result_neg = &a_neg * &b;
        assert_eq!(result_neg.to_string(), "-7006652");

        let b_neg = BigInteger::from(-5678i32);
        let result_neg2 = &a * &b_neg;
        assert_eq!(result_neg2.to_string(), "-7006652");

        let both_neg = &a_neg * &b_neg;
        assert_eq!(both_neg.to_string(), "7006652");
    }

    #[test]
    fn test_mul_zero_one() {
        let zero = BigInteger::zero();
        let one = BigInteger::one();
        let a = BigInteger::from(12345i32);

        assert_eq!((&a * &zero).to_string(), "0");
        assert_eq!((&zero * &a).to_string(), "0");
        assert_eq!((&a * &one).to_string(), a.to_string());
        assert_eq!((&one * &a).to_string(), a.to_string());
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
    #[should_panic(expected = "division by zero")]
    fn test_div_by_zero() {
        let a = BigInteger::from(123456789i32);
        let b = BigInteger::zero();
        let _ = &a / &b;
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

    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_rem_by_zero() {
        let a = BigInteger::from(123456789i32);
        let b = BigInteger::zero();
        let _ = &a % &b;
    }
}
