use std::collections::VecDeque;

use crate::{
    error::{AlgodsError, AlgodsResult},
    hierarchy::tree::{NodeId, Tree},
};

/// 层次结构算法
pub trait Hierarchy: Tree {
    /// 判断节点是否为根节点
    ///
    /// ## 参数
    /// * `node` - 待检查节点
    ///
    /// ## 返回
    /// - Ok(true)：节点是根
    /// - Ok(false)：节点不是根
    /// - Err(_)：节点无效
    #[inline]
    fn is_root(&self, node: NodeId) -> AlgodsResult<bool> {
        Ok(self.parent(node)?.is_none())
    }

    /// 判断节点是否为叶子节点
    ///
    /// ## 参数
    /// * `node` - 待检查节点
    ///
    /// ## 返回
    /// - Ok(true)：节点没有子节点
    /// - Ok(false)：节点至少有一个子节点
    /// - Err(_)：节点无效
    #[inline]
    fn is_leaf(&self, node: NodeId) -> AlgodsResult<bool> {
        Ok(self.children(node)?.is_empty())
    }

    /// 节点的出度（直接子节点数量）
    #[inline]
    fn degree(&self, node: NodeId) -> AlgodsResult<usize> {
        Ok(self.children(node)?.len())
    }

    /// 节点深度（根节点深度为 0）
    ///
    /// ## 返回
    /// - Ok(depth)：节点深度
    /// - Err(_)：节点无效
    fn depth(&self, mut node: NodeId) -> AlgodsResult<usize> {
        let mut d = 0;
        while let Some(p) = self.parent(node)? {
            d += 1;
            node = p;
        }
        Ok(d)
    }

    /// 迭代器：从父节点开始向上遍历祖先（不包含自身）
    fn ancestors_iter(&self, node: NodeId) -> AlgodsResult<AncestorsIter<'_, Self>>
    where
        Self: Sized,
    {
        if !self.contains(node) {
            return Err(AlgodsError::InvalidNodeId);
        }
        Ok(AncestorsIter {
            tree: self,
            curr: self.parent_unchecked(node),
        })
    }

    /// 深度优先搜索迭代器
    ///
    /// ## 参数
    /// * `start` - 遍历起点
    ///
    /// ## 返回
    /// - Ok(DfsIter)：合法节点起点
    /// - Err(_)：起点无效
    fn dfs_iter(&self, start: NodeId) -> AlgodsResult<DfsIter<'_, Self>>
    where
        Self: Sized,
    {
        if !self.contains(start) {
            return Err(AlgodsError::InvalidNodeId);
        }
        Ok(DfsIter {
            tree: self,
            stack: vec![start],
        })
    }

    /// 广度优先搜索迭代器
    ///
    /// ## 参数
    /// * `start` - 遍历起点
    ///
    /// ## 返回
    /// - Ok(BfsIter)：合法节点起点
    /// - Err(_)：起点无效
    fn bfs_iter(&self, start: NodeId) -> AlgodsResult<BfsIter<'_, Self>>
    where
        Self: Sized,
    {
        if !self.contains(start) {
            return Err(AlgodsError::InvalidNodeId);
        }
        let mut queue = VecDeque::new();
        queue.push_back(start);
        Ok(BfsIter { tree: self, queue })
    }
}

/// 祖先迭代器
pub struct AncestorsIter<'a, T: Tree + ?Sized> {
    tree: &'a T,
    curr: Option<NodeId>,
}

impl<'a, T: Tree> Iterator for AncestorsIter<'a, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.curr?;
        self.curr = self.tree.parent_unchecked(n);
        Some(n)
    }
}

/// 深度优先搜索迭代器
pub struct DfsIter<'a, T: Tree + ?Sized> {
    tree: &'a T,
    stack: Vec<NodeId>,
}

impl<'a, T: Tree> Iterator for DfsIter<'a, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.stack.pop()?;
        for &c in self.tree.children_unchecked(n).iter().rev() {
            self.stack.push(c);
        }
        Some(n)
    }
}

/// 广度优先搜索迭代器
pub struct BfsIter<'a, T: Tree + ?Sized> {
    tree: &'a T,
    queue: VecDeque<NodeId>,
}

impl<'a, T: Tree> Iterator for BfsIter<'a, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.queue.pop_front()?;
        for &c in self.tree.children_unchecked(n) {
            self.queue.push_back(c);
        }
        Some(n)
    }
}

// 自动实现 Hierarchy trait 给所有 Tree
impl<T: Tree> Hierarchy for T {}
