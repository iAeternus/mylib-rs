use crate::collections::rbtree::tree::{Link, RBTree};

pub struct Iter<'a, K, V> {
    tree: &'a RBTree<K, V>,
    next: Link<K, V>,
}

pub struct IterMut<'a, K, V> {
    tree: &'a mut RBTree<K, V>,
    next: Link<K, V>,
}

pub struct Keys<'a, K, V>(pub Iter<'a, K, V>);
pub struct Values<'a, K, V>(pub Iter<'a, K, V>);
pub struct ValuesMut<'a, K, V>(pub IterMut<'a, K, V>);

impl<'a, K: Ord, V> Iter<'a, K, V> {
    pub fn new(tree: &'a RBTree<K, V>) -> Self {
        let next = tree.min(tree.root);
        Self { tree, next }
    }
}

impl<'a, K: Ord, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.next == self.tree.nil {
                return None;
            }
            let node = self.next.unwrap().as_ptr();
            self.next = self.tree.successor(Some(self.next.unwrap()));
            Some((&(*node).key, &(*node).val))
        }
    }
}

impl<'a, K: Ord, V> IterMut<'a, K, V> {
    pub fn new(tree: &'a mut RBTree<K, V>) -> Self {
        let next = tree.min(tree.root);
        Self { tree, next }
    }
}

impl<'a, K: Ord, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.next == self.tree.nil {
                return None;
            }
            let node = self.next.unwrap().as_ptr();
            self.next = self.tree.successor(Some(self.next.unwrap()));
            Some((&(*node).key, &mut (*node).val))
        }
    }
}

impl<'a, K: Ord, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, _)| k)
    }
}

impl<'a, K: Ord, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_, v)| v)
    }
}

impl<'a, K: Ord, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_, v)| v)
    }
}
