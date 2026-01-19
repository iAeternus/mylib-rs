use crate::{
    Zero,
    big_num::big_integer::{
        big_integer::{BigInteger, Sign},
        mul::BigIntMul,
    },
    complex::Complex,
};
use std::{f64::consts::PI, usize};

pub struct FFTMul;

impl FFTMul {
    /// 将每个 10^8 进制的 digit 拆分为两个 10^4 进制的部分
    const SPLIT_BASE: u64 = 10_000u64;

    /// 计算 FFT 所需的长度（2 的幂）
    fn fft_len(a_len: usize, b_len: usize) -> usize {
        // 乘法结果的最大位数 = a_len + b_len
        let mut len = 1;
        while len < a_len + b_len {
            len <<= 1;
        }
        len
    }

    fn fft(data: &mut [Complex<f64>], inverse: bool) {
        let n: usize = data.len();

        // 位逆序置换
        let mut j = 0;
        for i in 1..n {
            let mut bit = n >> 1;
            while j & bit != 0 {
                j ^= bit;
                bit >>= 1;
            }
            j ^= bit;

            if i < j {
                data.swap(i, j);
            }
        }

        // 蝴蝶操作
        let mut len = 2;
        while len <= n {
            let half_len = len >> 1;
            let angle = if inverse { 2.0 * PI } else { -2.0 * PI } / len as f64;
            let w_len = Complex::new(angle.cos(), angle.sin());

            for i in (0..n).step_by(len) {
                let mut w = Complex::new(1.0, 0.0);

                for j in 0..half_len {
                    let u = data[i + j];
                    let v = data[i + j + half_len] * w;
                    data[i + j] = u + v;
                    data[i + j + half_len] = u - v;
                    w *= w_len;
                }
            }
            len <<= 1;
        }

        // 如果是逆变换，需要除以 n
        if inverse {
            let factor = 1.0 / n as f64;
            for x in data.iter_mut() {
                *x *= factor;
            }
        }
    }

    /// 将 BigInteger 转换为复数数组
    fn bigint_to_complex_slice(num: &BigInteger, target: &mut [Complex<f64>]) {
        for elem in target.iter_mut() {
            *elem = Complex::new(0.0, 0.0);
        }

        // 拆分每个 digit
        for (i, &digit) in num.digits.iter().enumerate() {
            let digit_val = digit as u64;
            let low = digit_val % Self::SPLIT_BASE;
            let high = digit_val / Self::SPLIT_BASE;

            // 每个 digit 占用两个复数位置
            let pos = i * 2;
            if pos < target.len() {
                target[pos] = Complex::new(low as f64, 0.0);
            }
            if pos + 1 < target.len() {
                target[pos + 1] = Complex::new(high as f64, 0.0);
            }
        }
    }

    /// 将复数数组转换回 BigInteger
    fn complex_slice_to_bigint(data: &[Complex<f64>]) -> BigInteger {
        let real_parts: Vec<i64> = data.iter().map(|c| (c.re + 0.5).floor() as i64).collect();

        let mut digits = Vec::new();
        let mut carry = 0u64;

        // 合并
        for chunk in real_parts.chunks(2) {
            let low = if chunk.len() > 0 {
                chunk[0].max(0) as u64
            } else {
                0
            };
            let high = if chunk.len() > 1 {
                chunk[1].max(0) as u64
            } else {
                0
            };

            let combined = low + high * Self::SPLIT_BASE + carry;

            digits.push((combined % BigInteger::BASE as u64) as u32);
            carry = combined / BigInteger::BASE as u64;
        }

        while carry > 0 {
            digits.push((carry % BigInteger::BASE as u64) as u32);
            carry /= BigInteger::BASE as u64;
        }

        while digits.len() > 1 && *digits.last().unwrap() == 0 {
            digits.pop();
        }

        // 如果没有数字，返回 0
        if digits.is_empty() {
            digits.push(0);
        }

        BigInteger::from_digits(Sign::Positive, digits)
    }

    /// 使用内存池执行乘法，避免重复分配
    fn multiply_with_pool(
        lhs: &BigInteger,
        rhs: &BigInteger,
        pool: &mut Vec<Complex<f64>>,
    ) -> BigInteger {
        // 计算 FFT 长度，每个 digit 拆分为 2 个系数
        let a_coeff_len = lhs.digits.len() << 1;
        let b_coeff_len = rhs.digits.len() << 1;
        let fft_len = Self::fft_len(a_coeff_len, b_coeff_len);

        // 调整内存池大小
        pool.resize(fft_len * 2, Complex::new(0.0, 0.0));
        let (left, right) = pool.split_at_mut(fft_len);

        // 转换为复数数组
        Self::bigint_to_complex_slice(lhs, left);
        Self::bigint_to_complex_slice(rhs, right);

        // FFT
        Self::fft(left, false);
        Self::fft(right, false);

        // 点值表示相乘
        for i in 0..fft_len {
            left[i] *= right[i];
        }

        // IFFT
        Self::fft(left, true);

        // 转换回 BigInteger
        Self::complex_slice_to_bigint(left)
    }
}

impl BigIntMul for FFTMul {
    fn mul(lhs: &BigInteger, rhs: &BigInteger) -> BigInteger {
        if lhs.is_zero() || rhs.is_zero() {
            return BigInteger::zero();
        }

        let mut pool = Vec::new();
        let mut result = Self::multiply_with_pool(lhs, rhs, &mut pool);

        result.sign = lhs.sign ^ rhs.sign;
        result
    }

    #[inline]
    fn limit() -> usize {
        1048576 // 超大数字块阈值
    }
}

#[cfg(test)]
mod tests {
    use crate::big_num::big_integer::big_integer::Sign;

    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_basic() {
        let a = BigInteger::from_str("12345678").unwrap(); // 1 digit in BASE=10^8
        let b = BigInteger::from_str("87654321").unwrap(); // 1 digit

        let result = FFTMul::mul(&a, &b);
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

        let result = FFTMul::mul(&a, &b);
        assert_eq!(result.to_string(), expect);
    }

    #[test]
    fn test_zero() {
        let a = BigInteger::from_str("12345678901234567890").unwrap();
        let zero = BigInteger::zero();

        let result1 = FFTMul::mul(&a, &zero);
        assert!(result1.is_zero());

        let result2 = FFTMul::mul(&zero, &a);
        assert!(result2.is_zero());
    }

    #[test]
    fn test_negative() {
        let a = BigInteger::from_str("12345678").unwrap();
        let mut b = BigInteger::from_str("87654321").unwrap();
        b.sign = Sign::Negative;

        let result = FFTMul::mul(&a, &b);

        assert_eq!(result.sign, Sign::Negative);

        // 绝对值应该正确
        let abs_result = BigInteger::from_digits(Sign::Positive, result.digits.clone());
        let expected = BigInteger::from_str("1082152022374638").unwrap();
        assert_eq!(abs_result.digits, expected.digits);
    }
}
