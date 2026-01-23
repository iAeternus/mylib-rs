use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use num::big_num::big_integer::big_integer::{BigInteger, Sign};
use std::hint::black_box;

/// 构造指定 digits 数量的 BigInteger
fn make_bigint(digits: usize) -> BigInteger {
    assert!(digits > 0);

    let mut v = Vec::with_capacity(digits);
    for i in 0..digits {
        v.push(((i as u32 + 1) * 12_345_679) % BigInteger::BASE);
    }

    BigInteger {
        sign: Sign::Positive,
        digits: v,
    }
}

fn bench_bigint_mul(c: &mut Criterion) {
    let mut group = c.benchmark_group("bigint_mul");

    let sizes = [
        1usize, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192,
    ];

    for &n in &sizes {
        let a = make_bigint(n);
        let b = make_bigint(n);

        group.bench_with_input(
            BenchmarkId::new("mul", format!("{}-digits", n)),
            &n,
            |bencher, _| {
                bencher.iter(|| {
                    let _ = black_box(&a) * black_box(&b);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    name = bigint;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_secs(2))
        .measurement_time(std::time::Duration::from_secs(5))
        .sample_size(100);
    targets = bench_bigint_mul
);

criterion_main!(bigint);
