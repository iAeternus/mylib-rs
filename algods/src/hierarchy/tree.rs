use std::{collections::VecDeque, fmt};

/// 树节点 ID
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId {
    /// arena 下标
    pub(crate) index: usize,
    /// 防止悬垂/复用
    pub(crate) generation: u32,
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeId({}, gen={})", self.index, self.generation)
    }
}

/// 有根树接口
pub trait Tree {
    type Value;

    /// 获取根节点
    fn root(&self) -> NodeId;

    /// 节点是否仍然有效
    fn contains(&self, node: NodeId) -> bool;

    /// 父节点（根返回 None）
    fn parent(&self, node: NodeId) -> Option<NodeId>;

    /// 直接子节点
    fn children(&self, node: NodeId) -> &[NodeId];

    /// 不可变访问值
    fn value(&self, node: NodeId) -> &Self::Value;

    /// 可变访问值
    fn value_mut(&mut self, node: NodeId) -> &mut Self::Value;

    /// 添加子节点
    fn add_child(&mut self, parent: NodeId, value: Self::Value) -> NodeId;

    /// 删除节点及其整个子树
    ///
    /// - 若 node 不存在：**no-op**
    /// - 若 node 是根：**panic**
    fn remove_subtree(&mut self, node: NodeId);

    /// 当前存活节点数（不含已删除）
    fn size(&self) -> usize;
}

/// 层次结构算法
pub trait Hierarchy: Tree {
    /// 是否为根节点
    #[inline]
    fn is_root(&self, node: NodeId) -> bool {
        self.parent(node).is_none()
    }

    /// 是否为叶子节点
    #[inline]
    fn is_leaf(&self, node: NodeId) -> bool {
        self.children(node).is_empty()
    }

    /// 出度
    #[inline]
    fn degree(&self, node: NodeId) -> usize {
        self.children(node).len()
    }

    /// 深度（root = 0）
    fn depth(&self, mut node: NodeId) -> usize {
        let mut d = 0;
        while let Some(p) = self.parent(node) {
            d += 1;
            node = p;
        }
        d
    }

    /// 祖先（不含自身）
    fn ancestors_iter(&self, node: NodeId) -> AncestorsIter<'_, Self>
    where
        Self: Sized,
    {
        AncestorsIter {
            tree: self,
            curr: self.parent(node),
        }
    }

    /// 祖先（包含自身）
    fn ancestors_inclusive_iter(&self, node: NodeId) -> AncestorsIter<'_, Self>
    where
        Self: Sized,
    {
        AncestorsIter {
            tree: self,
            curr: Some(node),
        }
    }

    /// 从根到当前节点的路径
    fn path_from_root(&self, node: NodeId) -> Vec<NodeId>
    where
        Self: Sized,
    {
        let mut v: Vec<_> = self.ancestors_inclusive_iter(node).collect();
        v.reverse();
        v
    }

    /// 兄弟节点（不含自身）
    fn siblings(&self, node: NodeId) -> Vec<NodeId> {
        self.parent(node)
            .map(|p| {
                self.children(p)
                    .iter()
                    .copied()
                    .filter(|&n| n != node)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 深度优先搜索
    fn dfs_iter(&self, start: NodeId) -> DfsIter<'_, Self>
    where
        Self: Sized,
    {
        DfsIter {
            tree: self,
            stack: vec![start],
        }
    }

    /// 广度优先搜索
    fn bfs_iter(&self, start: NodeId) -> BfsIter<'_, Self>
    where
        Self: Sized,
    {
        let mut q = VecDeque::new();
        q.push_back(start);
        BfsIter {
            tree: self,
            queue: q,
        }
    }
}

impl<T: Tree> Hierarchy for T {}

/// 祖先迭代器
pub struct AncestorsIter<'a, T: Tree + ?Sized> {
    tree: &'a T,
    curr: Option<NodeId>,
}

impl<'a, T: Tree> Iterator for AncestorsIter<'a, T> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.curr?;
        self.curr = self.tree.parent(n);
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
        for &c in self.tree.children(n).iter().rev() {
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
        for &c in self.tree.children(n) {
            self.queue.push_back(c);
        }
        Some(n)
    }
}
