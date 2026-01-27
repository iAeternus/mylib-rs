//! 图数据结构和图论算法

use std::hash::Hash;
pub mod algo;
mod base;
pub mod graph;

pub trait GraphView {
    /// 节点句柄类型
    type Node: Copy + Eq + Hash;

    /// 邻居迭代器
    type Neighbors<'a>: Iterator<Item = Self::Node>
    where
        Self: 'a;

    /// 返回某节点的出邻居
    ///
    /// 时间复杂度: O(deg(v))
    fn neighbors(&self, n: Self::Node) -> Self::Neighbors<'_>;

    /// 节点总数（可选，但强烈推荐）
    ///
    /// 时间复杂度: O(1)
    fn node_count(&self) -> usize;

    /// 是否存在该节点（用于算法防御式检查）
    ///
    /// 时间复杂度: O(1)
    fn contains_node(&self, n: Self::Node) -> bool;
}
