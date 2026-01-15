use smallvec::SmallVec;

use crate::hierarchy::tree::{NodeId, Tree};

const INLINE_CHILDREN: usize = 4;

/// 数组实现的有根树，适用于读多写少场景
pub struct VecTree<T> {
    values: Vec<T>,
    parents: Vec<Option<NodeId>>,
    children: Vec<SmallVec<[NodeId; INLINE_CHILDREN]>>,
    generations: Vec<u32>,
    alive_count: usize,
}

impl<T> VecTree<T> {
    /// 创建一棵带根节点的树
    pub fn with_root(value: T) -> Self {
        let mut tree = Self {
            values: Vec::new(),
            parents: Vec::new(),
            children: Vec::new(),
            generations: Vec::new(),
            alive_count: 0,
        };
        tree.make_node(value, None);
        tree
    }

    #[inline]
    fn make_node(&mut self, value: T, parent: Option<NodeId>) -> NodeId {
        let index = self.values.len();
        let generation = 0;
        let id = NodeId { index, generation };

        self.values.push(value);
        self.parents.push(parent);
        self.children.push(SmallVec::new());
        self.generations.push(generation);
        self.alive_count += 1;

        if let Some(p) = parent {
            debug_assert!(self.contains(p));
            self.children[p.index].push(id);
        }

        id
    }

    #[inline]
    fn exists(&self, node: NodeId) -> bool {
        node.index < self.generations.len() && self.generations[node.index] == node.generation
    }

    fn remove_inner(&mut self, node: NodeId) {
        let idx = node.index;
        for &c in self.children[idx].clone().iter() {
            self.remove_inner(c);
        }
        self.children[idx].clear();
        self.parents[idx] = None;
        self.generations[idx] += 1;
        self.alive_count -= 1;
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
        self.exists(node)
    }

    fn parent(&self, node: NodeId) -> Option<NodeId> {
        self.exists(node)
            .then(|| self.parents[node.index])
            .flatten()
    }

    fn children(&self, node: NodeId) -> &[NodeId] {
        assert!(self.exists(node));
        &self.children[node.index]
    }

    fn value(&self, node: NodeId) -> &Self::Value {
        assert!(self.exists(node));
        &self.values[node.index]
    }

    fn value_mut(&mut self, node: NodeId) -> &mut Self::Value {
        assert!(self.exists(node));
        &mut self.values[node.index]
    }

    fn add_child(&mut self, parent: NodeId, value: Self::Value) -> NodeId {
        assert!(self.exists(parent));
        self.make_node(value, Some(parent))
    }

    fn remove_subtree(&mut self, node: NodeId) {
        if !self.exists(node) {
            return; // no-op
        }
        assert!(node.index != 0, "cannot remove root");

        if let Some(p) = self.parents[node.index] {
            self.children[p.index].retain(|&mut c| c != node);
        }

        self.remove_inner(node);
    }

    fn size(&self) -> usize {
        self.alive_count
    }
}

#[cfg(test)]
mod tests {
    use crate::hierarchy::tree::Hierarchy;

    use super::*;

    #[test]
    fn create_tree_with_root() {
        let tree = VecTree::with_root(42);
        let root = tree.root();

        assert!(tree.contains(root));
        assert_eq!(*tree.value(root), 42);
        assert!(tree.parent(root).is_none());
        assert!(tree.children(root).is_empty());
        assert_eq!(tree.size(), 1);
    }

    #[test]
    fn add_children() {
        let mut tree = VecTree::with_root("root");
        let root = tree.root();

        let a = tree.add_child(root, "a");
        let b = tree.add_child(root, "b");

        assert_eq!(tree.parent(a), Some(root));
        assert_eq!(tree.parent(b), Some(root));

        let children = tree.children(root);
        assert_eq!(children.len(), 2);
        assert!(children.contains(&a));
        assert!(children.contains(&b));
        assert_eq!(tree.size(), 3);
    }

    #[test]
    fn remove_subtree_invalidates_nodes() {
        let mut tree = VecTree::with_root(0);
        let root = tree.root();

        let a = tree.add_child(root, 1);
        let b = tree.add_child(a, 2);
        let c = tree.add_child(b, 3);

        tree.remove_subtree(a);

        assert!(!tree.contains(a));
        assert!(!tree.contains(b));
        assert!(!tree.contains(c));

        // root 仍然存在
        assert!(tree.contains(root));
        assert!(tree.children(root).is_empty());
        assert_eq!(tree.size(), 1);
    }

    #[test]
    fn generation_prevents_stale_node_id() {
        let mut tree = VecTree::with_root(0);
        let root = tree.root();

        let a = tree.add_child(root, 1);
        let stale = a;

        tree.remove_subtree(a);

        // 原 NodeId 已悬垂
        assert!(!tree.contains(stale));
    }

    #[test]
    fn dfs_traversal_order() {
        let mut tree = VecTree::with_root(0);
        let r = tree.root();
        let a = tree.add_child(r, 1);
        let _b = tree.add_child(r, 2);
        let _c = tree.add_child(a, 3);

        let dfs: Vec<_> = tree.dfs_iter(r).map(|n| *tree.value(n)).collect();
        assert_eq!(dfs, vec![0, 1, 3, 2]);
    }

    #[test]
    fn bfs_traversal_order() {
        let mut tree = VecTree::with_root(0);
        let r = tree.root();
        let a = tree.add_child(r, 1);
        let _b = tree.add_child(r, 2);
        let _c = tree.add_child(a, 3);

        let bfs: Vec<_> = tree.bfs_iter(r).map(|n| *tree.value(n)).collect();
        assert_eq!(bfs, vec![0, 1, 2, 3]);
    }

    #[test]
    fn removing_nonexistent_node_is_noop() {
        let mut tree = VecTree::with_root(1);
        let fake = NodeId {
            index: 100,
            generation: 0,
        };

        tree.remove_subtree(fake); // should not panic
        assert_eq!(tree.size(), 1);
    }

    #[test]
    #[should_panic(expected = "cannot remove root")]
    fn removing_root_panics() {
        let mut tree = VecTree::with_root(1);
        let root = tree.root();
        tree.remove_subtree(root);
    }
}
