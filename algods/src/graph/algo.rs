//! 图算法模块
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use crate::graph::GraphBase;

pub fn dfs<G: GraphBase>(g: &G, start: G::Node) -> Vec<G::Node> {
    fn dfs_inner<G: GraphBase>(
        g: &G,
        u: G::Node,
        vis: &mut HashSet<G::Node>,
        out: &mut Vec<G::Node>,
    ) {
        vis.insert(u);
        out.push(u);
        for (v, _) in g.neighbors(u) {
            if !vis.contains(&v) {
                dfs_inner(g, v, vis, out);
            }
        }
    }

    let mut vis = HashSet::new();
    let mut out = Vec::new();
    dfs_inner(g, start, &mut vis, &mut out);
    out
}

pub fn bfs<G: GraphBase>(g: &G, start: G::Node) -> Vec<G::Node> {
    let mut vis = HashSet::new();
    let mut q = VecDeque::new();
    let mut out = Vec::new();

    vis.insert(start);
    q.push_back(start);

    while let Some(u) = q.pop_front() {
        out.push(u);
        for (v, _) in g.neighbors(u) {
            if !vis.contains(&v) {
                q.push_back(v);
                vis.insert(v);
            }
        }
    }
    out
}

pub fn dijkstra<G>(g: &G, from: G::Node, to: G::Node) -> Option<G::EdgeWeight>
where
    G: GraphBase,
    G::EdgeWeight: From<u8>,
{
    let mut dis: HashMap<G::Node, G::EdgeWeight> = HashMap::new();
    let mut heap = BinaryHeap::new();

    let zero = 0u8.into();
    dis.insert(from, zero);
    heap.push((Reverse(zero), from));

    while let Some((Reverse(d), u)) = heap.pop() {
        if let Some(&best) = dis.get(&u) {
            if d > best {
                continue;
            }
        }

        if u == to {
            return Some(d);
        }

        for (v, w) in g.neighbors(u) {
            let nd = w + d;
            let relax = match dis.get(&v) {
                Some(&old) => nd < old,
                None => true,
            };

            if relax {
                dis.insert(v, nd);
                heap.push((Reverse(nd), v));
            }
        }
    }

    None // 不可达
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_dfs() {
        let g = TestGraph::new(&[(0, &[(1, 1), (2, 1)]), (1, &[(3, 1)]), (2, &[]), (3, &[])]);
        let res = dfs(&g, 0);
        assert_eq!(res, vec![0, 1, 3, 2]);
    }

    #[test]
    fn test_bfs() {
        let g = TestGraph::new(&[(0, &[(1, 1), (2, 1)]), (1, &[(3, 1)]), (2, &[]), (3, &[])]);
        let res = bfs(&g, 0);
        assert_eq!(res, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_bfs_cycle_no_duplicate_visit() {
        // 0 -> 1,2
        // 1 -> 2,3
        // 2 -> 1,3
        // 3 -> 0
        // 图中既有环，也有对同一节点(3)的多入边
        let g = TestGraph::new(&[
            (0, &[(1, 1), (2, 1)]),
            (1, &[(2, 1), (3, 1)]),
            (2, &[(1, 1), (3, 1)]),
            (3, &[(0, 1)]),
        ]);

        let res = bfs(&g, 0);
        assert_eq!(res, vec![0, 1, 2, 3]);

        // 验证不会重复访问
        let unique: std::collections::HashSet<_> = res.iter().copied().collect();
        assert_eq!(unique.len(), res.len());
    }

    #[test]
    fn test_dijkstra() {
        // 图结构：
        // 0 -> 1 (2)
        // 0 -> 2 (5)
        // 1 -> 2 (1)
        // 1 -> 3 (3)
        // 2 -> 3 (1)
        //
        // 最短路：0 -> 1 -> 2 -> 3 = 2 + 1 + 1 = 4
        let g = TestGraph::new(&[
            (0, &[(1, 2), (2, 5)]),
            (1, &[(2, 1), (3, 3)]),
            (2, &[(3, 1)]),
            (3, &[]),
        ]);

        let dist = dijkstra(&g, 0, 3);
        assert_eq!(dist, Some(4));
    }

    struct TestGraph {
        adj: HashMap<usize, Vec<(usize, usize)>>, // (to, weight)
    }

    impl TestGraph {
        fn new(edges: &[(usize, &[(usize, usize)])]) -> Self {
            let mut adj = HashMap::new();
            for (u, vs) in edges {
                adj.insert(*u, vs.to_vec());
            }
            Self { adj }
        }
    }

    impl GraphBase for TestGraph {
        type Node = usize;
        type EdgeWeight = usize;

        type Neighbors<'a>
            = std::iter::Copied<std::slice::Iter<'a, (usize, usize)>>
        where
            Self: 'a;

        fn neighbors(&self, n: usize) -> Self::Neighbors<'_> {
            self.adj
                .get(&n)
                .map(|v| v.iter().copied())
                .unwrap_or_else(|| [].iter().copied())
        }

        fn node_count(&self) -> usize {
            self.adj.len()
        }

        fn contains_node(&self, n: usize) -> bool {
            self.adj.contains_key(&n)
        }
    }
}
