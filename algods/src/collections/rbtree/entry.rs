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
