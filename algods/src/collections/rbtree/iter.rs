use std::ops::Bound;

use crate::{collections::rbtree::tree::{Node, RBTree}, hierarchy::NodeId};

pub struct Iter<'a, K, V> {
    tree: &'a RBTree<K, V>,
    next: Option<NodeId>,
}

pub struct IterMut<'a, K, V> {
    tree: &'a mut RBTree<K, V>,
    next: Option<NodeId>,
}

pub struct Keys<'a, K, V>(Iter<'a, K, V>);
pub struct Values<'a, K, V>(Iter<'a, K, V>);

pub struct ValuesMut<'a, K, V>(IterMut<'a, K, V>);

pub struct IntoIter<K, V> {
    nodes: Vec<Node<K, V>>,
    stack: Vec<NodeId>,
}



