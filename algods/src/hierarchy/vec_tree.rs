use smallvec::SmallVec;

use crate::{
    error::{AlgodsError, AlgodsResult},
    hierarchy::tree::{NodeId, Tree},
};

const INLINE_CHILDREN: usize = 4;

/// 数组实现的有根树（VecTree）
///
/// 适用于读多写少场景。底层使用 Vec 存储节点值和关系，
/// 子节点使用 SmallVec 内联存储以减少小树的堆分配。
pub struct VecTree<T> {
    /// 节点值数组
    values: Vec<T>,
    /// 父节点数组（根节点为 None）
    parents: Vec<Option<NodeId>>,
    /// 子节点数组，每个节点存储自己的直接子节点
    children: Vec<SmallVec<[NodeId; INLINE_CHILDREN]>>,
    /// 节点世代，用于防止悬垂引用
    generations: Vec<u32>,
    /// 当前存活节点数量
    alive_count: usize,
}

impl<T> VecTree<T> {
    /// 创建一棵带根节点的树
    ///
    /// # 参数
    /// * `value` - 根节点的值
    ///
    /// # 返回
    /// 新创建的 VecTree 实例，根节点已经存在
    pub fn with_root(value: T) -> Self {
        let mut tree = Self {
            values: Vec::new(),
            parents: Vec::new(),
            children: Vec::new(),
            generations: Vec::new(),
            alive_count: 0,
        };
        tree.alloc_node(value, None);
        tree
    }

    #[inline]
    fn alloc_node(&mut self, value: T, parent: Option<NodeId>) -> NodeId {
        let id = NodeId {
            index: self.values.len(),
            generation: 0,
        };
        self.values.push(value);
        self.parents.push(parent);
        self.children.push(SmallVec::new());
        self.generations.push(0);
        self.alive_count += 1;

        if let Some(p) = parent {
            self.children[p.index].push(id);
        }
        id
    }

    fn check_alive(&self, node: NodeId) -> AlgodsResult<()> {
        if node.index < self.generations.len() && self.generations[node.index] == node.generation {
            Ok(())
        } else {
            Err(AlgodsError::InvalidNodeId)
        }
    }

    fn remove_inner(&mut self, root: NodeId) {
        let mut stack = vec![root];
        while let Some(n) = stack.pop() {
            let idx = n.index;
            for &c in self.children[idx].iter() {
                stack.push(c);
            }
            self.children[idx].clear();
            self.parents[idx] = None;
            self.generations[idx] += 1;
            self.alive_count -= 1;
        }
    }
}

impl<T> Tree for VecTree<T> {
    type Value = T;

    fn root(&self) -> NodeId {
        NodeId {
            index: 0,
            generation: self.generations[0],
        }
    }

    fn contains(&self, node: NodeId) -> bool {
        node.index < self.generations.len() && self.generations[node.index] == node.generation
    }

    fn parent(&self, node: NodeId) -> AlgodsResult<Option<NodeId>> {
        self.check_alive(node)?;
        Ok(self.parents[node.index])
    }

    fn children(&self, node: NodeId) -> AlgodsResult<&[NodeId]> {
        self.check_alive(node)?;
        Ok(&self.children[node.index])
    }

    fn value(&self, node: NodeId) -> AlgodsResult<&Self::Value> {
        self.check_alive(node)?;
        Ok(&self.values[node.index])
    }

    fn value_mut(&mut self, node: NodeId) -> AlgodsResult<&mut Self::Value> {
        self.check_alive(node)?;
        Ok(&mut self.values[node.index])
    }

    fn add_child(&mut self, parent: NodeId, value: T) -> AlgodsResult<NodeId> {
        self.check_alive(parent)?;
        Ok(self.alloc_node(value, Some(parent)))
    }

    fn remove_subtree(&mut self, node: NodeId) -> AlgodsResult<()> {
        self.check_alive(node)?;
        if node.index == 0 {
            return Err(AlgodsError::CannotRemoveRoot);
        }

        let parent = self.parents[node.index].unwrap();
        self.children[parent.index].retain(|&mut c| c != node);

        self.remove_inner(node);
        Ok(())
    }

