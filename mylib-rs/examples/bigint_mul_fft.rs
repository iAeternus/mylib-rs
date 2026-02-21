//! 使用快速傅里叶变换的大整数乘法
//! cargo run -p mylib-rs --example bigint_mul_fft

#[cfg(not(feature = "core"))]
mod demo {
    use std::f64::consts::PI;

    use num::{
        One, Zero,
        complex::{Complex, ComplexNumber},
    };

    pub fn run() {
        let a = "123456789012345678901234567890";
        let b = "987654321098765432109876543210";

        let c = multiply(a, b);

        println!("a = {}", a);
        println!("b = {}", b);
        println!("a * b = {}", c);
    }

    fn fft(a: &mut [Complex<f64>], invert: bool) {
        let n = a.len();
        if n == 1 {
            return;
        }

        let mut even = Vec::with_capacity(n / 2);
        let mut odd = Vec::with_capacity(n / 2);

        for i in 0..n / 2 {
            even.push(a[2 * i]);
            odd.push(a[2 * i + 1]);
        }

        fft(&mut even, invert);
        fft(&mut odd, invert);

        let ang = 2.0 * PI / n as f64 * if invert { -1.0 } else { 1.0 };
        let wn = Complex::new(ang.cos(), ang.sin());
        let mut w: Complex<f64> = Complex::one();

        for i in 0..n / 2 {
            let u = even[i];
            let v = w * odd[i];

            a[i] = u + v;
            a[i + n / 2] = u - v;

            w = w * wn;
        }

        if invert {
            for x in a.iter_mut() {
                *x = *x / Complex::from(2.0);
            }
        }
    }

    fn multiply(a: &str, b: &str) -> String {
        let mut fa: Vec<_> = a
            .bytes()
            .rev()
            .map(|c| Complex::from((c - b'0') as f64))
            .collect();

        let mut fb: Vec<_> = b
            .bytes()
            .rev()
            .map(|c| Complex::from((c - b'0') as f64))
            .collect();

        let mut n = 1;
        while n < fa.len() + fb.len() {
            n <<= 1;
        }

        fa.resize(n, Complex::zero());
        fb.resize(n, Complex::zero());

        fft(&mut fa, false);
        fft(&mut fb, false);

        for i in 0..n {
            fa[i] = fa[i] * fb[i];
        }

        fft(&mut fa, true);

        let mut result = vec![0i64; n];
        let mut carry = 0i64;

        for i in 0..n {
            let t = fa[i].re().round() as i64 + carry;
            result[i] = t % 10;
            carry = t / 10;
        }

        while carry > 0 {
            result.push(carry % 10);
            carry /= 10;
        }

        while result.len() > 1 && *result.last().unwrap() == 0 {
            result.pop();
        }

        result
            .iter()
            .rev()
            .map(|d| (d + b'0' as i64) as u8 as char)
            .collect()
    }
}

#[cfg(not(feature = "core"))]
fn main() {
    demo::run();
}

#[cfg(feature = "core")]
fn main() {}
