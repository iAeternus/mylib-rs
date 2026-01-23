# RBTree 模块文档

## 概述

`rbtree` 模块实现了一个基于红黑树算法的键值映射容器 `RBTreeMap`，API 设计仿照了 Rust 标准库的 `std::collections::BTreeMap`。该模块提供了高效的插入、删除、查找和范围查询操作。

## 模块结构

```
algods/src/collections/rbtree/
├── tree.rs      # 红黑树核心算法实现
├── map.rs       # RBTreeMap 公共接口
├── entry.rs     # Entry API 实现
├── iter.rs       # 迭代器实现
└── range.rs      # 范围查询实现
```

## tree.rs - 红黑树核心

### 数据结构

#### `Color` 枚举
```rust
enum Color {
    Red,
    Black,
}
```
表示节点的颜色。

#### `Node<K, V>` 结构
```rust
pub(crate) struct Node<K, V> {
    pub(crate) key: K,
    pub(crate) val: V,
    pub(crate) lch: Link<K, V>,   // 左子节点
    pub(crate) rch: Link<K, V>,   // 右子节点
    pub(crate) parent: Link<K, V>,
    color: Color,
}
```

#### `RBTree<K, V>` 结构
```rust
pub(crate) struct RBTree<K, V> {
    pub(crate) root: Link<K, V>,
    len: usize,
    pub(crate) nil: Link<K, V>,  // 哨兵节点
    _boo: PhantomData<Box<(K, V)>>,
}
```

### 核心方法

#### 创建和清理
- `new(nil_key: K, nil_val: V)` - 创建红黑树，需要传入哨兵键值
- `clear(&mut self)` - 清空整棵树，但保留 nil 哨兵

#### 查询操作
- `search_tree(&self, key: &K) -> Link<K, V>` - 查找指定键的节点
- `min(&self, x: Link<K, V>) -> Link<K, V>` - 查找以 x 为根的子树中最小节点
- `max(&self, x: Link<K, V>) -> Link<K, V>` - 查找以 x 为根的子树中最大节点
- `successor(&self, x: Link<K, V>) -> Link<K, V>` - 查找中序遍历的后继节点
- `predecessor(&self, x: Link<K, V>) -> Link<K, V>` - 查找中序遍历的前驱节点

#### 插入操作
- `insert(&mut self, key: K, val: V) -> Link<K, V>` - 插入新节点并执行红黑树修复

**插入修复算法** (`insert_fixup`)：

1. **Case 1**: 插入节点 z 的叔节点 y 是红色的
   - 将父节点、叔节点染黑
   - 将爷节点染红
   - 当前节点指向爷节点，继续调整

2. **Case 2**: 叔节点 y 是黑色的，且 z 是右孩子
   - 对 z 进行左旋
   - 转换为 Case 3

3. **Case 3**: 叔节点 y 是黑色的，且 z 是左孩子
   - 将父节点染黑
   - 将爷节点染红
   - 对爷节点进行右旋

#### 删除操作
- `remove(&mut self, z: Link<K, V>) -> Link<K, V>` - 删除指定节点并执行红黑树修复

**删除修复算法** (`remove_fixup`)：

1. **Case 1**: 兄弟节点 w 是红色的
   - 将兄节点染黑
   - 将父节点染红
   - 对父节点进行左旋

2. **Case 2**: 兄弟节点 w 是黑色的，且 w 的两个子节点都是黑色的
   - 将兄节点染红
   - 将双黑上移

3. **Case 3**: 兄弟节点 w 是黑色的，w 的左孩子是红色，右孩子是黑色
   - 将兄节点及其左孩子变色
   - 对兄节点进行右旋

4. **Case 4**: 兄弟节点 w 是黑色的，w 的左孩子是黑色，右孩子是红色
   - 将兄节点与其父节点互换颜色
   - 对父节点进行左旋

### 红黑树性质

1. 每个节点要么是红色，要么是黑色
2. 根节点是黑色的
3. 红色节点的两个子节点都是黑色的
4. 从任意节点到其所有后代节点的路径上，黑色节点数量相同

---

## map.rs - 公共接口

