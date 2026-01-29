use std::{cmp::Ordering, fmt::Display, str::FromStr};

use crate::{
    One, Zero,
    error::{NumError, NumResult},
};

/// 符号
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

/// 任意精度有符号整数
///
/// ## 存储方式
/// - 基数：`10^8`
/// - 小端序（低位在前）
///
/// ## 示例
/// ```text
/// 1234_56789012_34567890
/// digits = [
///     34567890,
///     56789012,
///     1234,
/// ]
/// ```
///
/// ## 约定
/// - `digits[0]` 为最低有效块
/// - 最高位块不为 0（无前导零）
/// - 零始终表示为正数（不存在负零）
#[derive(Clone, Debug)]
pub struct BigInteger {
    /// 符号位（正 / 负）
    pub sign: Sign,

    /// 数值块（base = 10^8，小端序）
    pub digits: Vec<u32>,
}

impl BigInteger {
    /// 每个数字块的进制基数（`digits[i] < BASE`，小端存储）
    pub const BASE: u32 = 100_000_000;

    /// 单个数字块表示的十进制位数（`BASE = 10^WIDTH`）
    pub const WIDTH: usize = 8;

    pub(crate) fn from_digits(sign: Sign, mut digits: Vec<u32>) -> Self {
        // 去除高位前导 0
        while digits.len() > 1 && *digits.last().unwrap() == 0 {
            digits.pop();
        }

        // 0 永远是正数
        let sign = if digits.len() == 1 && digits[0] == 0 {
            Sign::Positive
        } else {
            sign
        };

        Self { sign, digits }
    }

    /// 获取数字位数
    pub fn size(&self) -> usize {
        let mut size = (self.digits.len() - 1) * Self::WIDTH;
        let mut high_chunk = *self.digits.last().unwrap();
        while high_chunk > 0 {
            size += 1;
            high_chunk /= 10;
        }
        size
    }

    pub fn abs(&self) -> Self {
        let mut x = self.clone();
        x.sign = Sign::Positive;
        x
    }

    pub fn is_negative(&self) -> bool {
        self.sign == Sign::Negative
    }

    pub fn is_odd(&self) -> bool {
        !self.is_zero() && (self.digits[0] & 1) == 1
    }

    pub fn is_even(&self) -> bool {
        (self.digits[0] & 1) == 0
    }

    pub fn gcd(&self, other: &Self) -> Self {
        let mut a = self.abs();
        let mut b = other.abs();

        while !b.is_zero() {
            let r = a.div_rem(&b).unwrap().1;
            a = b;
            b = r;
        }

        a
    }

