use crate::collections::rbtree::tree::{Link, RBTree};
use std::ops::{Bound, RangeBounds};

/// 不可变范围
///
/// ## Notes
/// - Range 使用 key-bound（只读，允许 &K）
pub struct Range<'a, K, V> {
    tree: &'a RBTree<K, V>,
    next: Link<K, V>,
    end: Bound<&'a K>,
}

/// 可变范围
///
/// ## Nodes
/// RangeMut 使用 node-bound（可变，避免 &K）
pub struct RangeMut<'a, K, V> {
    tree: &'a mut RBTree<K, V>,
    next: Link<K, V>,
    end: Link<K, V>,
}

impl<'a, K: Ord, V> Range<'a, K, V> {
    pub fn new<Q, R>(tree: &'a RBTree<K, V>, range: R) -> Self
    where
        K: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
        R: RangeBounds<Q>,
    {
        unsafe {
            let next = match range.start_bound() {
                Bound::Included(k) => tree.find_ge(k),
                Bound::Excluded(k) => tree.find_gt(k),
                Bound::Unbounded => tree.min(tree.root),
            };
            let end = match range.end_bound() {
                Bound::Included(k) => Bound::Included(tree.borrow_key(k)),
                Bound::Excluded(k) => Bound::Excluded(tree.borrow_key(k)),
                Bound::Unbounded => Bound::Unbounded,
            };
            Self { tree, next, end }
        }
    }
}

impl<'a, K: Ord, V> Iterator for Range<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.next == self.tree.nil {
                return None;
            }

            let within_end = match &self.end {
                Bound::Included(end_key) => &(*self.next.unwrap().as_ptr()).key <= *end_key,
                Bound::Excluded(end_key) => &(*self.next.unwrap().as_ptr()).key < *end_key,
                Bound::Unbounded => true,
            };

            if !within_end {
                return None;
            }

            let node = self.next.unwrap().as_ptr();
            self.next = self.tree.successor(self.next);
            Some((&(*node).key, &(*node).val))
        }
    }
}

impl<'a, K: Ord, V> RangeMut<'a, K, V> {
    pub fn new<Q, R>(tree: &'a mut RBTree<K, V>, range: R) -> Self
    where
        K: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
        R: RangeBounds<Q>,
    {
        let next = match range.start_bound() {
            Bound::Included(k) => tree.find_ge(k),
            Bound::Excluded(k) => tree.find_gt(k),
            Bound::Unbounded => tree.min(tree.root),
        };

        let end = match range.end_bound() {
            Bound::Included(k) => tree.find_gt(k),
            Bound::Excluded(k) => tree.find_ge(k),
            Bound::Unbounded => tree.nil,
        };

        Self { tree, next, end }
    }
}

impl<'a, K: Ord, V> Iterator for RangeMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.next == self.end {
                return None;
            }
            if self.next == self.tree.nil {
                return None;
            }

            let node = self.next.unwrap().as_ptr();
            self.next = self.tree.successor(self.next);
            Some((&(*node).key, &mut (*node).val))
        }
    }
}

impl<K: Ord, V> RBTree<K, V> {
    /// 查找第一个 >= key 的节点
    fn find_ge<Q: ?Sized + Ord>(&self, key: &Q) -> Link<K, V>
    where
        K: std::borrow::Borrow<Q>,
    {
        unsafe {
            let mut result = self.nil;
            let mut curr = self.root;

            while curr != self.nil {
                let node = curr.unwrap().as_ref();
                if node.key.borrow() >= key {
                    result = curr;
                    curr = node.lch;
                } else {
                    curr = node.rch;
                }
            }

            result
        }
    }

    /// 查找第一个 > key 的节点
    fn find_gt<Q: ?Sized + Ord>(&self, key: &Q) -> Link<K, V>
    where
        K: std::borrow::Borrow<Q>,
    {
        unsafe {
            let mut result = self.nil;
            let mut curr = self.root;

            while curr != self.nil {
                let node = curr.unwrap().as_ref();
                if node.key.borrow() > key {
                    result = curr;
                    curr = node.lch;
                } else {
                    curr = node.rch;
                }
            }

            result
        }
    }

    /// ## Safety
    /// q must be <= max key in tree
    unsafe fn borrow_key<Q: ?Sized + Ord>(&self, q: &Q) -> &K
    where
        K: std::borrow::Borrow<Q>,
    {
        let link = self.find_ge(q);
        unsafe { &(*link.unwrap().as_ptr()).key }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_ge_basic() {
        let mut t = new_i32_i32_tree();
        insert_seq(&mut t, &[10, 20, 30, 40]);

        unsafe {
            let n = t.find_ge(&20).unwrap().as_ref();
            assert_eq!(n.key, 20);

            let n = t.find_ge(&25).unwrap().as_ref();
            assert_eq!(n.key, 30);

            let n = t.find_ge(&5).unwrap().as_ref();
            assert_eq!(n.key, 10);

            assert_eq!(t.find_ge(&50), t.nil);
        }
    }

    #[test]
    fn find_gt_basic() {
        let mut t = new_i32_i32_tree();
        insert_seq(&mut t, &[10, 20, 30, 40]);

        unsafe {
            let n = t.find_gt(&20).unwrap().as_ref();
            assert_eq!(n.key, 30);

            let n = t.find_gt(&39).unwrap().as_ref();
            assert_eq!(n.key, 40);

            assert_eq!(t.find_gt(&40), t.nil);
        }
    }

    #[test]
    fn find_on_empty_tree() {
        let t = new_i32_i32_tree();
        assert_eq!(t.find_ge(&1), t.nil);
        assert_eq!(t.find_gt(&1), t.nil);
    }

    #[test]
    fn find_on_single_node() {
        let mut t = new_i32_i32_tree();
        t.insert(10, 10);

        unsafe {
            assert_eq!(t.find_ge(&10).unwrap().as_ref().key, 10);
            assert_eq!(t.find_gt(&10), t.nil);
        }
    }

    fn new_i32_i32_tree() -> RBTree<i32, i32> {
        RBTree::new(0, 0)
    }

    fn insert_seq(t: &mut RBTree<i32, i32>, xs: &[i32]) {
        for &x in xs {
            t.insert(x, x);
        }
    }
}
