use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash, RandomState},
    ptr::NonNull,
};

pub struct LruCache<K, V, S = RandomState> {
    map: HashMap<K, NonNull<Node<K, V>>, S>,
    /// 最近使用的
    head: Link<K, V>,
    /// 最久未使用的
    tail: Link<K, V>,
    capacity: usize,
}

type Link<K, V> = Option<NonNull<Node<K, V>>>;

struct Node<K, V> {
    key: K,
    val: V,
    prev: Link<K, V>,
    next: Link<K, V>,
}

impl<K, V> LruCache<K, V, RandomState>
where
    K: Eq + Hash + Clone,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::new(),
            head: None,
            tail: None,
            capacity,
        }
    }
}

impl<K, V, S> LruCache<K, V, S>
where
    K: Eq + Hash + Clone,
    S: BuildHasher,
{
    pub fn with_hasher(capacity: usize, hasher_builder: S) -> Self {
        Self {
            map: HashMap::with_hasher(hasher_builder),
            head: None,
            tail: None,
            capacity,
        }
    }

    /// 访问并将节点移到头部
    pub fn get(&mut self, key: &K) -> Option<&mut V> {
        let ptr = *self.map.get(key)?;

        unsafe {
            self.move_to_front(ptr);

            Some(&mut (*ptr.as_ptr()).val)
        }
    }

    /// 只读不更新顺序
    pub fn peek(&self, key: &K) -> Option<&V> {
        let ptr = *self.map.get(key)?;
        unsafe { Some(&(*ptr.as_ptr()).val) }
    }

    /// 插入键值对
    /// 若 key 已存在，则更新值并移动到头部
    /// 若插入后超过容量，则淘汰 LRU 节点并返回被淘汰的值
    pub fn put(&mut self, key: K, val: V) -> Option<V> {
        if self.capacity == 0 {
            return Some(val);
        }

        // 已存在
        if let Some(&ptr) = self.map.get(&key) {
            unsafe {
                (*ptr.as_ptr()).val = val;

                self.move_to_front(ptr);
            }

            return None;
        }

        // 新节点
        let node = Box::new(Node {
            key: key.clone(),
            val,
            prev: None,
            next: None,
        });

        let ptr = NonNull::new(Box::into_raw(node)).unwrap();

        unsafe {
            self.attach_front(ptr);
        }

        self.map.insert(key, ptr);

        if self.map.len() > self.capacity {
            return self.pop_tail();
        }

        None
    }

    /// 删除
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let ptr = self.map.remove(key)?;

        unsafe {
            self.detach(ptr);

            let boxed = Box::from_raw(ptr.as_ptr());
            let Node { val, .. } = *boxed;
            Some(val)
        }
    }

    /// 存在性检查
    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    /// 淘汰尾节点
    fn pop_tail(&mut self) -> Option<V> {
        let tail = self.tail?;

        unsafe {
            self.detach(tail);

            let boxed = Box::from_raw(tail.as_ptr());
            let Node { key, val, .. } = *boxed;
            self.map.remove(&key);
            Some(val)
        }
    }
}

impl<K, V, S> LruCache<K, V, S> {
    /// 清空
    pub fn clear(&mut self) {
        let mut cur = self.head;

        while let Some(node) = cur {
            unsafe {
                cur = (*node.as_ptr()).next;
                drop(Box::from_raw(node.as_ptr()));
            }
        }

        self.head = None;
        self.tail = None;

        self.map.clear();
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// 摘下链表节点
    unsafe fn detach(&mut self, node: NonNull<Node<K, V>>) {
        let node_ref = unsafe { node.as_ref() };

        match node_ref.prev {
            Some(mut prev) => unsafe {
                prev.as_mut().next = node_ref.next;
            },
            None => self.head = node_ref.next,
        }

        match node_ref.next {
            Some(mut next) => unsafe {
                next.as_mut().prev = node_ref.prev;
            },
            None => self.tail = node_ref.prev,
        }
    }

    /// 头插
    unsafe fn attach_front(&mut self, mut node: NonNull<Node<K, V>>) {
        unsafe {
            let node_ref = node.as_mut();
            node_ref.prev = None;
            node_ref.next = self.head;
        }

        match self.head {
            Some(mut head) => unsafe {
                head.as_mut().prev = Some(node);
            },
            None => {
                self.tail = Some(node);
            }
        }

        self.head = Some(node);
    }

    /// 将结点移动到链表头
    unsafe fn move_to_front(&mut self, node: NonNull<Node<K, V>>) {
        if self.head == Some(node) {
            return;
        }

        unsafe {
            self.detach(node);
            self.attach_front(node);
        }
    }
}

impl<K, V, S> Drop for LruCache<K, V, S> {
    fn drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::RandomState;