    pub fn lcm(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }
        (self / &self.gcd(other)) * other.abs()
    }

    pub fn two() -> Self {
        Self {
            sign: Sign::Positive,
            digits: vec![2],
        }
    }

    pub(crate) fn abs_cmp(&self, other: &Self) -> Ordering {
        if self.digits.len() != other.digits.len() {
            return self.digits.len().cmp(&other.digits.len());
        }

        for i in (0..self.digits.len()).rev() {
            match self.digits[i].cmp(&other.digits[i]) {
                Ordering::Equal => continue,
                non_eq => return non_eq,
            }
        }

        Ordering::Equal
    }

    pub(crate) fn abs_add(a: &Self, b: &Self) -> Self {
        let mut res = Vec::new();
        let mut carry: u64 = 0;

        let n = a.digits.len().max(b.digits.len());
        for i in 0..n {
            let x = *a.digits.get(i).unwrap_or(&0) as u64;
            let y = *b.digits.get(i).unwrap_or(&0) as u64;
            let sum = x + y + carry;
            res.push((sum % Self::BASE as u64) as u32);
            carry = sum / Self::BASE as u64;
        }

        if carry > 0 {
            res.push(carry as u32);
        }

        Self {
            sign: Sign::Positive,
            digits: res,
        }
    }

    pub(crate) fn abs_sub(a: &Self, b: &Self) -> Self {
        // 要求 |a| >= |b|
        let mut res = Vec::new();
        let mut borrow: u32 = 0;

        for i in 0..a.digits.len() {
            let x = (a.digits[i] - borrow) as i64;
            let y = *b.digits.get(i).unwrap_or(&0) as i64;

            if x >= y {
                res.push((x - y) as u32);
                borrow = 0;
            } else {
                res.push((x + Self::BASE as i64 - y) as u32);
                borrow = 1;
            }
        }

        Self::from_digits(Sign::Positive, res)
    }

    pub fn div_rem(&self, rhs: &Self) -> NumResult<(Self, Self)> {
        if rhs.is_zero() {
            return Err(NumError::DivisionByZero);
        }

        unsafe { Ok(self.div_rem_unchecked(rhs)) }
    }

    /// 大整数除法，不做除零检验
    ///
    /// ## Safety
    /// 调用者需保证`rhs`不为零
    ///
    /// ## Return
    /// (商, 余数)
    pub unsafe fn div_rem_unchecked(&self, rhs: &Self) -> (Self, Self) {
        if self.abs() < rhs.abs() {
            return (Self::zero(), self.clone());
        }

        let mut quotient = Vec::with_capacity(self.digits.len());
        let mut current = Self::zero();

        // 从高位到低位
        for &d in self.digits.iter().rev() {
            current = current.mul_base_add(d);

            // 二分查商 (0..BASE)
            let mut low = 0u32;
            let mut high = Self::BASE - 1;
            let mut q = 0;

            while low <= high {
                let mid = (low + high) >> 1;
                if rhs.abs().mul_u32(mid) <= current {
                    q = mid;
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }

            current -= rhs.abs().mul_u32(q);
            quotient.push(q);
        }

        quotient.reverse();
        let q = Self::from_digits(self.sign ^ rhs.sign, quotient);

        let mut r = current;
        r.sign = self.sign;

        (q, r)
    }

    #[inline]
    pub(crate) fn mul_u32(&self, x: u32) -> Self {
        // self * x (x < BASE)
        if x == 0 || self.is_zero() {
            return Self::zero();
        }

        let mut res = Vec::with_capacity(self.digits.len() + 1);
        let mut carry: u64 = 0;

        for &d in &self.digits {
            let tmp = d as u64 * x as u64 + carry;
            res.push((tmp % Self::BASE as u64) as u32);
            carry = tmp / Self::BASE as u64;
        }

        if carry > 0 {
            res.push(carry as u32);
        }

        Self {
            sign: self.sign,
            digits: res,
        }
    }

    // TODO: 还未重载运算符
    #[inline]
    pub fn div_u32(&self, rhs: u32) -> Self {
        assert!(rhs > 0);

        let mut res = Vec::with_capacity(self.digits.len());
        let mut rem: u64 = 0;

        for &d in self.digits.iter().rev() {
            let cur = rem * Self::BASE as u64 + d as u64;
            res.push((cur / rhs as u64) as u32);
            rem = cur % rhs as u64;
        }

        res.reverse();
        Self::from_digits(self.sign, res)
    }

    #[inline]
    fn mul_base_add(&self, d: u32) -> Self {
        // self * BASE + d
        if self.is_zero() {
            return Self {
                sign: Sign::Positive,
                digits: vec![d],
            };
        }

        let mut digits = Vec::with_capacity(self.digits.len() + 1);
        digits.push(d); // 低位
        digits.extend_from_slice(&self.digits);

        Self {
            sign: self.sign,
            digits,
        }
    }

    pub fn pow(&self, mut exp: u64) -> Self {
        if exp == 0 {
            return Self::one();
        }

        let mut base = self.clone();
        let mut result = Self::one();

        while exp > 0 {
            if exp & 1 == 1 {
                result *= &base;
            }
            base = &base * &base;
            exp >>= 1;
        }

        result
    }

    pub fn mod_pow(&self, exp: &Self, m: &Self) -> NumResult<Self> {
        if m.is_zero() {
            return Err(NumError::DivisionByZero);
        }

        unsafe { Ok(self.mod_pow_unchecked(exp, m)) }
    }

    pub unsafe fn mod_pow_unchecked(&self, exp: &Self, m: &Self) -> Self {
        if exp.is_zero() {
            return Self::one();
        }

        let mut base = self % m;
        let mut e = exp.clone();
        let mut result = Self::one();
        let two: BigInteger = Self::two();

        while !e.is_zero() {
            if e.is_odd() {
                result = (&result * &base) % m;
            }
            base = (&base * &base) % m;
            e /= &two;
        }

        result
    }

    /// 乘以 10^k
    pub fn mul_pow10(&self, k: usize) -> Self {
        if self.is_zero() {
            return Self::zero();
        }

        let block_shift = k / Self::WIDTH;
        let digit_shift = k % Self::WIDTH;

        // 整块扩展
        let mut digits = Vec::with_capacity(self.digits.len() + block_shift + 1);
        digits.extend(std::iter::repeat(0).take(block_shift));
        digits.extend_from_slice(&self.digits);

        if digit_shift == 0 {
            return Self::from_digits(self.sign, digits);
        }

        // 块内乘 10^digit_shift
        let mul = 10u32.pow(digit_shift as u32) as u64;
        let mut carry = 0u64;

        for d in digits.iter_mut() {
            let tmp = *d as u64 * mul + carry;
            *d = (tmp % Self::BASE as u64) as u32;
            carry = tmp / Self::BASE as u64;
        }

        if carry > 0 {
            digits.push(carry as u32);
        }

        Self::from_digits(self.sign, digits)
    }

    /// 除以 10^k，返回 (商, 余数)
    pub fn div_rem_pow10(&self, k: usize) -> (Self, Self) {
        if self.is_zero() {
            return (Self::zero(), Self::zero());
        }

        let block_shift = k / Self::WIDTH;
        let digit_shift = k % Self::WIDTH;

        if block_shift >= self.digits.len() {
            return (Self::zero(), self.clone());
        }

        let mut q_digits = self.digits[block_shift..].to_vec();
        let mut rem_high = 0u64;

        if digit_shift != 0 {
            let div = 10u32.pow(digit_shift as u32) as u64;

            for d in q_digits.iter_mut().rev() {
                let cur = rem_high * Self::BASE as u64 + *d as u64;
                *d = (cur / div) as u32;
                rem_high = cur % div;
            }
        }

        let q = Self::from_digits(self.sign, q_digits);

        let mut r_digits = self.digits[..block_shift].to_vec();
        if digit_shift != 0 {
            let mul = 10u32.pow(digit_shift as u32) as u64;
            let mut carry = rem_high;

            for d in r_digits.iter_mut() {
                let tmp = *d as u64 * mul + carry;
                *d = (tmp % Self::BASE as u64) as u32;
                carry = tmp / Self::BASE as u64;
            }

            if carry > 0 {
                r_digits.push(carry as u32);
            }
        }

        let r = Self::from_digits(self.sign, r_digits);

        (q, r)
    }
}