    fn size(&self) -> usize {
        self.alive_count
    }

    fn parent_unchecked(&self, node: NodeId) -> Option<NodeId> {
        unsafe { *self.parents.get_unchecked(node.index) }
    }

    fn children_unchecked(&self, node: NodeId) -> &[NodeId] {
        unsafe { self.children.get_unchecked(node.index) }
    }
}

#[cfg(test)]
mod tests {
    use crate::hierarchy::hierarchy::Hierarchy;

    use super::*;

    #[test]
    fn create_tree_with_root() {
        let tree = VecTree::with_root(42);
        let root = tree.root();

        assert!(tree.contains(root));
        assert_eq!(*tree.value(root).unwrap(), 42);
        assert!(tree.parent(root).unwrap().is_none());
        assert!(tree.children(root).unwrap().is_empty());
        assert_eq!(tree.size(), 1);
    }

    #[test]
    fn add_children() {
        let mut tree = VecTree::with_root("root");
        let root = tree.root();

        let a = tree.add_child(root, "a").unwrap();
        let b = tree.add_child(root, "b").unwrap();

        assert_eq!(tree.parent(a).unwrap(), Some(root));
        assert_eq!(tree.parent(b).unwrap(), Some(root));

        let children = tree.children(root).unwrap();
        assert_eq!(children.len(), 2);
        assert!(children.contains(&a));
        assert!(children.contains(&b));
        assert_eq!(tree.size(), 3);
    }

    #[test]
    fn remove_subtree_invalidates_nodes() {
        let mut tree = VecTree::with_root(0);
        let root = tree.root();

        let a = tree.add_child(root, 1).unwrap();
        let b = tree.add_child(a, 2).unwrap();
        let c = tree.add_child(b, 3).unwrap();

        tree.remove_subtree(a).unwrap();

        assert!(!tree.contains(a));
        assert!(!tree.contains(b));
        assert!(!tree.contains(c));

        // root 仍然存在
        assert!(tree.contains(root));
        assert!(tree.children(root).unwrap().is_empty());
        assert_eq!(tree.size(), 1);
    }

    #[test]
    fn generation_prevents_stale_node_id() {
        let mut tree = VecTree::with_root(0);
        let root = tree.root();

        let a = tree.add_child(root, 1).unwrap();
        let stale = a;

        tree.remove_subtree(a).unwrap();

        // 原 NodeId 已悬垂
        assert!(!tree.contains(stale));
    }

    #[test]
    fn dfs_traversal_order() {
        let mut tree = VecTree::with_root(0);
        let r = tree.root();
        let a = tree.add_child(r, 1).unwrap();
        let _b = tree.add_child(r, 2).unwrap();
        let _c = tree.add_child(a, 3).unwrap();

        let dfs: Vec<_> = tree
            .dfs_iter(r)
            .unwrap()
            .map(|n| *tree.value(n).unwrap())
            .collect();
        assert_eq!(dfs, vec![0, 1, 3, 2]);
    }

    #[test]
    fn bfs_traversal_order() {
        let mut tree = VecTree::with_root(0);
        let r = tree.root();
        let a = tree.add_child(r, 1).unwrap();
        let _b = tree.add_child(r, 2).unwrap();
        let _c = tree.add_child(a, 3).unwrap();

        let bfs: Vec<_> = tree
            .bfs_iter(r)
            .unwrap()
            .map(|n| *tree.value(n).unwrap())
            .collect();
        assert_eq!(bfs, vec![0, 1, 2, 3]);
    }

    #[test]
    fn removing_nonexistent_node_returns_error() {
        let mut tree = VecTree::with_root(1);
        let fake = NodeId {
            index: 100,
            generation: 0,
        };

        // remove_subtree 返回 InvalidNodeId 错误
        let res = tree.remove_subtree(fake);
        assert!(matches!(res, Err(AlgodsError::InvalidNodeId)));
        assert_eq!(tree.size(), 1);
    }

    #[test]
    fn removing_root_returns_error() {
        let mut tree = VecTree::with_root(1);
        let root = tree.root();
        let res = tree.remove_subtree(root);
        assert!(matches!(res, Err(AlgodsError::CannotRemoveRoot)));
    }
}
