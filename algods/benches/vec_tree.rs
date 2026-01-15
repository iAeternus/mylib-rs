use std::hint::black_box;

use algods::hierarchy::{
    tree::{Hierarchy, Tree},
    vec_tree::VecTree,
};
use criterion::{Criterion, criterion_group, criterion_main};

fn build_large_tree(n: usize) -> VecTree<u32> {
    let mut tree = VecTree::with_root(0);
    let root = tree.root();

    let mut parents = vec![root];
    for i in 1..n {
        let p = parents[i >> 1];
        let c = tree.add_child(p, i as u32);
        parents.push(c);
    }
    tree
}

fn bench_dfs(c: &mut Criterion) {
    let tree = build_large_tree(1_000_000);
    let root = tree.root();

    c.bench_function("VecTree dfs 1M", |b| {
        b.iter(|| {
            let mut sum = 0u64;
            for n in tree.dfs_iter(root) {
                sum += *tree.value(n) as u64
            }
            black_box(sum);
        });
    });
}

fn bench_bfs(c: &mut Criterion) {
    let tree = build_large_tree(1_000_000);
    let root = tree.root();

    c.bench_function("VecTree bfs 1M", |b| {
        b.iter(|| {
            let mut sum = 0u64;
            for n in tree.bfs_iter(root) {
                sum += *tree.value(n) as u64;
            }
            black_box(sum);
        })
    });
}

fn bench_parent_children(c: &mut Criterion) {
    let tree = build_large_tree(1_000_000);
    let root = tree.root();

    c.bench_function("VecTree parent+children", |b| {
        b.iter(|| {
            let mut count = 0;
            for n in tree.dfs_iter(root) {
                if let Some(p) = tree.parent(n) {
                    black_box(p);
                }
                count += tree.children(n).len();
            }
            black_box(count);
        })
    });
}

criterion_group!(benches, bench_dfs, bench_bfs, bench_parent_children);
criterion_main!(benches);