### `RBTreeMap<K, V>` 结构
```rust
pub struct RBTreeMap<K, V> {
    pub(crate) tree: RBTree<K, V>,
}
```

### 基础方法

#### 创建
```rust
pub fn new(nil_key: K, nil_val: V) -> Self
```
需要传入哨兵键值（通常作为最小可能的键值）。

#### 容量操作
```rust
pub fn len(&self) -> usize           // 返回键值对数量
pub fn is_empty(&self) -> bool         // 判断是否为空
pub fn clear(&mut self)               // 清空所有元素
```

#### 查询操作
```rust
pub fn get(&self, key: &K) -> Option<&V>
pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
pub fn contains_key(&self, key: &K) -> bool
pub fn first_key_value(&self) -> Option<(&K, &V)>   // 最小键值对
pub fn last_key_value(&self) -> Option<(&K, &V)>    // 最大键值对
```

#### 修改操作
```rust
pub fn insert(&mut self, key: K, val: V) -> Option<V>
```
返回被替换的旧值（如果键已存在），否则返回 `None`。

```rust
pub fn remove(&mut self, key: &K) -> Option<V>
```
返回被删除的值（如果键存在），否则返回 `None`。

#### Entry API
```rust
pub fn entry(&mut self, key: K) -> Entry<'_, K, V>
```
返回对应键的 Entry，用于原地修改或条件插入。

#### 迭代器
```rust
pub fn iter(&self) -> Iter<'_, K, V>
pub fn iter_mut(&mut self) -> IterMut<'_, K, V>
pub fn keys(&self) -> Keys<'_, K, V>
pub fn values(&self) -> Values<'_, K, V>
pub fn values_mut(&mut self) -> ValuesMut<'_, K, V>
```

#### 范围查询
```rust
pub fn range<Q, R>(&self, range: R) -> Range<'_, K, V>
pub fn range_mut<Q, R>(&mut self, range: R) -> RangeMut<'_, K, V>
```
支持 `std::ops::RangeBounds` 的范围操作：
- `a..b` - [a, b)
- `a..=b` - [a, b]
- `a..` - [a, +∞)
- `..b` - (-∞, b]

---

## entry.rs - Entry API

### 数据结构

#### `Entry<'a, K, V>` 枚举
```rust
pub enum Entry<'a, K, V> {
    Vacant(VacantEntry<'a, K, V>),      // 键不存在
    Occupied(OccupiedEntry<'a, K, V>),  // 键已存在
}
```

#### `VacantEntry<'a, K, V>` 结构
```rust
pub struct VacantEntry<'a, K, V> {
    pub(crate) map: &'a mut RBTreeMap<K, V>,
    pub(crate) key: K,
}
```

#### `OccupiedEntry<'a, K, V>` 结构
```rust
pub struct OccupiedEntry<'a, K, V> {
    pub(crate) map: &'a mut RBTreeMap<K, V>,
    pub(crate) node: Link<K, V>,
}
```

### Entry 方法

#### `Entry<'a, K, V>` 方法
```rust
pub fn or_insert(self, default: V) -> &'a mut V
pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V
pub fn and_modify<F: FnOnce(&mut V)>(self, f: F) -> Self
pub fn key(&self) -> &K
```

#### `VacantEntry<'a, K, V>` 方法
```rust
pub fn insert(self, val: V) -> &'a mut V
```

#### `OccupiedEntry<'a, K, V>` 方法
```rust
pub fn get(&self) -> &V
pub fn get_mut(&mut self) -> &mut V
pub fn insert(&mut self, val: V) -> V           // 替换并返回旧值
pub fn into_mut(self) -> &'a mut V
pub fn key(&self) -> &K
pub fn remove(self) -> V                         // 删除并返回值
```

### 使用示例

```rust
let mut map = RBTreeMap::new(0, 0);

// or_insert - 如果键不存在则插入默认值
map.entry(1).or_insert(10);

// or_insert_with - 延迟计算默认值
map.entry(2).or_insert_with(|| expensive_computation());

// and_modify - 如果键存在则修改值
map.entry(3).and_modify(|v| *v += 1);

// 链式操作
map.entry(1).and_modify(|v| *v = 100).or_insert(200);
```

---

## iter.rs - 迭代器

### 数据结构

