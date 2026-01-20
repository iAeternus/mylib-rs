use std::ops::Bound;

use crate::{collections::rbtree::tree::RBTree, hierarchy::NodeId};

pub struct Range<'a, K, V> {
    tree: &'a RBTree<K, V>,
    next: Option<NodeId>,
    end: Bound<K>,
}

pub struct RangeMut<'a, K, V> {
    tree: &'a mut RBTree<K, V>,
    next: Option<NodeId>,
    end: Bound<K>,
}
