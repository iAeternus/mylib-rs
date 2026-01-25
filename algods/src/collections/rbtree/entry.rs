use crate::collections::rbtree::{map::RBTreeMap, tree::Link};

/// 对 map 中某个 key 的一次性访问视图
pub enum Entry<'a, K, V> {
    Vacant(VacantEntry<'a, K, V>),
    Occupied(OccupiedEntry<'a, K, V>),
}

pub struct VacantEntry<'a, K, V> {
    pub(crate) map: &'a mut RBTreeMap<K, V>,
    pub(crate) key: K,
}

pub struct OccupiedEntry<'a, K, V> {
    pub(crate) map: &'a mut RBTreeMap<K, V>,
    pub(crate) node: Link<K, V>,
}

impl<'a, K: Ord, V> Entry<'a, K, V> {
    /// 确保值存在，通过插入默认值来处理 Vacant 情况
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    /// 确保值存在，通过闭包计算默认值来处理 Vacant 情况
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }

    /// 如果为 Occupied 则修改值
    pub fn and_modify<F: FnOnce(&mut V)>(self, f: F) -> Self {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }

    /// 获取值的引用
    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(entry) => unsafe { &entry.node.unwrap().as_ref().key },
            Entry::Vacant(entry) => &entry.key,
        }
    }
}

impl<'a, K: Ord, V> VacantEntry<'a, K, V> {
    pub fn insert(self, val: V) -> &'a mut V {
        let link = self.map.tree.insert(self.key, val);
        unsafe { &mut (*link.unwrap().as_ptr()).val }
    }
}

impl<'a, K: Ord, V> OccupiedEntry<'a, K, V> {
    pub fn get(&self) -> &V {
        unsafe { &(*self.node.unwrap().as_ptr()).val }
    }

    pub fn get_mut(&mut self) -> &mut V {
        unsafe { &mut (*self.node.unwrap().as_ptr()).val }
    }

    pub fn insert(&mut self, val: V) -> V {
        unsafe {
            let old = std::mem::replace(&mut (*self.node.unwrap().as_ptr()).val, val);
            old
        }
    }

    pub fn into_mut(self) -> &'a mut V {
        unsafe { &mut (*self.node.unwrap().as_ptr()).val }
    }

    pub fn key(&self) -> &K {
        unsafe { &self.node.unwrap().as_ref().key }
    }

    pub fn remove(self) -> V
    where
        V: Clone,
    {
        unsafe {
            let node = self.node.unwrap().as_ptr();
            let old_val = (*node).val.clone();
            self.map.tree.remove(self.node);
            old_val
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_vacant_or_insert() {
        let mut map = new_i32_i32_map();

        let v = map.entry(10).or_insert(42);
        assert_eq!(*v, 42);

        // 已插入
        assert_eq!(map.get(&10), Some(&42));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn entry_or_insert_with_called_once() {
        let mut map = new_i32_i32_map();
        let mut called = 0;

        let v = map.entry(1).or_insert_with(|| {
            called += 1;
            100
        });

        assert_eq!(*v, 100);
        assert_eq!(called, 1);

        // 再次 entry，不应再调用
        let v2 = map.entry(1).or_insert_with(|| {
            called += 1;
            200
        });

        assert_eq!(*v2, 100);
        assert_eq!(called, 1);
    }

    #[test]
    fn entry_occupied_get_and_mut() {
        let mut map = new_i32_i32_map();
        map.entry(5).or_insert(10);

        match map.entry(5) {
            Entry::Occupied(mut e) => {
                assert_eq!(*e.get(), 10);

                *e.get_mut() = 20;
                assert_eq!(*e.get(), 20);
            }
            _ => panic!("expected occupied"),
        }

        assert_eq!(map.get(&5), Some(&20));
    }

    #[test]
    fn entry_into_mut() {
        let mut map = new_i32_i32_map();
        map.entry(7).or_insert(30);

        let v = match map.entry(7) {
            Entry::Occupied(e) => e.into_mut(),
            _ => panic!("expected occupied"),
        };

        *v = 99;
        assert_eq!(map.get(&7), Some(&99));
    }

    #[test]
    fn entry_and_modify_only_on_occupied() {
        let mut map = new_i32_i32_map();

        map.entry(1).and_modify(|v| *v += 1).or_insert(10);

        assert_eq!(map.get(&1), Some(&10));

        map.entry(1).and_modify(|v| *v += 5).or_insert(0);

        assert_eq!(map.get(&1), Some(&15));
    }

    #[test]
    fn entry_insert_replaces_value() {
        let mut map = new_i32_i32_map();
        map.entry(3).or_insert(10);

        match map.entry(3) {
            Entry::Occupied(mut e) => {
                let old = e.insert(50);
                assert_eq!(old, 10);
            }
            _ => panic!("expected occupied"),
        }

        assert_eq!(map.get(&3), Some(&50));
    }

    #[test]
    fn entry_remove() {
        let mut map = new_i32_i32_map();
        map.entry(8).or_insert(123);

        let removed = match map.entry(8) {
            Entry::Occupied(e) => e.remove(),
            _ => panic!("expected occupied"),
        };

        assert_eq!(removed, 123);
        assert_eq!(map.get(&8), None);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn entry_key() {
        let mut map = new_i32_i32_map();

        let e1 = map.entry(1);
        let k1 = e1.key();
        assert_eq!(*k1, 1);

        map.entry(1).or_insert(10);

        let e2 = map.entry(1);
        let k2 = e2.key();
        assert_eq!(*k2, 1);
    }

    fn new_i32_i32_map() -> RBTreeMap<i32, i32> {
        RBTreeMap::new(0, 0)
    }
}
