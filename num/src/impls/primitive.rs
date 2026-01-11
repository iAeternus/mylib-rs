use crate::core::*;

/// Zero / One
macro_rules! impl_zero_one {
    ($($t:ty => $zero:expr, $one:expr);+ $(;)?) => {
        $(
            impl Zero for $t {
                #[inline]
                fn zero() -> Self {
                    $zero
                }

                #[inline]
                fn is_zero(&self) -> bool {
                    *self == $zero
                }
            }

            impl One for $t {
                #[inline]
                fn one() -> Self {
                    $one
                }

                #[inline]
                fn is_one(&self) -> bool {
                    *self == $one
                }
            }
        )+
    };
}

impl_zero_one!(
    i8    => 0_i8, 1_i8;
    u8    => 0_u8, 1_u8;
    i16   => 0_i16, 1_i16;
    u16   => 0_u16, 1_u16;
    i32   => 0_i32, 1_i32;
    u32   => 0_u32, 1_u32;
    i64   => 0_i64, 1_i64;
    u64   => 0_u64, 1_u64;
    i128  => 0_i128, 1_i128;
    u128  => 0_u128, 1_u128;
    isize => 0_isize, 1_isize;
    usize => 0_usize, 1_usize;
);

impl_zero_one!(
    f32 => 0.0_f32, 1.0_f32;
    f64 => 0.0_f64, 1.0_f64;
);

/// Signed + Integer
macro_rules! impl_signed_integer {
    ($($t:ty),+ $(,)?) => {
        $(
            impl Signed for $t {
                #[inline]
                fn abs(self) -> Self {
                    self.abs()
                }

                #[inline]
                fn is_negative(self) -> bool {
                    self < 0
                }
            }

            impl Integer for $t {
                fn gcd(self, other: Self) -> Self {
                    let mut a = self.abs();
                    let mut b = other.abs();
                    while b != 0 {
                        let r = a % b;
                        a = b;
                        b = r;
                    }
                    a
                }

                fn lcm(self, other: Self) -> Self {
                    self / self.gcd(other) * other
                }
            }
        )+
    };
}

impl_signed_integer!(i8, i16, i32, i64, i128, isize);

/// Unsigned marker
macro_rules! impl_unsigned {
    ($($t:ty),+ $(,)?) => {
        $( impl Unsigned for $t {} )+
    };
}

impl_unsigned!(u8, u16, u32, u64, u128, usize);

/// Float
macro_rules! impl_float {
    ($($t:ty),+ $(,)?) => {
        $(
            impl Signed for $t {
                #[inline]
                fn is_negative(self) -> bool {
                    self < 0.0
                }

                #[inline]
                fn abs(self) -> Self {
                    self.abs()
                }
            }

            impl Float for $t {
                #[inline]
                fn nan() -> Self {
                    <$t>::NAN
                }

                #[inline]
                fn infinity() -> Self {
                    <$t>::INFINITY
                }

                #[inline]
                fn neg_infinity() -> Self {
                    <$t>::NEG_INFINITY
                }

                #[inline]
                fn is_nan(self) -> bool {
                    self.is_nan()
                }

                #[inline]
                fn is_infinite(self) -> bool {
                    self.is_infinite()
                }

                #[inline]
                fn is_finite(self) -> bool {
                    self.is_finite()
                }

                #[inline]
                fn sqrt(self) -> Self {
                    self.sqrt()
                }

                #[inline]
                fn powi(self, n: i32) -> Self {
                    self.powi(n)
                }
            }
        )+
    };
}

impl_float!(f32, f64);

macro_rules! impl_approx_eq_float {
    ($t:ty) => {
        impl ApproxEq<$t> for $t {
            #[inline]
            fn approx_eq(&self, rhs: &$t, eps: f64) -> bool {
                if eps < 0.0 {
                    return false;
                }

                if self.is_nan() || rhs.is_nan() {
                    return false;
                }

                if self.is_infinite() || rhs.is_infinite() {
                    return self == rhs;
                }

                if self == rhs {
                    return true;
                }

                let diff = (*self - *rhs).abs();
                let eps = eps as $t;
                if diff <= eps {
                    return true;
                }

                // 相对误差
                let scale = self.abs().max(rhs.abs());
                diff <= eps * scale
            }
        }
    };
}

impl_approx_eq_float!(f32);
impl_approx_eq_float!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflexivity() {
        let x = 1.2345_f64;
        assert!(x.approx_eq(&x, 0.0));
    }

    #[test]
    fn test_symmetry() {
        let a = 1.0_f64;
        let b = 1.0000000001_f64;
        assert!(a.approx_eq(&b, 1e-9));
        assert!(b.approx_eq(&a, 1e-9));
    }

    #[test]
    fn test_absolute_tolerance_near_zero() {
        let a = 1e-10_f64;
        let b = -1e-10_f64;

        // diff = 2e-10 <= eps
        assert!(a.approx_eq(&b, 1e-9));
        assert!(!a.approx_eq(&b, 1e-11));
    }

    #[test]
    fn test_relative_tolerance_large_numbers() {
        let a = 1e100_f64;
        let b = 1.0000000001e100_f64;

        assert!(a.approx_eq(&b, 2e-10));
        assert!(!a.approx_eq(&b, 5e-11));
    }

    #[test]
    fn test_extreme_small_numbers() {
        let a = 1e-30_f32;
        let b = 1.1e-30_f32;

        assert!(a.approx_eq(&b, 1e-1));
    }

    #[test]
    fn test_rounding_boundary() {
        let a = 1.0_f64;
        let b = 1.0000000000000004_f64;

        // diff ≈ 4.44e-16
        assert!(a.approx_eq(&b, 1e-15));
        assert!(!a.approx_eq(&b, 1e-17));
    }
}
