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
                fn is_negative(self) -> bool {
                    self < 0
                }

                #[inline]
                fn abs(self) -> Self {
                    self.abs()
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
