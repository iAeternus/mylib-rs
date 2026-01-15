use std::hint::black_box;

use algods::hierarchy::{hierarchy::Hierarchy, tree::Tree, vec_tree::VecTree};
use criterion::{Criterion, criterion_group, criterion_main};

fn build_large_tree(n: usize) -> VecTree<u32> {
    let mut tree = VecTree::with_root(0);
    let root = tree.root();

    let mut parents = vec![root];
    for i in 1..n {
        let p = parents[i >> 1];
        let c = tree.add_child(p, i as u32).unwrap();
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
            for n in tree.dfs_iter(root).unwrap() {
                sum += *tree.value(n).unwrap() as u64;
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
            for n in tree.bfs_iter(root).unwrap() {
                sum += *tree.value(n).unwrap() as u64;
            }
            black_box(sum);
        });
    });
}

fn bench_parent_children(c: &mut Criterion) {
    let tree = build_large_tree(1_000_000);
    let root = tree.root();

    c.bench_function("VecTree parent+children", |b| {
        b.iter(|| {
            let mut count = 0usize;
            for n in tree.dfs_iter(root).unwrap() {
                let _ = tree.parent(n).unwrap();
                count += tree.children(n).unwrap().len();
            }
            black_box(count);
        });
    });
}

criterion_group!(benches, bench_dfs, bench_bfs, bench_parent_children);
criterion_main!(benches);