    #[test]
    fn test_new_initial_state() {
        let cache: LruCache<i32, i32> = LruCache::new(3);
        assert_eq!(cache.capacity(), 3);
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_with_hasher_initial_state() {
        let cache: LruCache<i32, i32> = LruCache::with_hasher(5, RandomState::new());
        assert_eq!(cache.capacity(), 5);
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_capacity_zero_put_returns_value() {
        let mut cache: LruCache<i32, i32> = LruCache::new(0);
        assert_eq!(cache.put(1, 10), Some(10));
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
        assert_eq!(cache.put(2, 20), Some(20));
    }

    #[test]
    fn test_put_new_key() {
        let mut cache = LruCache::new(3);
        assert_eq!(cache.put(1, 10), None);
        assert_eq!(cache.len(), 1);
        assert!(!cache.is_empty());
    }

    #[test]
    fn test_put_existing_key_updates_value() {
        let mut cache = LruCache::new(3);
        cache.put(1, 10);
        assert_eq!(cache.put(1, 100), None);
        assert_eq!(cache.get(&1), Some(&mut 100));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_get_existing_key() {
        let mut cache = LruCache::new(3);
        cache.put(1, 10);
        assert_eq!(cache.get(&1), Some(&mut 10));
    }

    #[test]
    fn test_get_nonexistent_key() {
        let mut cache: LruCache<i32, i32> = LruCache::new(3);
        assert_eq!(cache.get(&1), None);
    }

    #[test]
    fn test_peek_existing_key() {
        let mut cache = LruCache::new(3);
        cache.put(1, 10);
        assert_eq!(cache.peek(&1), Some(&10));
    }

    #[test]
    fn test_peek_nonexistent_key() {
        let cache: LruCache<i32, i32> = LruCache::new(3);
        assert_eq!(cache.peek(&1), None);
    }

    #[test]
    fn test_peek_does_not_change_order() {
        let mut cache = LruCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.peek(&1);
        // 1 is still LRU (at tail)
        assert_eq!(cache.put(3, 30), Some(10));
        assert_eq!(cache.get(&1), None);
    }

    #[test]
    fn test_eviction_simple() {
        let mut cache = LruCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        assert_eq!(cache.put(3, 30), Some(10));
        assert_eq!(cache.get(&1), None);
    }

    #[test]
    fn test_eviction_order_after_get() {
        let mut cache = LruCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.get(&1);
        assert_eq!(cache.put(3, 30), Some(20));
        assert_eq!(cache.get(&2), None);
    }

    #[test]
    fn test_eviction_after_get_of_non_head() {
        let mut cache = LruCache::new(3);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);
        cache.get(&2);
        assert_eq!(cache.put(4, 40), Some(10));
    }

    #[test]
    fn test_remove_existing_key() {
        let mut cache = LruCache::new(3);
        cache.put(1, 10);
        cache.put(2, 20);
        assert_eq!(cache.remove(&1), Some(10));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_remove_nonexistent_key() {
        let mut cache: LruCache<i32, i32> = LruCache::new(3);
        assert_eq!(cache.remove(&1), None);
    }

    #[test]
    fn test_remove_twice() {
        let mut cache = LruCache::new(3);
        cache.put(1, 10);
        assert_eq!(cache.remove(&1), Some(10));
        assert_eq!(cache.remove(&1), None);
    }

    #[test]
    fn test_remove_head_prev_none() {
        let mut cache = LruCache::new(3);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);
        assert_eq!(cache.remove(&1), Some(10));
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&mut 20));
        assert_eq!(cache.get(&3), Some(&mut 30));
    }

    #[test]
    fn test_remove_tail_next_none() {
        let mut cache = LruCache::new(3);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);
        assert_eq!(cache.remove(&3), Some(30));
        assert_eq!(cache.get(&3), None);
        assert_eq!(cache.get(&2), Some(&mut 20));
    }

    #[test]
    fn test_remove_middle_both_some() {
        let mut cache = LruCache::new(3);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);
        assert_eq!(cache.remove(&2), Some(20));
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&1), Some(&mut 10));
        assert_eq!(cache.get(&3), Some(&mut 30));
    }

    #[test]
    fn test_remove_single_element_both_none() {
        let mut cache = LruCache::new(3);
        cache.put(1, 10);
        assert_eq!(cache.remove(&1), Some(10));
        assert!(cache.is_empty());
        // attach_front on empty list
        cache.put(2, 20);
        assert_eq!(cache.get(&2), Some(&mut 20));
    }

    #[test]
    fn test_contains_key() {
        let mut cache = LruCache::new(3);
        assert!(!cache.contains_key(&1));
        cache.put(1, 10);
        assert!(cache.contains_key(&1));
    }

    #[test]
    fn test_clear_empty() {
        let mut cache: LruCache<i32, i32> = LruCache::new(3);
        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_clear_nonempty() {
        let mut cache = LruCache::new(3);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);
        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
        assert!(!cache.contains_key(&1));
    }

    #[test]
    fn test_reuse_after_clear() {
        let mut cache = LruCache::new(3);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
        cache.put(3, 30);
        cache.put(4, 40);
        cache.put(5, 50);
        assert_eq!(cache.len(), 3);
        let evicted = cache.put(6, 60);
        assert!(evicted.is_some());
        assert_eq!(cache.len(), 3);
        assert!(cache.contains_key(&6));
    }

    #[test]
    fn test_get_head_node_early_return_in_move_to_front() {
        let mut cache = LruCache::new(3);
        cache.put(1, 10);
        assert_eq!(cache.get(&1), Some(&mut 10));
        cache.put(2, 20);
        assert_eq!(cache.get(&1), Some(&mut 10));
    }

    #[test]
    fn test_get_moves_non_head_to_front() {
        let mut cache = LruCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.get(&1);
        assert_eq!(cache.put(3, 30), Some(20));
    }

    #[test]
    fn test_put_existing_moves_to_front() {
        let mut cache = LruCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(1, 100);
        assert_eq!(cache.put(3, 30), Some(20));
        assert_eq!(cache.get(&1), Some(&mut 100));
        assert_eq!(cache.get(&3), Some(&mut 30));
    }

    #[test]
    fn test_drop_nonempty_cache() {
        let mut cache = LruCache::new(3);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);
    }
}
