use crate::big_num::big_integer::{big_integer::BigInteger, mul::mul::BigIntMul};

pub struct NaiveMul;

impl BigIntMul for NaiveMul {
    fn mul(lhs: &BigInteger, rhs: &BigInteger) -> BigInteger {
        let a_len = lhs.digits.len();
        let b_len = rhs.digits.len();
        let mut res: Vec<i64> = vec![0; a_len + b_len];

        for i in 0..a_len {
            for j in 0..b_len {
                res[i + j] += lhs.digits[i] as i64 * rhs.digits[j] as i64;
            }
        }

        // 处理进位
        let mut carry = 0i64;
        let mut digits = Vec::with_capacity(res.len());
        for x in res.iter() {
            let total = *x + carry;
            digits.push((total % BigInteger::BASE as i64) as u32);
            carry = total / BigInteger::BASE as i64;
        }
        if carry > 0 {
            digits.push(carry as u32);
        }

        // 去除前导零
        while digits.len() > 1 && *digits.last().unwrap() == 0 {
            digits.pop();
        }

        BigInteger {
            sign: lhs.sign ^ rhs.sign,
            digits,
        }
    }

    #[inline]
    fn limit() -> usize {
        32 // 小数字块阈值
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{Zero, big_num::big_integer::big_integer::Sign};

    use super::*;

    #[test]
    fn test_basic() {
        let a = BigInteger::from_str("12345678").unwrap(); // 1 digit in BASE=10^8
        let b = BigInteger::from_str("87654321").unwrap(); // 1 digit

        let result = NaiveMul::mul(&a, &b);
        let expected = BigInteger::from_str("1082152022374638").unwrap();

        assert_eq!(result.sign, Sign::Positive);
        assert_eq!(result.digits, expected.digits);
    }

    #[test]
    fn test_extremely_large() {
        let a_str = "1234567890".repeat(4);
        let b_str = "9876543210".repeat(4);
        let expect =
            "12193263113702179522618503273386678859448712086533622923332237463801111263526900";

        let a = BigInteger::from_str(&a_str).unwrap();
        let b = BigInteger::from_str(&b_str).unwrap();

        let result = NaiveMul::mul(&a, &b);
        assert_eq!(result.to_string(), expect);
    }

    #[test]
    fn test_zero() {
        let a = BigInteger::from_str("12345678901234567890").unwrap();
        let zero = BigInteger::zero();

        let result1 = NaiveMul::mul(&a, &zero);
        assert!(result1.is_zero());

        let result2 = NaiveMul::mul(&zero, &a);
        assert!(result2.is_zero());
    }

    #[test]
    fn test_negative() {
        let a = BigInteger::from_str("12345678").unwrap();
        let mut b = BigInteger::from_str("87654321").unwrap();
        b.sign = Sign::Negative;

        let result = NaiveMul::mul(&a, &b);

        assert_eq!(result.sign, Sign::Negative);

        // 绝对值应该正确
        let abs_result = BigInteger::from_digits(Sign::Positive, result.digits.clone());
        let expected = BigInteger::from_str("1082152022374638").unwrap();
        assert_eq!(abs_result.digits, expected.digits);
    }
}
