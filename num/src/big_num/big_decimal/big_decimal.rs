use std::{
    cmp::Ordering,
    fmt::{self, Display},
    str::FromStr,
};

use crate::{One, Zero, big_num::big_integer::big_integer::BigInteger, error::NumError};

/// 高精度浮点数
///
/// ## Notes
/// - value = coeff * 10^(-scale)
/// - coeff 不含多余尾零
#[derive(Clone, Debug)]
pub struct BigDecimal {
    /// 系数，去掉小数点后的整数
    coeff: BigInteger,
    /// 小数位数
    scale: i32,
}

impl BigDecimal {
    pub fn new(coeff: BigInteger, scale: i32) -> Self {
        let mut x = Self { coeff, scale };
        x.normalize();
        x
    }

    pub fn coeff(&self) -> &BigInteger {
        &self.coeff
    }

    pub fn scale(&self) -> i32 {
        self.scale
    }

    /// 去除 coeff 的十进制尾随 0，并同步减少 scale
    pub fn normalize(&mut self) {
        if self.coeff.is_zero() {
            self.scale = 0;
            return;
        }

        while self.scale > 0 {
            // 判断最低一位是否为 0
            if self.coeff.digits[0] % 10 != 0 {
                break;
            }

            // coeff /= 10
            self.coeff = self.coeff.div_u32(10);
            self.scale -= 1;
        }
    }
}

impl Zero for BigDecimal {
    fn zero() -> Self {
        Self {
            coeff: BigInteger::zero(),
            scale: 0,
        }
    }

    fn is_zero(&self) -> bool {
        self.coeff.is_zero()
    }
}

impl One for BigDecimal {
    fn one() -> Self {
        Self {
            coeff: BigInteger::one(),
            scale: 0,
        }
    }

    fn is_one(&self) -> bool {
        self.scale == 0 && self.coeff.is_one()
    }
}

impl From<i64> for BigDecimal {
    fn from(n: i64) -> Self {
        Self {
            coeff: BigInteger::from(n),
            scale: 0,
        }
    }
}

impl From<BigInteger> for BigDecimal {
    fn from(n: BigInteger) -> Self {
        Self { coeff: n, scale: 0 }
    }
}

impl FromStr for BigDecimal {
    type Err = NumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(NumError::ParseBigDecError);
        }

        let neg = s.starts_with('-');
        let s = if neg { &s[1..] } else { s };

        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() > 2 {
            return Err(NumError::ParseBigDecError);
        }

        let int_part = parts[0];
        let frac_part = if parts.len() == 2 { parts[1] } else { "" };

        if int_part.is_empty() && frac_part.is_empty() {
            return Err(NumError::ParseBigDecError);
        }

        let mut digits = String::new();
        digits.push_str(int_part);
        digits.push_str(frac_part);

        let mut coeff = BigInteger::from_str(&digits)?;
        if neg {
            coeff = -coeff;
        }

        let scale = frac_part.len() as i32;

        Ok(Self::new(coeff, scale))
    }
}

impl PartialEq for BigDecimal {
    fn eq(&self, other: &Self) -> bool {
        let mut a = self.clone();
        let mut b = other.clone();
        a.normalize();
        b.normalize();
        a.coeff == b.coeff && a.scale == b.scale
    }
}

impl Eq for BigDecimal {}

impl Ord for BigDecimal {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut a = self.clone();
        let mut b = other.clone();

        a.normalize();
        b.normalize();

        if a.scale == b.scale {
            return a.coeff.cmp(&b.coeff);
        }

        if a.scale > b.scale {
            let diff = (a.scale - b.scale) as usize;
            let lhs = a.coeff.clone();
            let rhs = b.coeff.mul_pow10(diff);
            lhs.cmp(&rhs)
        } else {
            let diff = (b.scale - a.scale) as usize;
            let lhs = a.coeff.mul_pow10(diff);
            let rhs = b.coeff.clone();
            lhs.cmp(&rhs)
        }
    }
}

impl PartialOrd for BigDecimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for BigDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut x = self.clone();
        x.normalize();

        let mut coeff = x.coeff.to_string();

        if x.scale == 0 {
            return write!(f, "{}", coeff);
        }

        let neg = coeff.starts_with('-');
        if neg {
            coeff.remove(0);
        }

        let scale = x.scale as usize;

        if coeff.len() > scale {
            let split = coeff.len() - scale;
            if neg {
                write!(f, "-")?;
            }
            write!(f, "{}.{}", &coeff[..split], &coeff[split..])
        } else {
            if neg {
                write!(f, "-")?;
            }
            write!(f, "0.")?;
            for _ in 0..(scale - coeff.len()) {
                write!(f, "0")?;
            }
            write!(f, "{}", coeff)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_display() {
        let x = BigDecimal::from_str("123.4500").unwrap();
        assert_eq!(x.to_string(), "123.45");

        let y = BigDecimal::from_str("-0.00100").unwrap();
        assert_eq!(y.to_string(), "-0.001");
    }

    #[test]
    fn test_eq() {
        let a = BigDecimal::from_str("1.2300").unwrap();
        let b = BigDecimal::from_str("1.23").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_cmp() {
        let a = BigDecimal::from_str("1.2").unwrap();
        let b = BigDecimal::from_str("1.20").unwrap();
        let c = BigDecimal::from_str("1.3").unwrap();

        assert_eq!(a, b);
        assert!(c > a);
    }

    #[test]
    fn test_from_int() {
        let x = BigDecimal::from(123i64);
        assert_eq!(x.to_string(), "123");
    }

    #[test]
    fn test_zero() {
        let z = BigDecimal::zero();
        assert_eq!(z.to_string(), "0");
    }
}
