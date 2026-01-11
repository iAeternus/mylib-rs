use super::{Number, Signed};

/// IEEE 754 浮点语义
pub trait Float: Number + Signed {
    /// NaN
    fn nan() -> Self;

    /// 正无穷
    fn infinity() -> Self;

    /// 负无穷
    fn neg_infinity() -> Self;

    /// 是否为 NaN
    fn is_nan(self) -> bool;

    /// 是否为无穷
    fn is_infinite(self) -> bool;

    /// 是否为有限值
    fn is_finite(self) -> bool;

    /// 返回平方根
    fn sqrt(self) -> Self;

    /// 返回整数幂
    fn powi(self, n: i32) -> Self;

    /// 返回指数函数 e^x
    fn exp(self) -> Self;

    /// 返回自然对数 ln(x)
    fn ln(self) -> Self;

    /// 返回正弦 sin(x)
    fn sin(self) -> Self;

    /// 返回余弦 cos(x)
    fn cos(self) -> Self;

    /// 返回正切 tan(x)
    fn tan(self) -> Self;

    /// 返回反正弦 asin(x)，结果范围 [-π/2, π/2]
    fn asin(self) -> Self;

    /// 返回反余弦 acos(x)，结果范围 [0, π]
    fn acos(self) -> Self;

    /// 返回反正切 atan(x)，结果范围 [-π/2, π/2]
    fn atan(self) -> Self;

    /// 返回两个浮点数的四象限反正切 atan2(y, x)，结果范围 [-π, π]
    fn atan2(self, other: Self) -> Self;
}

/// 近似相等（Approximate Equality）
///
/// 用于浮点数或其他存在舍入误差的数值类型的“近似比较”，
/// 以避免直接使用 `==` 带来的不稳定行为。
///
/// ## 设计哲学
///
/// 该接口遵循数值分析中的通用约定：
///
/// - **在接近零的区域，使用绝对误差判断**
/// - **在远离零的区域，使用相对误差判断**
///
/// 这种“绝对误差 + 相对误差”的组合策略，
/// 是科学计算、数值分析与工程实践中最常见、最稳定的近似比较方法。
///
/// ## 语义说明
///
/// `self.approx_eq(rhs, eps)` 在满足以下任一条件时返回 `true`：
///
/// - 两个值在 **绝对误差意义下足够接近**
/// - 两个值在 **相对误差意义下足够接近**
///
/// 其中：
///
/// - `eps` 表示**误差容忍度（tolerance）**
/// - 当数值量级较小时（接近零），`eps` 被视为绝对误差
/// - 当数值量级较大时，`eps` 被视为相对误差比例
///
/// 具体判定策略由实现类型决定，但应遵循上述原则。
///
/// ## 关于 `eps`
///
/// - `eps` **必须为非负数**
/// - 当 `eps == 0` 时，行为应等价于“精确相等”（考虑浮点的 `+0.0 / -0.0` 等情况）
/// - 对于负的 `eps`，实现应返回 `false`
///
/// ## 特殊值约定（浮点类型）
///
/// 对于浮点数实现，推荐遵循以下规则：
///
/// - 若任一值为 `NaN`，返回 `false`
/// - 若任一值为无穷大（`±∞`），仅当两者完全相等时返回 `true`
///
/// ## 数学性质
///
/// `ApproxEq` **不是等价关系**，因此不保证：
///
/// - ❌ 传递性（`a ≈ b` 且 `b ≈ c` 不一定推出 `a ≈ c`）
///
/// 但应满足：
///
/// - ✔ 自反性（`a ≈ a`）
/// - ✔ 对称性（`a ≈ b` ⇔ `b ≈ a`）
///
/// ## 示例
///
/// ```rust
/// use num::core::ApproxEq;
///
/// let a = 1.0_f64;
/// let b = 1.0 + 1e-12;
///
/// assert!(a.approx_eq(&b, 1e-9));
/// assert!(!a.approx_eq(&b, 1e-15));
/// ```
pub trait ApproxEq<Rhs = Self> {
    /// 判断 `self` 与 `rhs` 是否在给定误差容忍度 `eps` 下近似相等
    fn approx_eq(&self, rhs: &Rhs, eps: f64) -> bool;
}
