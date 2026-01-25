use criterion::{Criterion, criterion_group, criterion_main};

use std::{collections::BTreeMap, hint::black_box};

use algods::collections::RBTreeMap;

const N: usize = 10_000;

fn build_data() -> Vec<i32> {
    (0..N as i32).collect()
}

fn bench_insert(c: &mut Criterion) {
    let data = build_data();

    let mut group = c.benchmark_group("insert");

    group.bench_function("RBTreeMap", |b| {
        b.iter(|| {
            let mut map = RBTreeMap::new(0, 0);
            for &k in &data {
                map.insert(black_box(k), black_box(k));
            }
        })
    });

    group.bench_function("BTreeMap", |b| {
        b.iter(|| {
            let mut map = BTreeMap::new();
            for &k in &data {
                map.insert(black_box(k), black_box(k));
            }
        })
    });

    group.finish();
}

fn bench_get(c: &mut Criterion) {
    let data = build_data();

    let mut rbt = RBTreeMap::new(0, 0);
    let mut std = BTreeMap::new();

    for &k in &data {
        rbt.insert(k, k);
        std.insert(k, k);
    }

    let mut group = c.benchmark_group("get");

    group.bench_function("RBTreeMap", |b| {
        b.iter(|| {
            for &k in &data {
                black_box(rbt.get(&k));
            }
        })
    });

    group.bench_function("BTreeMap", |b| {
        b.iter(|| {
            for &k in &data {
                black_box(std.get(&k));
            }
        })
    });

    group.finish();
}

fn bench_iter(c: &mut Criterion) {
    let data = build_data();

    let mut rbt = RBTreeMap::new(0, 0);
    let mut std = BTreeMap::new();

    for &k in &data {
        rbt.insert(k, k);
        std.insert(k, k);
    }

    let mut group = c.benchmark_group("iter");

    group.bench_function("RBTreeMap", |b| {
        b.iter(|| {
            for (k, v) in rbt.iter() {
                black_box((k, v));
            }
        })
    });

    group.bench_function("BTreeMap", |b| {
        b.iter(|| {
            for (k, v) in std.iter() {
                black_box((k, v));
            }
        })
    });

    group.finish();
}

fn bench_range(c: &mut Criterion) {
    let data = build_data();

    let mut rbt = RBTreeMap::new(0, 0);
    let mut std = BTreeMap::new();

    for &k in &data {
        rbt.insert(k, k);
        std.insert(k, k);
    }

    let mut group = c.benchmark_group("range");

    let range = (N as i32 / 4)..=(N as i32 / 2);

    group.bench_function("RBTreeMap", |b| {
        b.iter(|| {
            for (k, v) in rbt.range(range.clone()) {
                black_box((k, v));
            }
        })
    });

    group.bench_function("BTreeMap", |b| {
        b.iter(|| {
            for (k, v) in std.range(range.clone()) {
                black_box((k, v));
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_insert, bench_get, bench_iter, bench_range);
criterion_main!(benches);
