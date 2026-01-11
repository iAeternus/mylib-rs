use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{core::Integer, frac::frac::Frac};

impl<T: Integer> Neg for Frac<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            numer: -self.numer,
            denom: self.denom,
        }
    }
}

impl<T: Integer> Add for Frac<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new_unchecked(
            self.numer * rhs.denom + self.denom * rhs.numer,
            self.denom * rhs.denom,
        )
    }
}

impl<T: Integer> Sub for Frac<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new_unchecked(
            self.numer * rhs.denom - self.denom * rhs.numer,
            self.denom * rhs.denom,
        )
    }
}

impl<T: Integer> Mul for Frac<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new_unchecked(self.numer * rhs.numer, self.denom * rhs.denom)
    }
}

impl<T: Integer> Div for Frac<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new_unchecked(self.numer * rhs.denom, self.denom * rhs.numer)
    }
}

impl<T: Integer> AddAssign for Frac<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.numer = self.numer * rhs.denom + self.denom * rhs.numer;
        self.denom = self.denom * rhs.denom;
    }
}

impl<T: Integer> SubAssign for Frac<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.numer = self.numer * rhs.denom - self.denom * rhs.numer;
        self.denom = self.denom * rhs.denom;
    }
}

impl<T: Integer> MulAssign for Frac<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.numer *= rhs.numer;
        self.denom *= rhs.denom;
    }
}

impl<T: Integer> DivAssign for Frac<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.numer *= rhs.denom;
        self.denom *= rhs.numer;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{One, Zero},
        frac::rational::Rational,
    };

    use super::*;

    #[test]
    fn test_neg() {
        let a = Frac::new(1, 2);
        let b = -a;

        assert_eq!(b.numer, -1);
        assert_eq!(b.denom, 2);
    }

    #[test]
    fn test_add() {
        let a = Frac::new(1, 2);
        let b = Frac::new(1, 3);

        let c = a + b;

        // 1/2 + 1/3 = 5/6
        assert_eq!(c.numer, 5);
        assert_eq!(c.denom, 6);
    }

    #[test]
    fn test_sub() {
        let a = Frac::new(1, 2);
        let b = Frac::new(1, 3);

        let c = a - b;

        // 1/2 - 1/3 = 1/6
        assert_eq!(c.numer, 1);
        assert_eq!(c.denom, 6);
    }

    #[test]
    fn test_mul() {
        let a = Frac::new(2, 3);
        let b = Frac::new(3, 4);

        let c = a * b;

        // 2/3 * 3/4 = 1/2
        assert_eq!(c.numer, 1);
        assert_eq!(c.denom, 2);
    }

    #[test]
    fn test_div() {
        let a = Frac::new(2, 3);
        let b = Frac::new(4, 5);

        let c = a / b;

        // 2/3 ÷ 4/5 = 5/6
        assert_eq!(c.numer, 5);
        assert_eq!(c.denom, 6);
    }

    #[test]
    fn add_assign_does_not_reduce() {
        let mut a = Frac::new(1, 2);
        let b = Frac::new(1, 2);

        a += b;

        // 1/2 + 1/2 = 4/4 (未约分)
        assert_eq!(a.numer, 4);
        assert_eq!(a.denom, 4);
    }

    #[test]
    fn mul_assign_does_not_reduce() {
        let mut a = Frac::new(2, 3);
        let b = Frac::new(3, 4);

        a *= b;

        // 2/3 * 3/4 = 6/12 (未约分)
        assert_eq!(a.numer, 6);
        assert_eq!(a.denom, 12);
    }

    #[test]
    fn test_zero() {
        let z = Frac::zero();
        let a = Frac::new(3, 5);

        let r = z + a;

        assert_eq!(r.numer, 3);
        assert_eq!(r.denom, 5);
    }

    #[test]
    fn test_one() {
        let o = Frac::one();
        let a = Frac::new(3, 5);

        let r = a * o;

        assert_eq!(r.numer, 3);
        assert_eq!(r.denom, 5);
    }

    #[test]
    fn sign_is_on_numerator_only() {
        let a = Frac::new(-1, 2);
        let b = Frac::new(1, -2);

        assert_eq!(a.numer, -1);
        assert_eq!(a.denom, 2);

        assert_eq!(b.numer, -1);
        assert_eq!(b.denom, 2);
    }

    #[test]
    fn normalize_always_keeps_denom_positive() {
        let a = Frac::new(1, -2);
        assert!(a.denom > 0);
        assert_eq!(a.numer, -1);
    }

    #[test]
    fn normalize_reduces_fraction() {
        let a = Frac::new(6, 8);
        assert_eq!(a.numer, 3);
        assert_eq!(a.denom, 4);
    }

    #[test]
    fn add_result_is_normalized() {
        let a = Frac::new(1, 6);
        let b = Frac::new(1, 3);

        let c = a + b;

        // 1/6 + 1/3 = 1/2
        assert_eq!(c.numer, 1);
        assert_eq!(c.denom, 2);
    }

    #[test]
    fn mul_result_is_normalized() {
        let a = Frac::new(4, 6);
        let b = Frac::new(3, 8);

        let c = a * b;

        // (4/6)*(3/8) = 1/4
        assert_eq!(c.numer, 1);
        assert_eq!(c.denom, 4);
    }

    #[test]
    fn zero_is_additive_identity() {
        let z = Frac::zero();
        let a = Frac::new(7, 9);

        assert_eq!((a + z).numer, 7);
        assert_eq!((z + a).denom, 9);
    }

    #[test]
    fn zero_mul_is_zero() {
        let z = Frac::zero();
        let a = Frac::new(7, 9);

        let r = z * a;
        assert!(r.is_zero());
    }

    #[test]
    fn one_is_multiplicative_identity() {
        let o = Frac::one();
        let a = Frac::new(5, 7);

        let r = o * a;
        assert_eq!(r.numer, 5);
        assert_eq!(r.denom, 7);
    }

    #[test]
    fn assign_chain_keeps_raw_form() {
        let mut a = Frac::new(1, 2);

        a += Frac::new(1, 2); // 4/4
        a += Frac::new(1, 2); // 12/8

        assert_eq!(a.numer, 12);
        assert_eq!(a.denom, 8);

        a = a.reduce();
        assert_eq!(a.numer, 3);
        assert_eq!(a.denom, 2);
    }
}
