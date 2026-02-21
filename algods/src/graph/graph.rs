use std::{hash::Hash, marker::PhantomData, usize};

use crate::graph::{
    GraphBase,
    base::{Directed, Direction, EdgeIndex, EdgeType, NodeIndex},
};

/// 节点
pub struct Node<N, Idx> {
    pub weight: N,
    /// [first_outgoing, first_incoming]
    next: [EdgeIndex<Idx>; 2],
}

/// 边
pub struct Edge<E, Idx> {
    pub weight: E,
    /// [source, target]
    node: [NodeIndex<Idx>; 2],
    /// [next_outgoing_of_source, next_incoming_of_target]
    next: [EdgeIndex<Idx>; 2],
}

/// 图（侵入式邻接表）
///
/// N: 节点权重类型
/// E: 边权重类型
/// Ty: Directed / Undirected
/// Idx: 索引类型
pub struct Graph<N, E, Ty = Directed, Idx = usize>
where
    Idx: Copy + PartialEq + From<usize> + Into<usize>,
{
    nodes: Vec<Node<N, Idx>>,
    edges: Vec<Edge<E, Idx>>,
    _boo: PhantomData<Ty>,
}

/// 边引用（供迭代器返回）
pub struct EdgeReference<'a, E, Ix> {
    pub index: EdgeIndex<Ix>,
    pub weight: &'a E,
    pub node: [NodeIndex<Ix>; 2],
}

/// 边迭代器
pub struct EdgeIter<'a, N, E, Ty, Idx>
where
    Idx: Copy + PartialEq + From<usize> + Into<usize>,
{
    graph: &'a Graph<N, E, Ty, Idx>,
    direction: usize, // 0 = outgoing, 1 = incoming
    curr: EdgeIndex<Idx>,
}

/// 邻居迭代器
pub struct Neighbors<'a, N, E, Ty, Idx>
where
    Idx: Copy + PartialEq + From<usize> + Into<usize>,
{
    inner: EdgeIter<'a, N, E, Ty, Idx>,
}

impl<N, E, Ty, Idx> Graph<N, E, Ty, Idx>
where
    Ty: EdgeType,
    Idx: Copy + PartialEq + From<usize> + Into<usize>,
    E: Clone,
{
    /// 创建空图
    ///
    /// 时间复杂度: O(1)
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            _boo: PhantomData,
        }
    }

    /// 返回一条边的 (source, target)
    ///
    /// 时间复杂度: O(1)
    pub fn edge_endpoints(&self, e: EdgeIndex<Idx>) -> (NodeIndex<Idx>, NodeIndex<Idx>) {
        let edge = &self.edges[e.0.into()];
        (edge.node[0], edge.node[1])
    }

    /// 添加一个节点，返回其索引
    ///
    /// 时间复杂度: O(1)
    pub fn add_node(&mut self, weight: N) -> NodeIndex<Idx> {
        let node = Node {
            weight,
            next: [EdgeIndex::end(), EdgeIndex::end()],
        };
        self.nodes.push(node);
        NodeIndex(Idx::from((self.nodes.len() - 1) as usize))
    }

    /// 添加一条边 a -> b（有向图）
    /// 若是无向图，则自动补一条 b -> a 的反向边
    ///
    /// 时间复杂度: O(1)
    pub fn add_edge(&mut self, a: NodeIndex<Idx>, b: NodeIndex<Idx>, weight: E) -> EdgeIndex<Idx> {
        let edge_idx = EdgeIndex(Idx::from(self.edges.len() as usize));

        let edge = Edge {
            weight: weight.clone(),
            node: [a, b],
            next: [EdgeIndex::end(), EdgeIndex::end()],
        };
        self.edges.push(edge);

        // 插入到源节点的出边链表头部
        self.link_edge(
            a,
            edge_idx,
            Direction::Outgoing as usize,
            Direction::Outgoing as usize,
        );
        // 插入到目标节点的入边链表头部
        self.link_edge(
            b,
            edge_idx,
            Direction::Incoming as usize,
            Direction::Incoming as usize,
        );

        // 若是无向图，补一条反向边
        if !Ty::DIRECTED {
            let rev_idx = EdgeIndex(Idx::from(self.edges.len() as usize));

            let rev_edge = Edge {
                weight,
                node: [b, a],
                next: [EdgeIndex::end(), EdgeIndex::end()],
            };
            self.edges.push(rev_edge);

            // b -> a 出边
            self.link_edge(
                b,
                rev_idx,
                Direction::Outgoing as usize,
                Direction::Outgoing as usize,
            );
            // a <- b 入边
            self.link_edge(
                a,
                rev_idx,
                Direction::Incoming as usize,
                Direction::Incoming as usize,
            );
        }

        edge_idx
    }

    /// 头插边到链表
    fn link_edge(
        &mut self,
        node_idx: NodeIndex<Idx>,
        edge_idx: EdgeIndex<Idx>,
        node_dir: usize,
        edge_dir: usize,
    ) {
        let node = &mut self.nodes[node_idx.0.into()];
        let next_edge = node.next[node_dir];
        node.next[node_dir] = edge_idx;
        self.edges[edge_idx.0.into()].next[edge_dir] = next_edge;
    }
}

