/// 加法单位元
pub trait Zero {
    /// 返回加法单位元
    fn zero() -> Self;

    /// 是否为零
    fn is_zero(&self) -> bool;
}
