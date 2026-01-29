//! 并查集
//!
//! 用于维护若干不相交集合，支持集合的合并与连通性查询

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    /// 创建一个新的并查集，初始化大小为`n`
    ///
    /// ## Notes
    /// 时间复杂度: O(n)
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect::<Vec<usize>>();
        let rank = vec![0; n];
        let size = vec![1; n];
        Self { parent, rank, size }
    }

    /// 查找元素`x`所在集合的根节点
    ///
    /// ## Notes
    /// - 在查找时应用路径压缩
    /// - 时间复杂度: 均摊 O(α(n))
    pub fn find(&mut self, x: usize) -> usize {
        let mut node = x;
        while self.parent[node] != node {
            self.parent[node] = self.parent[self.parent[node]];
            node = self.parent[node];
        }
        node
    }

    /// 合并两个元素所在的集合
    ///
    /// ## Notes
    /// 时间复杂度: 均摊 O(α(n))
    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        } else if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
            self.size[root_x] += self.size[root_y];
        }
    }

    /// 判断两个元素是否在同一个集合中
    ///
    /// ## Notes
    /// 时间复杂度: 均摊 O(α(n))
    pub fn is_connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// 返回指定元素`x`所在集合的大小
    ///
    /// ## Notes
    /// 时间复杂度: 均摊 O(α(n))
    pub fn size(&mut self, x: usize) -> usize {
        let root_x = self.find(x);
        self.size[root_x]
    }

    /// 返回并查集中总共有多少个集合
    ///
    /// ## Notes
    /// 时间复杂度: O(n α(n))
    pub fn sets_count(&mut self) -> usize {
        let mut roots = HashSet::new();
        for i in 0..self.parent.len() {
            roots.insert(self.find(i));
        }
        roots.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut dsu = DisjointSet::new(10);

        // 初始时，所有元素的父节点是自己
        assert_eq!(dsu.find(0), 0);
        assert_eq!(dsu.find(1), 1);
        assert_eq!(dsu.find(2), 2);
        assert_eq!(dsu.find(3), 3);

        // 1 和 2 合并，2 变成 1 的子集
        dsu.union(1, 2);
        assert_eq!(dsu.find(1), 1);
        assert_eq!(dsu.find(2), 1);

        // 0 和 1 合并，0 成为 1 的子集
        dsu.union(0, 1);
        assert_eq!(dsu.find(0), 1);
        assert_eq!(dsu.find(2), 1);

        // 3 和 4 合并
        dsu.union(3, 4);
        assert_eq!(dsu.find(3), 3);
        assert_eq!(dsu.find(4), 3);

        // 5 和 6 合并
        dsu.union(5, 6);
        assert_eq!(dsu.find(5), 5);
        assert_eq!(dsu.find(6), 5);

        // 检查集合的大小
        assert_eq!(dsu.size(0), 3); // {0, 1, 2}
        assert_eq!(dsu.size(3), 2); // {3, 4}
        assert_eq!(dsu.size(5), 2); // {5, 6}

        // 检查是否在同一集合
        assert!(dsu.is_connected(0, 2));
        assert!(dsu.is_connected(1, 2));
        assert!(!dsu.is_connected(0, 3));
        assert!(!dsu.is_connected(4, 5));

        // 计算并查集的集合数量
        assert_eq!(dsu.sets_count(), 6);

        // 合并一些集合
        dsu.union(2, 3);
        assert!(dsu.is_connected(0, 4));
        assert_eq!(dsu.sets_count(), 5);

        // 合并所有剩余集合
        dsu.union(7, 8);
        dsu.union(8, 9);
        assert!(dsu.is_connected(7, 9));
        assert_eq!(dsu.sets_count(), 3);

        // 合并所有集合
        dsu.union(0, 5);
        dsu.union(4, 9);
        assert!(dsu.is_connected(0, 6));
        assert_eq!(dsu.sets_count(), 1);
    }
}
