//! RSA演示demo，使用BigInteger
//! cargo run -p mylib-rs --example rsa_demo

use std::str::FromStr;

use num::{One, Zero, big_num::big_integer::big_integer::BigInteger};

fn main() {
    println!("RSA演示demo\n");

    println!("1. 生成RSA密钥对");
    let p = BigInteger::from_str("10007").unwrap(); // 质数
    let q = BigInteger::from_str("10009").unwrap(); // 质数
    let n = &p * &q;
    println!(" - 模数 n = p*q = {}", n);

    let phi_n = (&p - &BigInteger::one()) * (&q - &BigInteger::one());
    println!(" - φ(n) = {}", phi_n);

    let e = BigInteger::from_str("65537").unwrap();
    let d = mod_inverse(&e, &phi_n).expect("无法计算模逆元");
    println!(" - 公钥: (e={}, n={})", e, n);
    println!(" - 私钥: (d={}, n={})", d, n);

    println!("\n2. 加密/解密");
    let message = BigInteger::from_str("12345").unwrap();
    println!(" - 明文: {}", message);

    let ciphertext = message.mod_pow(&e, &n).expect("除零错误");
    println!(" - 密文: {}", ciphertext);

    let decrypted = ciphertext.mod_pow(&d, &n).expect("除零错误");
    println!(" - 解密: {}", decrypted);
    println!(" - 成功: {}", decrypted == message);

    println!("\n3. 数字签名");
    let hash = BigInteger::from_str("67890").unwrap();
    println!(" - 哈希: {}", hash);

    let signature = hash.mod_pow(&d, &n).expect("除零错误");
    println!(" - 签名: {}", signature);

    let verified = signature.mod_pow(&e, &n).expect("除零错误");
    println!(" - 验证: {}", verified);
    println!(" - 有效: {}", verified == hash);
}

/// 扩展欧几里得算法
fn extended_gcd(a: &BigInteger, b: &BigInteger) -> (BigInteger, BigInteger, BigInteger) {
    if b.is_zero() {
        return (a.clone(), BigInteger::one(), BigInteger::zero());
    }

    let (g, x1, y1) = extended_gcd(b, &(a % b));
    let x = y1.clone();
    let y = x1 - (a / b) * y1;

    (g, x, y)
}

/// 计算模逆元: `a^(-1) mod m`
fn mod_inverse(a: &BigInteger, m: &BigInteger) -> Option<BigInteger> {
    if m.is_zero() || a.is_zero() {
        return None;
    }

    let (gcd, x, _) = extended_gcd(a, m);

    if gcd != BigInteger::one() {
        return None;
    }

    let result = (x % m + m) % m;
    Some(result)
}