impl Zero for BigInteger {
    fn zero() -> Self {
        Self {
            sign: Sign::Positive,
            digits: vec![0],
        }
    }

    fn is_zero(&self) -> bool {
        self.digits.len() == 1 && self.digits[0] == 0
    }
}

impl One for BigInteger {
    fn one() -> Self {
        Self {
            sign: Sign::Positive,
            digits: vec![1],
        }
    }

    fn is_one(&self) -> bool {
        self.sign == Sign::Positive && self.digits.len() == 1 && self.digits[0] == 1
    }
}

impl Default for BigInteger {
    fn default() -> Self {
        Self::zero()
    }
}

impl From<i32> for BigInteger {
    fn from(n: i32) -> Self {
        Self::from(n as i64)
    }
}

impl From<i64> for BigInteger {
    fn from(mut n: i64) -> Self {
        if n == 0 {
            return Self::zero();
        }

        let sign = if n < 0 {
            n = -n;
            Sign::Negative
        } else {
            Sign::Positive
        };

        let mut digits = Vec::new();
        while n > 0 {
            digits.push((n % Self::BASE as i64) as u32);
            n /= Self::BASE as i64;
        }

        Self { sign, digits }
    }
}

impl FromStr for BigInteger {
    type Err = NumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(NumError::ParseBigIntError);
        }

        let (sign, digits_str) = if s.starts_with('-') {
            (Sign::Negative, &s[1..])
        } else {
            (Sign::Positive, s)
        };

        let mut digits = Vec::new();
        let mut i = digits_str.len();

        while i > 0 {
            let start = i.saturating_sub(Self::WIDTH);
            let chunk = &digits_str[start..i];
            digits.push(
                chunk
                    .parse::<u32>()
                    .map_err(|_| NumError::ParseBigIntError)?,
            );
            i = start;
        }

        Ok(Self::from_digits(sign, digits))
    }
}

impl PartialEq for BigInteger {
    fn eq(&self, other: &Self) -> bool {
        self.sign == other.sign && self.digits == other.digits
    }
}

impl Eq for BigInteger {}

impl Ord for BigInteger {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.sign, other.sign) {
            (Sign::Positive, Sign::Negative) => Ordering::Greater,
            (Sign::Negative, Sign::Positive) => Ordering::Less,
            (Sign::Positive, Sign::Positive) => self.abs_cmp(other),
            (Sign::Negative, Sign::Negative) => other.abs_cmp(self),
        }
    }
}

