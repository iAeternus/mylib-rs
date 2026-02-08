use std::collections::BinaryHeap;

use algods::collections::fibonacci_heap::FibonacciHeap;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

const SIZES: &[usize] = &[100, 1000, 10000, 50000];

fn bench_push_pop(c: &mut Criterion) {
    let mut group = c.benchmark_group("push_pop");

    for &n in SIZES {
        group.bench_with_input(BenchmarkId::new("BinaryHeap", n), &n, |b, &n| {
            b.iter(|| {
                let mut heap = BinaryHeap::new();
                for i in 0..n {
                    heap.push(i);
                }
                while heap.pop().is_some() {}
            });
        });

        group.bench_with_input(BenchmarkId::new("FibonacciHeap", n), &n, |b, &n| {
            b.iter(|| {
                let mut heap = FibonacciHeap::new();
                for i in 0..n {
                    heap.push(i);
                }
                while heap.pop().is_some() {}
            })
        });
    }

    group.finish();
}

fn bench_decrease_key(c: &mut Criterion) {
    let mut group = c.benchmark_group("decrease_key");
    for &n in SIZES {
        group.bench_with_input(BenchmarkId::new("BinaryHeap", n), &n, |b, &n| {
            b.iter(|| {
                let mut heap = BinaryHeap::new();
                let mut data = Vec::new();
                for i in 0..n {
                    data.push(i + n);
                    heap.push(i + n);
                } // 模拟 decrease-key: 先 pop 再 push
                for i in 0..n {
                    let _ = heap.pop();
                    heap.push(i);
                }
                while heap.pop().is_some() {}
            })
        });
        group.bench_with_input(BenchmarkId::new("FibonacciHeap", n), &n, |b, &n| {
            b.iter(|| {
                let mut heap = FibonacciHeap::new();
                let mut handles = Vec::new();
                for i in 0..n {
                    handles.push(heap.push(i + n));
                }
                for (i, h) in handles.iter().enumerate() {
                    heap.decrease_key(*h, i);
                } 
                while heap.pop().is_some() {}
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_push_pop, bench_decrease_key);
criterion_main!(benches);
