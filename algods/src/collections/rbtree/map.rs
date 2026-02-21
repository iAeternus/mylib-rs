use crate::collections::rbtree::{
    entry::Entry,
    iter::*,
    range::*,
    tree::{EntrySearch, RBTree},
};
use std::borrow::Borrow;
use std::ops::RangeBounds;

/// 红黑树Map，api仿std::collections::BTreeMap
pub struct RBTreeMap<K, V> {
    pub(crate) tree: RBTree<K, V>,
}

impl<K: Ord, V> RBTreeMap<K, V> {
    pub fn len(&self) -> usize {
        self.tree.len()
    }

    pub fn clear(&mut self) {
        self.tree.clear();
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        unsafe { self.tree.search_tree(key).map(|link| &(*link.as_ptr()).val) }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        unsafe {
            self.tree
                .search_tree(key)
                .map(|link| &mut (*link.as_ptr()).val)
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        if let Some(link) = self.tree.search_tree(&key) {
            unsafe {
                let old = std::mem::replace(&mut (*link.as_ptr()).val, val);
                Some(old)
            }
        } else {
            self.tree.insert(key, val);
            None
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        if let Some(link) = self.tree.search_tree(key) {
            unsafe {
                let old_val = (*link.as_ptr()).val.clone();
                if let Some(removed) = self.tree.remove(Some(link)) {
                    let _ = Box::from_raw(removed.as_ptr());
                }
                Some(old_val)
            }
        } else {
            None
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.tree.search_tree(key).is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first_key_value(&self) -> Option<(&K, &V)> {
        unsafe {
            let min_link = self.tree.min(self.tree.root);
            if min_link != self.tree.nil {
                let node = min_link.unwrap().as_ref();
                Some((&node.key, &node.val))
            } else {
                None
            }
        }
    }

    pub fn last_key_value(&self) -> Option<(&K, &V)> {
        unsafe {
            let max_link = self.tree.max(self.tree.root);
            if max_link != self.tree.nil {
                let node = max_link.unwrap().as_ref();
                Some((&node.key, &node.val))
            } else {
                None
            }
        }
    }

    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        match self.tree.search_entry(&key) {
            EntrySearch::Occupied(link) => {
                Entry::Occupied(crate::collections::rbtree::entry::OccupiedEntry {
                    map: self,
                    node: link,
                })
            }
            EntrySearch::Vacant(pos) => {
                Entry::Vacant(crate::collections::rbtree::entry::VacantEntry {
                    map: self,
                    key,
                    parent: pos.parent,
                    insert_left: pos.insert_left,
                })
            }
        }
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter::new(&self.tree)
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut::new(&mut self.tree)
    }

    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys(Iter::new(&self.tree))
    }

    pub fn values(&self) -> Values<'_, K, V> {
        Values(Iter::new(&self.tree))
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut(IterMut::new(&mut self.tree))
    }

    pub fn range<Q, R>(&self, range: R) -> Range<'_, K, V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
        R: RangeBounds<Q>,
    {
        Range::new(&self.tree, range)
    }

    pub fn range_mut<Q, R>(&mut self, range: R) -> RangeMut<'_, K, V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
        R: RangeBounds<Q>,
    {
        RangeMut::new(&mut self.tree, range)
    }
}

impl<K: Ord + Default, V: Default> RBTreeMap<K, V> {
    pub fn new() -> Self {
        Self {
            tree: RBTree::new(K::default(), V::default()),
        }
    }
}

impl<K: Ord + Default, V: Default> Default for RBTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_len() {
        let mut m = RBTreeMap::new();
        assert_eq!(m.len(), 0);
        assert!(m.is_empty());

        m.insert(1, 10);
        assert_eq!(m.len(), 1);
        assert!(!m.is_empty());
    }

    #[test]
    fn test_insert() {
        let mut m = RBTreeMap::new();

        // Insert new keys
        assert_eq!(m.insert(1, 10), None);
        assert_eq!(m.insert(2, 20), None);
        assert_eq!(m.insert(3, 30), None);

        // Insert existing key
        assert_eq!(m.insert(1, 100), Some(10));
        assert_eq!(m.len(), 3);
    }

