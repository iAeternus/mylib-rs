use crate::collections::rbtree::tree::RBTree;

/// 红黑树Map，api仿std::collections::BTreeMap
pub struct RBTreeMap<K, V> {
    tree: RBTree<K, V>,
}
