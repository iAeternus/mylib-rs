use std::{
    collections::HashMap,
    hash::{Hasher, RandomState},
    ptr::NonNull,
};

pub struct LruCache<K, V, S = RandomState> {
    map: HashMap<K, NonNull<Node<K, V>>, S>,
    /// 最近使用的
    head: Link<K, V>,
    /// 最久未使用的
    tail: Link<K, V>,
    cap: usize,
}

type Link<K, V> = Option<NonNull<Node<K, V>>>;

struct Node<K, V> {
    key: K,
    val: V,
    prev: Link<K, V>,
    next: Link<K, V>,
}

impl<K, V, S> LruCache<K, V, S> {
    pub fn new(cap: usize) -> Self {
        todo!()
    }

    pub fn with_hasher<H>(cap: usize, hasher: H) -> Self
    where
        H: Hasher,
    {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// 访问并将节点移到头部
    pub fn get(&mut self, key: K) -> Option<&mut V> {
        todo!()
    }

    /// 只读不更新顺序
    pub fn peek(&self, key: K) -> Option<&V> {
        todo!()
    }

    /// 插入，超容量时淘汰尾部并返回旧值
    pub fn put(&mut self, key: K, val: V) -> Option<V> {
        todo!()
    }

    /// 删除
    pub fn remove(&mut self, key: K) -> Option<V> {
        todo!()
    }

    /// 存在性检查
    pub fn contains_key(&self, key: K) -> bool {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }
}
