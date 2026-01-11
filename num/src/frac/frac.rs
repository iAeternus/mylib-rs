use crate::{
    core::{Integer, One, Signed, Zero},
    error::NumResult,
    frac::rational::Rational,
};

/// 分数
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Frac<T: Integer> {
    /// 分子
    pub(crate) numer: T,
    /// 分母，保证 > 0
    pub(crate) denom: T,
}

impl<T: Integer> Frac<T> {
    pub fn new(numer: T, denom: T) -> Self {
        assert!(!denom.is_zero(), "denominator must not be zero");
        Self::new_unchecked(numer, denom)
    }

    pub fn try_new(numer: T, denom: T) -> NumResult<Self> {
        if denom.is_zero() {
            return Err(crate::error::NumError::DivisionByZero);
        }
        Ok(Self::new_unchecked(numer, denom))
    }

    /// 调用者必须保证分母非 0，否则行为未定义（panic）
    #[inline(always)]
    pub(crate) fn new_unchecked(numer: T, denom: T) -> Self {
        let mut f = Self { numer, denom };
        f.normalize();
        f
    }

    fn normalize(&mut self) {
        if self.denom.is_negative() {
            self.numer = -self.numer;
            self.denom = -self.denom;
        }

        let g = self.numer.gcd(self.denom);
        if !g.is_one() {
            self.numer /= g;
            self.denom /= g;
        }
    }
}

impl<T: Integer> Rational for Frac<T> {
    type Int = T;

    #[inline]
    fn numer(&self) -> T {
        self.numer
    }

    #[inline]
    fn denom(&self) -> T {
        self.denom
    }

    #[inline]
    fn reduce(mut self) -> Self {
        self.normalize();
        self
    }
}

impl<T: Integer> Signed for Frac<T> {
    #[inline]
    fn abs(self) -> Self {
        Self {
            numer: self.numer.abs(),
            denom: self.denom,
        }
    }

    #[inline]
    fn is_negative(self) -> bool {
        self.numer.is_negative()
    }
}

impl<T: Integer> Zero for Frac<T> {
    #[inline]
    fn zero() -> Self {
        Self {
            numer: T::zero(),
            denom: T::one(),
        }
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.numer.is_zero()
    }
}

impl<T: Integer> One for Frac<T> {
    fn one() -> Self {
        Self {
            numer: T::one(),
            denom: T::one(),
        }
    }

    fn is_one(&self) -> bool {
        self.numer.is_one()
    }
}
