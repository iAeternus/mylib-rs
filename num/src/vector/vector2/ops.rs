use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{
    core::{Number, Signed},
    vector::vector2::vector2::Vector2,
};

impl<T: Number + Signed> Neg for Vector2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl<T: Number> Add for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Number> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Number> Mul<T> for Vector2<T> {
    type Output = Self;

    /// 向量乘标量
    fn mul(self, scalar: T) -> Self::Output {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

/// 标量乘向量
macro_rules! impl_left_mul_for_scalar {
    ($($t:ty),+ $(,)?) => {
        $(
            impl Mul<Vector2<$t>> for $t {
                type Output = Vector2<$t>;

                fn mul(self, vector: Vector2<$t>) -> Self::Output {
                    Vector2::new(self * vector.x, self * vector.y)
                }
            }
        )*
    };
}

impl_left_mul_for_scalar!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64,
);

impl<T: Number> AddAssign for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Number> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Number> MulAssign<T> for Vector2<T> {
    /// 向量乘标量
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neg_i32() {
        let v = Vector2::new(3, -4);
        let r = -v;
        assert_eq!(r.x, -3);
        assert_eq!(r.y, 4);
    }

    #[test]
    fn test_neg_f64() {
        let v = Vector2::new(1.5, -2.5);
        let r = -v;
        assert_eq!(r.x, -1.5);
        assert_eq!(r.y, 2.5);
    }

    #[test]
    fn test_add() {
        let a = Vector2::new(1, 2);
        let b = Vector2::new(3, 4);
        let r = a + b;

        assert_eq!(r.x, 4);
        assert_eq!(r.y, 6);
    }

    #[test]
    fn test_sub() {
        let a = Vector2::new(5, 3);
        let b = Vector2::new(2, 1);
        let r = a - b;

        assert_eq!(r.x, 3);
        assert_eq!(r.y, 2);
    }

    #[test]
    fn test_mul_scalar_right() {
        let v = Vector2::new(2, -3);
        let r = v * 4;

        assert_eq!(r.x, 8);
        assert_eq!(r.y, -12);
    }

    #[test]
    fn test_mul_scalar_right_f64() {
        let v = Vector2::new(1.5, -2.0);
        let r = v * 2.0;

        assert_eq!(r.x, 3.0);
        assert_eq!(r.y, -4.0);
    }

    #[test]
    fn test_mul_scalar_left() {
        let v = Vector2::new(3, 4);
        let r = 2_i32 * v;

        assert_eq!(r.x, 6);
        assert_eq!(r.y, 8);
    }

    #[test]
    fn test_mul_scalar_left_f64() {
        let v = Vector2::new(0.5, -1.5);
        let r = 2_f64 * v;

        assert_eq!(r.x, 1.0);
        assert_eq!(r.y, -3.0);
    }

    #[test]
    fn test_add_assign() {
        let mut v = Vector2::new(1, 1);
        v += Vector2::new(2, 3);

        assert_eq!(v.x, 3);
        assert_eq!(v.y, 4);
    }

    #[test]
    fn test_sub_assign() {
        let mut v = Vector2::new(5, 5);
        v -= Vector2::new(2, 1);

        assert_eq!(v.x, 3);
        assert_eq!(v.y, 4);
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vector2::new(2, -3);
        v *= 3;

        assert_eq!(v.x, 6);
        assert_eq!(v.y, -9);
    }

    #[test]
    fn test_mul_assign_f64() {
        let mut v = Vector2::new(1.5, -2.0);
        v *= 2.0;

        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, -4.0);
    }
}