#### `Iter<'a, K, V>` - 不可变迭代器
```rust
pub struct Iter<'a, K, V> {
    tree: &'a RBTree<K, V>,
    next: Link<K, V>,
}
```

#### `IterMut<'a, K, V>` - 可变迭代器
```rust
pub struct IterMut<'a, K, V> {
    tree: &'a mut RBTree<K, V>,
    next: Link<K, V>,
}
```

#### `Keys<'a, K, V>` - 键迭代器
```rust
pub struct Keys<'a, K, V>(pub Iter<'a, K, V>);
```

#### `Values<'a, K, V>` - 值迭代器
```rust
pub struct Values<'a, K, V>(pub Iter<'a, K, V>);
```

#### `ValuesMut<'a, K, V>` - 可变值迭代器
```rust
pub struct ValuesMut<'a, K, V>(pub IterMut<'a, K, V>);
```

### 使用示例

```rust
let mut map = RBTreeMap::new(0, 0);
map.insert(1, 10);
map.insert(2, 20);
map.insert(3, 30);

// 不可变遍历
for (k, v) in map.iter() {
    println!("{}: {}", k, v);
}

// 可变遍历
for (_, v) in map.iter_mut() {
    *v *= 2;
}

// 遍历键
for k in map.keys() {
    println!("{}", k);
}

// 遍历值
for v in map.values() {
    println!("{}", v);
}
```

---

## range.rs - 范围查询

### 数据结构

#### `Range<'a, K, V>` - 不可变范围
```rust
pub struct Range<'a, K, V> {
    tree: &'a RBTree<K, V>,
    next: Link<K, V>,
    end: Bound<&'a K>,
}
```

#### `RangeMut<'a, K, V>` - 可变范围
```rust
pub struct RangeMut<'a, K, V> {
    tree: &'a mut RBTree<K, V>,
    next: Link<K, V>,
    end: Bound<&'a K>,
}
```

### 辅助方法

#### `RBTree` 扩展
```rust
fn find_ge<Q: ?Sized + Ord>(&self, key: &Q) -> Link<K, V>
```
查找第一个 >= key 的节点（大于等于下界）。

```rust
fn find_gt<Q: ?Sized + Ord>(&self, key: &Q) -> Link<K, V>
```
查找第一个 > key 的节点（大于下界）。

### 使用示例

```rust
let mut map = RBTreeMap::new(0, 0);
for i in 1..=10 {
    map.insert(i, i * 10);
}

// 包含范围 [2, 5)
let range: Vec<_> = map.range(2..=5).collect();

// 排除范围 [2, 5)
let range: Vec<_> = map.range(2..5).collect();

// 全部范围
let all: Vec<_> = map.range(..).collect();
```

---

## 性能特性

| 操作 | 平均时间复杂度 | 最坏情况 |
|------|----------------|----------|
| 插入 | O(log n) | O(log n) |
| 删除 | O(log n) | O(log n) |
| 查找 | O(log n) | O(log n) |
| 最小/最大 | O(log n) | O(log n) |
| 后继/前驱 | O(log n) | O(log n) |
| 范围查询 | O(log n + k) | O(log n + k) |

---

## 注意事项

1. **哨兵键值**: 创建 `RBTreeMap` 时必须传入哨兵键值，这些值应当小于所有可能的实际键值。

2. **内存安全**: 大量使用 `unsafe` 代码来操作原始指针，使用 `NonNull` 确保指针非空。

3. **所有权**: 迭代器提供对底层节点的引用，需要注意生命周期管理。

4. **并发**: 当前实现不是线程安全的。

---

## API 与标准库对比

| 方法 | RBTreeMap | BTreeMap | 说明 |
|------|-----------|----------|------|
| new | 需要哨兵参数 | 不需要 | 设计差异 |
| insert | 相同 | 相同 | 兼容 |
| remove | 相同 | 相同 | 兼容 |
| get | 相同 | 相同 | 兼容 |
| entry | 相同 | 相同 | 兼容 |
| iter | 相同 | 相同 | 兼容 |
| keys | 相同 | 相同 | 兼容 |
| values | 相同 | 相同 | 兼容 |
| range | 相同 | 相同 | 兼容 |
