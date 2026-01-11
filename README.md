# **mylib-rs**

My Rust Utilities

## Quick Start

运行测试

```shell
cargo test
```

生成并查看文档

```shell
cargo doc --workspace --open
```

## Plan

* math
  * num 数值库：包含常用数值 trait、分数 Frac、复数 Complex、向量 Vector2/Vector3、矩阵 Matrix
  * bignum 高精度数值库：包含高精度整数 BigInteger、高精度浮点数 BigDecimal
  * expr 表达式库：包含表达式求值 Expr
  * graph 图算法库
* net
  * http 服务器
* data structure
  * 红黑树：RBTreeMap
  * 斐波那契堆：FibonacciHeap，给 graph 用
  * 并查集：UnionFind，给 graph 用
  * 线段树：SegmentTree，数组实现，支持区间求和/最大/最小查询和更新
  * 树状数组：FenwickTree，数组实现，支持区间前缀和/更新，O(log n)
  * trie 树：Trie
  * hierarchy 层次结构库：包含 LCRS 封装层次结构 LCRSTree、Vec 存储层次结构 Tree，提供统一接口
