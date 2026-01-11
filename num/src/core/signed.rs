/// 有符号数值
pub trait Signed {
    /// 是否为负
    fn is_negative(self) -> bool;

    /// 绝对值
    fn abs(self) -> Self;
}
