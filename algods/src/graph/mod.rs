//! 图数据结构和图论算法

use std::hash::Hash;
pub mod algo;
mod base;
pub mod graph;

pub trait GraphView {
    /// 节点句柄类型
    type Node: Copy + Eq + Hash + Ord;
    /// 边权类型
    type EdgeWeight: Copy + Ord + std::ops::Add<Output = Self::EdgeWeight>;

    /// 邻居迭代器 (neighbor, weight)
    type Neighbors<'a>: Iterator<Item = (Self::Node, Self::EdgeWeight)>
    where
        Self: 'a;

    /// 返回某节点的出邻居
    fn neighbors(&self, n: Self::Node) -> Self::Neighbors<'_>;

    /// 节点总数
    fn node_count(&self) -> usize;

    /// 是否存在该节点
    fn contains_node(&self, n: Self::Node) -> bool;
}
