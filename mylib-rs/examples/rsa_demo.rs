//! RSA演示demo，使用BigInteger
//! cargo run -p mylib-rs --example rsa_demo

#[cfg(not(feature = "core"))]
mod demo {
    use std::str::FromStr;

    use num::{One, Zero, big_num::big_integer::big_integer::BigInteger};

    pub fn run() {
        println!("RSA demo\n");

        println!("1. Generate RSA key pair");
        let p = BigInteger::from_str("10007").expect("valid prime");
        let q = BigInteger::from_str("10009").expect("valid prime");
        let n = &p * &q;
        println!(" - modulus n = p*q = {}", n);

        let phi_n = (&p - &BigInteger::one()) * (&q - &BigInteger::one());
        println!(" - phi(n) = {}", phi_n);

        let e = BigInteger::from_str("65537").expect("valid exponent");
        let d = mod_inverse(&e, &phi_n).expect("mod inverse exists");
        println!(" - public key: (e={}, n={})", e, n);
        println!(" - private key: (d={}, n={})", d, n);

        println!("\n2. Encrypt/decrypt");
        let message = BigInteger::from_str("12345").expect("valid message");
        println!(" - plaintext: {}", message);

        let ciphertext = message.mod_pow(&e, &n).expect("mod pow should work");
        println!(" - ciphertext: {}", ciphertext);

        let decrypted = ciphertext.mod_pow(&d, &n).expect("mod pow should work");
        println!(" - decrypted: {}", decrypted);
        println!(" - success: {}", decrypted == message);

        println!("\n3. Signature");
        let hash = BigInteger::from_str("67890").expect("valid hash");
        println!(" - hash: {}", hash);

        let signature = hash.mod_pow(&d, &n).expect("mod pow should work");
        println!(" - signature: {}", signature);

        let verified = signature.mod_pow(&e, &n).expect("mod pow should work");
        println!(" - verify: {}", verified);
        println!(" - valid: {}", verified == hash);
    }

    fn extended_gcd(a: &BigInteger, b: &BigInteger) -> (BigInteger, BigInteger, BigInteger) {
        if b.is_zero() {
            return (a.clone(), BigInteger::one(), BigInteger::zero());
        }

        let (g, x1, y1) = extended_gcd(b, &(a % b));
        let x = y1.clone();
        let y = x1 - (a / b) * y1;

        (g, x, y)
    }

    fn mod_inverse(a: &BigInteger, m: &BigInteger) -> Option<BigInteger> {
        if m.is_zero() || a.is_zero() {
            return None;
        }

        let (gcd, x, _) = extended_gcd(a, m);
        if gcd != BigInteger::one() {
            return None;
        }

        Some((x % m + m) % m)
    }
}

#[cfg(not(feature = "core"))]
fn main() {
    demo::run();
}

#[cfg(feature = "core")]
fn main() {}
