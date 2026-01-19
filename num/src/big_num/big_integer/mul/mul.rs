use crate::big_num::big_integer::big_integer::BigInteger;

pub trait BigIntMul {
    /// 高精度整数乘法
    fn mul(lhs: &BigInteger, rhs: &BigInteger) -> BigInteger;

    /// 该算法适用的数字块上限
    fn limit() -> usize;
}
