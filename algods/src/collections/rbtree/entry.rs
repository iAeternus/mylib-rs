use crate::{collections::rbtree::map::RBTreeMap, hierarchy::NodeId};

/// 对 map 中某个 key 的一次性访问视图
pub enum Entry<'a, K, V> {
    Vacant(VacantEntry<'a, K, V>),
    Occupied(OccupiedEntry<'a, K, V>),
}

pub struct VacantEntry<'a, K, V> {
    map: &'a mut RBTreeMap<K, V>,
    key: K,
}

pub struct OccupiedEntry<'a, K, V> {
    map: &'a mut RBTreeMap<K, V>,
    node: NodeId,
}