impl<N, E, Ty, Idx> Graph<N, E, Ty, Idx>
where
    Ty: EdgeType,
    Idx: Copy + PartialEq + From<usize> + Into<usize>,
{
    /// 返回指定节点在某个方向上的边迭代器
    ///
    /// 时间复杂度: O(1)
    pub fn edges_directed(
        &self,
        node: NodeIndex<Idx>,
        dir: Direction,
    ) -> EdgeIter<'_, N, E, Ty, Idx> {
        let start = self.nodes[node.0.into()].next[dir as usize];
        EdgeIter {
            graph: self,
            direction: dir as usize,
            curr: start,
        }
    }

    /// 返回节点的所有出邻居（对有向图）
    /// 无向图时等价于所有相邻节点
    ///
    /// 时间复杂度: O(deg(v))
    pub fn neighbors(&self, node: NodeIndex<Idx>) -> Neighbors<'_, N, E, Ty, Idx> {
        Neighbors {
            inner: self.edges_directed(node, Direction::Outgoing),
        }
    }
}

impl<N, E, Ty, Idx> GraphBase for Graph<N, E, Ty, Idx>
where
    Ty: EdgeType,
    Idx: Copy + Eq + Hash + From<usize> + Into<usize> + Ord,
    E: Copy + Ord + std::ops::Add<Output = E>,
{
    type Node = NodeIndex<Idx>;
    type EdgeWeight = E;

    type Neighbors<'a>
        = Neighbors<'a, N, E, Ty, Idx>
    where
        Self: 'a;

    fn neighbors(&self, n: Self::Node) -> Self::Neighbors<'_> {
        Neighbors {
            inner: self.edges_directed(n, Direction::Outgoing),
        }
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn contains_node(&self, n: Self::Node) -> bool {
        n.0.into() < self.nodes.len()
    }
}

impl<'a, N, E, Ty, Idx> Iterator for EdgeIter<'a, N, E, Ty, Idx>
where
    Ty: EdgeType,
    Idx: Copy + PartialEq + From<usize> + Into<usize>,
{
    type Item = EdgeReference<'a, E, Idx>;

    /// 迭代器前进到下一条边
    ///
    /// 单步时间复杂度: O(1)  
    /// 总遍历复杂度: O(deg(v))
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == EdgeIndex::end() {
            return None;
        }

        let edge_idx = self.curr;
        let edge = &self.graph.edges[edge_idx.0.into()];

        // 沿链表前进
        self.curr = edge.next[self.direction];

        Some(EdgeReference {
            index: edge_idx,
            weight: &edge.weight,
            node: edge.node,
        })
    }
}

impl<'a, N, E, Ty, Idx> Iterator for Neighbors<'a, N, E, Ty, Idx>
where
    Ty: EdgeType,
    Idx: Copy + PartialEq + From<usize> + Into<usize>,
    E: Copy,
{
    type Item = (NodeIndex<Idx>, E);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|edge| (edge.node[1], *edge.weight))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::base::{Directed, Direction, Undirected};

    #[test]
    fn test_add_node() {
        let mut g: Graph<i32, i32, Directed> = Graph::new();
        let a = g.add_node(10);
        let b = g.add_node(20);

        assert_eq!(a.index(), 0);
        assert_eq!(b.index(), 1);
        assert_eq!(g.nodes.len(), 2);
    }

    #[test]
    fn test_add_edge_directed() {
        let mut g: Graph<&str, i32, Directed> = Graph::new();
        let a = g.add_node("a");
        let b = g.add_node("b");

        let e = g.add_edge(a, b, 5);

        // 边数量
        assert_eq!(g.edges.len(), 1);

        let edge = &g.edges[e.index()];
        assert_eq!(edge.weight, 5);
        assert_eq!(edge.node[0], a);
        assert_eq!(edge.node[1], b);
    }

    #[test]
    fn test_outgoing_edges() {
        let mut g: Graph<&str, i32, Directed> = Graph::new();
        let a = g.add_node("a");
        let b = g.add_node("b");
        let c = g.add_node("c");

        g.add_edge(a, b, 1);
        g.add_edge(a, c, 2);

        let mut targets = Vec::new();
        for e in g.edges_directed(a, Direction::Outgoing) {
            targets.push(e.node[1]);
        }

        targets.sort_by_key(|x| x.index());
        assert_eq!(targets, vec![b, c]);
    }

    #[test]
    fn test_incoming_edges() {
        let mut g: Graph<&str, i32, Directed> = Graph::new();
        let a = g.add_node("a");
        let b = g.add_node("b");
        let c = g.add_node("c");

        g.add_edge(a, c, 1);
        g.add_edge(b, c, 2);

        let mut sources = Vec::new();
        for e in g.edges_directed(c, Direction::Incoming) {
            sources.push(e.node[0]);
        }

        sources.sort_by_key(|x| x.index());
        assert_eq!(sources, vec![a, b]);
    }

    #[test]
    fn test_undirected_edge() {
        let mut g: Graph<&str, i32, Undirected> = Graph::new();
        let a = g.add_node("a");
        let b = g.add_node("b");

        g.add_edge(a, b, 1);

        // 无向图应该有两条边
        assert_eq!(g.edges.len(), 2);

        let mut a_neighbors = Vec::new();
        for e in g.edges_directed(a, Direction::Outgoing) {
            a_neighbors.push(e.node[1]);
        }

        let mut b_neighbors = Vec::new();
        for e in g.edges_directed(b, Direction::Outgoing) {
            b_neighbors.push(e.node[1]);
        }

        assert_eq!(a_neighbors, vec![b]);
        assert_eq!(b_neighbors, vec![a]);
    }

    #[test]
    fn test_empty_graph() {
        let g: Graph<i32, i32, Directed> = Graph::new();
        assert_eq!(g.nodes.len(), 0);
        assert_eq!(g.edges.len(), 0);
    }

    #[test]
    fn test_single_node_no_edges() {
        let mut g: Graph<i32, i32, Directed> = Graph::new();
        let a = g.add_node(1);

        let edges: Vec<_> = g.edges_directed(a, Direction::Outgoing).collect();
        assert!(edges.is_empty());
    }
}