    #[test]
    fn test_get() {
        let mut m = RBTreeMap::new();
        m.insert(1, 10);
        m.insert(2, 20);
        m.insert(3, 30);

        assert_eq!(m.get(&1), Some(&10));
        assert_eq!(m.get(&2), Some(&20));
        assert_eq!(m.get(&3), Some(&30));
        assert_eq!(m.get(&4), None);
    }

    #[test]
    fn test_get_mut() {
        let mut m = RBTreeMap::new();
        m.insert(1, 10);

        if let Some(v) = m.get_mut(&1) {
            *v = 100;
        }
        assert_eq!(m.get(&1), Some(&100));
    }

    #[test]
    fn test_remove() {
        let mut m = RBTreeMap::new();
        m.insert(1, 10);
        m.insert(2, 20);
        m.insert(3, 30);

        assert_eq!(m.remove(&2), Some(20));
        assert_eq!(m.len(), 2);
        assert_eq!(m.get(&2), None);
        assert_eq!(m.remove(&2), None);
    }

    #[test]
    fn test_contains_key() {
        let mut m = RBTreeMap::new();
        m.insert(1, 10);
        m.insert(2, 20);

        assert!(m.contains_key(&1));
        assert!(m.contains_key(&2));
        assert!(!m.contains_key(&3));
    }

    #[test]
    fn test_clear() {
        let mut m = RBTreeMap::new();
        m.insert(1, 10);
        m.insert(2, 20);
        m.insert(3, 30);

        m.clear();
        assert_eq!(m.len(), 0);
        assert!(m.is_empty());
        assert_eq!(m.get(&1), None);
    }

    #[test]
    fn test_first_last_key_value() {
        let mut m = RBTreeMap::new();
        m.insert(2, 20);
        m.insert(1, 10);
        m.insert(3, 30);

        assert_eq!(m.first_key_value(), Some((&1, &10)));
        assert_eq!(m.last_key_value(), Some((&3, &30)));

        let empty: RBTreeMap<i32, i32> = RBTreeMap::new();
        assert_eq!(empty.first_key_value(), None);
        assert_eq!(empty.last_key_value(), None);
    }

    #[test]
    fn test_iter() {
        let mut m = RBTreeMap::new();
        m.insert(2, 20);
        m.insert(1, 10);
        m.insert(3, 30);

        let mut iter = m.iter();
        assert_eq!(iter.next(), Some((&1, &10)));
        assert_eq!(iter.next(), Some((&2, &20)));
        assert_eq!(iter.next(), Some((&3, &30)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_keys() {
        let mut m = RBTreeMap::new();
        m.insert(2, 20);
        m.insert(1, 10);
        m.insert(3, 30);

        let keys: Vec<_> = m.keys().collect();
        assert_eq!(keys, vec![&1, &2, &3]);
    }

    #[test]
    fn test_values() {
        let mut m = RBTreeMap::new();
        m.insert(2, 20);
        m.insert(1, 10);
        m.insert(3, 30);

        let values: Vec<_> = m.values().collect();
        assert_eq!(values, vec![&10, &20, &30]);
    }

    #[test]
    fn test_range() {
        let mut m = RBTreeMap::new();
        m.insert(1, 10);
        m.insert(2, 20);
        m.insert(3, 30);
        m.insert(4, 40);
        m.insert(5, 50);

        let range: Vec<_> = m.range(2..=4).collect();
        assert_eq!(range, vec![(&2, &20), (&3, &30), (&4, &40)]);

        let range: Vec<_> = m.range(2..4).collect();
        assert_eq!(range, vec![(&2, &20), (&3, &30)]);
    }

    #[test]
    fn test_range_mut() {
        let mut m = RBTreeMap::new();
        m.insert(1, 10);
        m.insert(2, 20);
        m.insert(3, 30);
        m.insert(4, 40);
        m.insert(5, 50);

        let range: Vec<_> = m.range_mut(2..=4).collect();
        assert_eq!(range, vec![(&2, &mut 20), (&3, &mut 30), (&4, &mut 40)]);

        let range: Vec<_> = m.range_mut(2..4).collect();
        assert_eq!(range, vec![(&2, &mut 20), (&3, &mut 30)]);
    }
}
