//! 图算法模块

use std::collections::{HashSet, VecDeque};

use crate::graph::GraphView;

pub fn dfs<G: GraphView>(g: &G, start: G::Node) -> Vec<G::Node> {
    fn dfs_inner<G: GraphView>(
        g: &G,
        u: G::Node,
        vis: &mut HashSet<G::Node>,
        out: &mut Vec<G::Node>,
    ) {
        vis.insert(u);
        out.push(u);
        for v in g.neighbors(u) {
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

pub fn bfs<G: GraphView>(g: &G, start: G::Node) -> Vec<G::Node> {
    let mut vis = HashSet::new();
    let mut q = VecDeque::new();
    let mut out = Vec::new();

    vis.insert(start);
    q.push_back(start);

    while let Some(u) = q.pop_front() {
        out.push(u);
        for v in g.neighbors(u) {
            if !vis.contains(&v) {
                q.push_back(v);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_dfs() {
        let g = TestGraph::new(&[(0, &[1, 2]), (1, &[3]), (2, &[]), (3, &[])]);
        let res = dfs(&g, 0);
        assert_eq!(res, vec![0, 1, 3, 2]);
    }

    #[test]
    fn test_bfs() {
        let g = TestGraph::new(&[(0, &[1, 2]), (1, &[3]), (2, &[]), (3, &[])]);
        let res = bfs(&g, 0);
        assert_eq!(res, vec![0, 1, 2, 3]);
    }

    /// 测试用图：邻接表
    struct TestGraph {
        adj: HashMap<usize, Vec<usize>>,
    }

    impl TestGraph {
        fn new(edges: &[(usize, &[usize])]) -> Self {
            let mut adj = HashMap::new();
            for (u, vs) in edges {
                adj.insert(*u, vs.to_vec());
            }
            Self { adj }
        }
    }

    impl GraphView for TestGraph {
        type Node = usize;

        type Neighbors<'a> = std::iter::Copied<std::slice::Iter<'a, usize>>;

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