impl PartialOrd for BigInteger {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for BigInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.sign == Sign::Negative {
            write!(f, "-")?;
        }
        let mut it = self.digits.iter().rev();
        write!(f, "{}", it.next().unwrap())?;
        for d in it {
            write!(f, "{:0width$}", d, width = Self::WIDTH)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let num = BigInteger::from(1234567890);
        assert_eq!(num.size(), 10);
    }

    #[test]
    fn test_from_i32() {
        let num = BigInteger::from(1234i32);
        assert_eq!(num.to_string(), "1234");

        let num_neg = BigInteger::from(-5678i32);
        assert_eq!(num_neg.to_string(), "-5678");
    }

    #[test]
    fn test_from_i64() {
        let num = BigInteger::from(1234567890i64);
        assert_eq!(num.to_string(), "1234567890");

        let num_neg = BigInteger::from(-987654321i64);
        assert_eq!(num_neg.to_string(), "-987654321");
    }

    #[test]
    fn test_gcd() {
        let a = BigInteger::from(56i32);
        let b = BigInteger::from(98i32);
        let result = a.gcd(&b);
        assert_eq!(result.to_string(), "14");

        let a_prime = BigInteger::from(13i32);
        let b_prime = BigInteger::from(17i32);
        let result_prime = a_prime.gcd(&b_prime);
        assert_eq!(result_prime.to_string(), "1");
    }

    #[test]
    fn test_lcm() {
        let a = BigInteger::from(56i32);
        let b = BigInteger::from(98i32);
        let result = a.lcm(&b);
        assert_eq!(result.to_string(), "392");

        let a_prime = BigInteger::from(13i32);
        let b_prime = BigInteger::from(17i32);
        let result_prime = a_prime.lcm(&b_prime);
        assert_eq!(result_prime.to_string(), "221");
    }

    #[test]
    fn test_pow() {
        let a = BigInteger::from(2i32);
        let result = a.pow(10);
        assert_eq!(result.to_string(), "1024");

        let a_neg = BigInteger::from(-2i32);
        let result_neg = a_neg.pow(3);
        assert_eq!(result_neg.to_string(), "-8");
    }

    #[test]
    fn test_mod_pow() {
        let a = BigInteger::from(2i32);
        let exp = BigInteger::from(10i32);
        let m = BigInteger::from(1000i32);
        let result = a.mod_pow(&exp, &m).unwrap();
        assert_eq!(result.to_string(), "24");
    }

    #[test]
    fn test_is_zero() {
        let zero = BigInteger::zero();
        assert!(zero.is_zero());

        let non_zero = BigInteger::from(123i32);
        assert!(!non_zero.is_zero());
    }

    #[test]
    fn test_is_negative() {
        let neg = BigInteger::from(-123i32);
        assert!(neg.is_negative());

        let pos = BigInteger::from(123i32);
        assert!(!pos.is_negative());
    }

    #[test]
    fn test_is_odd() {
        let odd = BigInteger::from(123i32);
        assert!(odd.is_odd());

        let even = BigInteger::from(124i32);
        assert!(!even.is_odd());
    }

    #[test]
    fn test_is_even() {
        let odd = BigInteger::from(123i32);
        assert!(!odd.is_even());

        let even = BigInteger::from(124i32);
        assert!(even.is_even());
    }

    #[test]
    fn test_zero_and_one() {
        let zero = BigInteger::zero();
        assert_eq!(zero.to_string(), "0");

        let one = BigInteger::one();
        assert_eq!(one.to_string(), "1");
    }

    #[test]
    fn test_large_number() {
        let large_num = BigInteger::from_str("1234567890123456789012345678901234567890").unwrap();
        assert_eq!(
            large_num.to_string(),
            "1234567890123456789012345678901234567890"
        );

        let large_num_neg =
            BigInteger::from_str("-1234567890123456789012345678901234567890").unwrap();
        assert_eq!(
            large_num_neg.to_string(),
            "-1234567890123456789012345678901234567890"
        );
    }

    #[test]
    fn test_division_by_zero() {
        let a = BigInteger::from(123456i32);
        let b = BigInteger::zero();
        let result = a.div_rem(&b);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid() {
        let invalid = BigInteger::from_str("abc");
        assert!(invalid.is_err());
    }

    #[test]
    fn test_pow10_ops() {
        let n = BigInteger::from_str("12345678901234567890").unwrap();

        let (q, r) = n.div_rem_pow10(3);
        assert_eq!(q.to_string(), "12345678901234567");
        assert_eq!(r.to_string(), "890");

        let m = BigInteger::from(1234);
        assert_eq!(m.mul_pow10(5).to_string(), "123400000");
    }
}
