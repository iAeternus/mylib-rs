use std::fmt::Display;

use crate::{
    core::{ApproxEq, Float, Number, Zero},
    vector::Vector,
};

/// 二维向量
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vector2<T: Number> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T: Number> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    /// 叉乘
    pub fn cross(&self, rhs: &Self) -> T {
        self.x * rhs.y - rhs.x * self.y
    }
}

impl<T: Number> Vector for Vector2<T> {
    type Scalar = T;

    fn dim(&self) -> usize {
        2
    }

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T: Number> Zero for Vector2<T> {
    fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T: Number> Default for Vector2<T> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<T: Number> From<T> for Vector2<T> {
    fn from(value: T) -> Self {
        Self::new(value, value)
    }
}

impl<T: Number> From<(T, T)> for Vector2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl<T: Float + ApproxEq> ApproxEq for Vector2<T> {
    fn approx_eq(&self, rhs: &Self, eps: f64) -> bool {
        self.x.approx_eq(&rhs.x, eps) && self.y.approx_eq(&rhs.y, eps)
    }
}

impl<T: Display + Number> Display for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_accessors() {
        let v = Vector2::new(3, 4);
        assert_eq!(v.x(), 3);
        assert_eq!(v.y(), 4);
    }

    #[test]
    fn test_cross_product() {
        let a = Vector2::new(1, 2);
        let b = Vector2::new(3, 4);

        // (1 2) x (3 4) = 1*4 - 3*2 = -2
        assert_eq!(a.cross(&b), -2);
    }

    #[test]
    fn test_cross_parallel_vectors() {
        let a = Vector2::new(2, 2);
        let b = Vector2::new(4, 4);

        assert_eq!(a.cross(&b), 0);
    }

    #[test]
    fn test_dim() {
        let v = Vector2::new(0, 0);
        assert_eq!(v.dim(), 2);
    }

    #[test]
    fn test_dot_product() {
        let a = Vector2::new(1, 3);
        let b = Vector2::new(2, 4);

        assert_eq!(a.dot(&b), 14); // 1*2 + 3*4
    }

    #[test]
    fn test_zero() {
        let z = Vector2::<i32>::zero();
        assert_eq!(z.x(), 0);
        assert_eq!(z.y(), 0);
        assert!(z.is_zero());
    }

    #[test]
    fn test_is_zero_false() {
        let v = Vector2::new(0, 1);
        assert!(!v.is_zero());
    }

    #[test]
    fn test_default() {
        let v: Vector2<i32> = Default::default();
        assert!(v.is_zero());
    }

    #[test]
    fn test_from_scalar() {
        let v: Vector2<i32> = 5.into();
        assert_eq!(v.x(), 5);
        assert_eq!(v.y(), 5);
    }

    #[test]
    fn test_from_tuple() {
        let v: Vector2<i32> = (3, 7).into();
        assert_eq!(v.x(), 3);
        assert_eq!(v.y(), 7);
    }

    #[test]
    fn test_approx_eq_true() {
        let a = Vector2::new(1.000_000_1_f64, 2.0);
        let b = Vector2::new(1.000_000_2_f64, 2.0);

        assert!(a.approx_eq(&b, 1e-6));
    }

    #[test]
    fn test_approx_eq_false() {
        let a = Vector2::new(1.0_f64, 2.0);
        let b = Vector2::new(1.1_f64, 2.0);

        assert!(!a.approx_eq(&b, 1e-6));
    }

    #[test]
    fn test_display() {
        let v = Vector2::new(3, 4);
        let s = format!("{}", v);

        assert_eq!(s, "(3,4)");
    }

    #[test]
    fn test_copy_clone_eq() {
        let v1 = Vector2::new(1, 2);
        let v2 = v1; // Copy
        let v3 = v1.clone(); // Clone

        assert_eq!(v1, v2);
        assert_eq!(v1, v3);
    }
}
