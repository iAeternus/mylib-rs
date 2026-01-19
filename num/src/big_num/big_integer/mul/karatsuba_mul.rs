use std::usize;

use crate::{
    Zero,
    big_num::big_integer::{
        big_integer::BigInteger,
        mul::{NaiveMul, mul::BigIntMul},
    },
};

pub struct KaratsubaMul;

impl KaratsubaMul {
    fn karatsuba(x: &BigInteger, y: &BigInteger) -> BigInteger {
        let n = x.digits.len().max(y.digits.len());

        // 小块使用朴素乘法
        if n <= NaiveMul::limit() {
            return NaiveMul::mul(x, y);
        }

        let m = n >> 1;

        // 拆分 x, y
        let (a, b) = Self::split(x, m);
        let (c, d) = Self::split(y, m);

        // 三次递归
        let ac = Self::karatsuba(&a, &c);
        let bd = Self::karatsuba(&b, &d);
        let ab_cd = Self::karatsuba(&(&a + &b), &(&c + &d));
        let mid = &ab_cd - &ac - &bd;

        // 合并结果
        let ac_shift = Self::shift_digits(&ac, m << 1);
        let mid_shift = Self::shift_digits(&mid, m);

        &(&ac_shift + &mid_shift) + &bd
    }

    #[inline]
    fn split(num: &BigInteger, m: usize) -> (BigInteger, BigInteger) {
        if num.digits.len() > m {
            (
                BigInteger::from_digits(num.sign, num.digits[m..].to_vec()),
                BigInteger::from_digits(num.sign, num.digits[..m].to_vec()),
            )
        } else {
            (BigInteger::zero(), num.clone())
        }
    }

    /// 左移 digits（相当于乘 BASE^shift）TODO: 下一步为BigInteger实现位运算
    fn shift_digits(num: &BigInteger, shift: usize) -> BigInteger {
        if num.is_zero() {
            return BigInteger::zero();
        }
        let mut digits = Vec::with_capacity(num.digits.len() + shift);
        digits.extend(std::iter::repeat(0).take(shift));
        digits.extend_from_slice(&num.digits);
        BigInteger::from_digits(num.sign, digits)
    }
}

impl BigIntMul for KaratsubaMul {
    fn mul(lhs: &BigInteger, rhs: &BigInteger) -> BigInteger {
        if lhs.is_zero() || rhs.is_zero() {
            return BigInteger::zero();
        }

        let mut res = Self::karatsuba(lhs, rhs);
        res.sign = lhs.sign ^ rhs.sign;
        res
    }

    #[inline]
    fn limit() -> usize {
        1024 // 中等数字块阈值
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::big_num::big_integer::{
        big_integer::Sign,
        mul::common::{MUL_RESULT_PATH, assert_res},
    };

    use super::*;

    #[test]
    fn test_basic() {
        let a = BigInteger::from_str("12345678").unwrap(); // 1 digit in BASE=10^8
        let b = BigInteger::from_str("87654321").unwrap(); // 1 digit

        let result = KaratsubaMul::mul(&a, &b);
        let expected = BigInteger::from_str("1082152022374638").unwrap();

        assert_eq!(result.sign, Sign::Positive);
        assert_eq!(result.digits, expected.digits);
    }

    #[test]
    fn test_extremely_large() {
        let a_str = "12345678".repeat(2048);
        let b_str = "87654321".repeat(2048);

        let a = BigInteger::from_str(&a_str).unwrap();
        let b = BigInteger::from_str(&b_str).unwrap();

        let result = KaratsubaMul::mul(&a, &b);

        assert!(!result.is_zero());

        let max_digits = a.digits.len() + b.digits.len();
        assert!(result.digits.len() <= max_digits);

        assert_res(&result.to_string(), MUL_RESULT_PATH);
    }

    #[test]
    fn test_zero() {
        let a = BigInteger::from_str("12345678901234567890").unwrap();
        let zero = BigInteger::zero();

        let result1 = KaratsubaMul::mul(&a, &zero);
        assert!(result1.is_zero());

        let result2 = KaratsubaMul::mul(&zero, &a);
        assert!(result2.is_zero());
    }

    #[test]
    fn test_negative() {
        let a = BigInteger::from_str("12345678").unwrap();
        let mut b = BigInteger::from_str("87654321").unwrap();
        b.sign = Sign::Negative;

        let result = KaratsubaMul::mul(&a, &b);

        assert_eq!(result.sign, Sign::Negative);

        // 绝对值应该正确
        let abs_result = BigInteger::from_digits(Sign::Positive, result.digits.clone());
        let expected = BigInteger::from_str("1082152022374638").unwrap();
        assert_eq!(abs_result.digits, expected.digits);
    }
}
