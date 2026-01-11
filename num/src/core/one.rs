/// 乘法单位元
pub trait One {
    /// 返回乘法单位元
    fn one() -> Self;

    /// 是否为一
    fn is_one(&self) -> bool;
}
