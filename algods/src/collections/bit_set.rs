#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitSet {
    bits: Vec<u64>,
    size: usize, // universe = {0..size-1}
}

pub struct BitSetIter<'a> {
    bitset: &'a BitSet,
    block: usize,
    mask: u64,
}

impl BitSet {
    /// 创建一个空集合，容量为 size
    pub fn new(size: usize) -> Self {
        let n = (size + 63) / 64; // 需要多少个 u64 才能覆盖 size 个元素
        Self {
            bits: vec![0; n],
            size,
        }
    }

    /// 创建一个包含所有元素 {0..size-1} 的全集
    pub fn full(size: usize) -> Self {
        let mut bs = Self::new(size);
        for i in 0..size {
            bs.insert(i);
        }
        bs
    }

    /// 判断集合是否为空
    pub fn is_empty(&self) -> bool {
        self.bits.iter().all(|&x| x == 0)
    }

    /// 返回集合的容量
    pub fn capacity(&self) -> usize {
        self.size
    }

    /// 返回一个按升序遍历元素的迭代器
    pub fn iter(&self) -> BitSetIter<'_> {
        BitSetIter {
            bitset: self,
            block: 0,
            mask: 0,
        }
    }

    /// 将元素 i 插入集合
    pub fn insert(&mut self, i: usize) {
        assert!(i < self.size);
        let (w, b) = (i / 64, i % 64);
        self.bits[w] |= 1u64 << b;
    }

    /// 从集合中删除元素 i
    pub fn remove(&mut self, i: usize) {
        assert!(i < self.size);
        let (w, b) = (i / 64, i % 64);
        self.bits[w] &= !(1u64 << b);
    }

    /// 判断元素 i 是否在集合中
    pub fn contains(&self, i: usize) -> bool {
        assert!(i < self.size);
        let (w, b) = (i / 64, i % 64);
        (self.bits[w] >> b) & 1 == 1
    }

    /// 翻转元素 i 的存在状态
    pub fn toggle(&mut self, i: usize) {
        assert!(i < self.size);
        let (w, b) = (i / 64, i % 64);
        self.bits[w] ^= 1u64 << b;
    }

    /// 计算并集
    pub fn union(&self, other: &BitSet) -> BitSet {
        assert_eq!(self.size, other.size);
        let bits = self
            .bits
            .iter()
            .zip(&other.bits)
            .map(|(a, b)| a | b)
            .collect();
        Self {
            bits,
            size: self.size,
        }
    }

    /// 计算交集
    pub fn intersection(&self, other: &BitSet) -> BitSet {
        assert_eq!(self.size, other.size);
        let bits = self
            .bits
            .iter()
            .zip(&other.bits)
            .map(|(a, b)| a & b)
            .collect();
        Self {
            bits,
            size: self.size,
        }
    }

    /// 计算差集
    pub fn difference(&self, other: &BitSet) -> BitSet {
        assert_eq!(self.size, other.size);
        let bits = self
            .bits
            .iter()
            .zip(&other.bits)
            .map(|(a, b)| a & !b)
            .collect();
        Self {
            bits,
            size: self.size,
        }
    }

    /// 计算对称差集
    pub fn symmetric_difference(&self, other: &BitSet) -> BitSet {
        assert_eq!(self.size, other.size);
        let bits = self
            .bits
            .iter()
            .zip(&other.bits)
            .map(|(a, b)| a ^ b)
            .collect();
        Self {
            bits,
            size: self.size,
        }
    }

    /// 判断 self 是否为 other 的子集
    pub fn is_subset_of(&self, other: &BitSet) -> bool {
        assert_eq!(self.size, other.size);
        self.bits
            .iter()
            .zip(&other.bits)
            .all(|(a, b)| (a & b) == *a)
    }

    /// 判断两个集合是否有交集
    pub fn intersects(&self, other: &BitSet) -> bool {
        assert_eq!(self.size, other.size);
        self.bits.iter().zip(&other.bits).any(|(a, b)| (a & b) != 0)
    }

    /// 返回集合中元素个数
    pub fn len(&self) -> usize {
        self.bits.iter().map(|x| x.count_ones() as usize).sum()
    }

    /// 返回集合中的最小元素
    pub fn min_element(&self) -> Option<usize> {
        for (i, &w) in self.bits.iter().enumerate() {
            if w != 0 {
                let b = w.trailing_zeros() as usize;
                return Some(i * 64 + b);
            }
        }
        None
    }

    /// 删除集合中的最小元素
    pub fn remove_min(&mut self) {
        for w in &mut self.bits {
            if *w != 0 {
                *w &= *w - 1;
                return;
            }
        }
    }
}

impl<'a> Iterator for BitSetIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.mask != 0 {
                let b = self.mask.trailing_zeros() as usize;
                self.mask &= self.mask - 1;
                return Some((self.block - 1) * 64 + b);
            }

            if self.block >= self.bitset.bits.len() {
                return None;
            }

            self.mask = self.bitset.bits[self.block];
            self.block += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_empty() {
        let bs = BitSet::new(100);
        assert!(bs.is_empty());
        assert_eq!(bs.capacity(), 100);
        assert_eq!(bs.len(), 0);
    }

    #[test]
    fn test_insert_and_contains() {
        let mut bs = BitSet::new(10);
        bs.insert(3);
        bs.insert(5);

        assert!(bs.contains(3));
        assert!(bs.contains(5));
        assert!(!bs.contains(4));
    }

    #[test]
    fn test_remove() {
        let mut bs = BitSet::new(10);
        bs.insert(3);
        bs.remove(3);

        assert!(!bs.contains(3));
        assert!(bs.is_empty());
    }

    #[test]
    fn test_toggle() {
        let mut bs = BitSet::new(10);
        bs.toggle(4);
        assert!(bs.contains(4));
        bs.toggle(4);
        assert!(!bs.contains(4));
    }

    #[test]
    fn test_full() {
        let bs = BitSet::full(5);
        for i in 0..5 {
            assert!(bs.contains(i));
        }
        assert_eq!(bs.len(), 5);
    }

    #[test]
    fn test_union() {
        let mut a = BitSet::new(10);
        let mut b = BitSet::new(10);
        a.insert(1);
        a.insert(3);
        b.insert(3);
        b.insert(4);

        let c = a.union(&b);
        assert!(c.contains(1));
        assert!(c.contains(3));
        assert!(c.contains(4));
        assert_eq!(c.len(), 3);
    }

    #[test]
    fn test_intersection() {
        let mut a = BitSet::new(10);
        let mut b = BitSet::new(10);
        a.insert(1);
        a.insert(3);
        b.insert(3);
        b.insert(4);

        let c = a.intersection(&b);
        assert!(c.contains(3));
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn test_difference() {
        let mut a = BitSet::new(10);
        let mut b = BitSet::new(10);
        a.insert(1);
        a.insert(3);
        a.insert(5);
        b.insert(3);

        let c = a.difference(&b);
        assert!(c.contains(1));
        assert!(c.contains(5));
        assert!(!c.contains(3));
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn test_symmetric_difference() {
        let mut a = BitSet::new(10);
        let mut b = BitSet::new(10);
        a.insert(1);
        a.insert(3);
        b.insert(3);
        b.insert(4);

        let c = a.symmetric_difference(&b);
        assert!(c.contains(1));
        assert!(c.contains(4));
        assert!(!c.contains(3));
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn test_is_subset_of() {
        let mut a = BitSet::new(10);
        let mut b = BitSet::new(10);
        a.insert(1);
        a.insert(3);
        b.insert(1);
        b.insert(3);
        b.insert(5);

        assert!(a.is_subset_of(&b));
        assert!(!b.is_subset_of(&a));
    }

    #[test]
    fn test_intersects() {
        let mut a = BitSet::new(10);
        let mut b = BitSet::new(10);
        a.insert(2);
        b.insert(2);

        assert!(a.intersects(&b));

        b.remove(2);
        b.insert(3);
        assert!(!a.intersects(&b));
    }

    #[test]
    fn test_len() {
        let mut bs = BitSet::new(100);
        for i in [1, 3, 5, 7, 9] {
            bs.insert(i);
        }
        assert_eq!(bs.len(), 5);
    }

    #[test]
    fn test_min_element() {
        let mut bs = BitSet::new(50);
        assert_eq!(bs.min_element(), None);

        bs.insert(20);
        bs.insert(3);
        bs.insert(10);

        assert_eq!(bs.min_element(), Some(3));
    }

    #[test]
    fn test_remove_min() {
        let mut bs = BitSet::new(50);
        bs.insert(20);
        bs.insert(3);
        bs.insert(10);

        bs.remove_min();
        assert!(!bs.contains(3));
        assert_eq!(bs.min_element(), Some(10));

        bs.remove_min();
        assert!(!bs.contains(10));
        assert_eq!(bs.min_element(), Some(20));
    }

    #[test]
    fn test_iterator_order() {
        let mut bs = BitSet::new(20);
        bs.insert(5);
        bs.insert(1);
        bs.insert(10);

        let elems: Vec<_> = bs.iter().collect();
        assert_eq!(elems, vec![1, 5, 10]);
    }

    #[test]
    fn test_iterator_empty() {
        let bs = BitSet::new(10);
        let elems: Vec<_> = bs.iter().collect();
        assert!(elems.is_empty());
    }

    #[test]
    fn test_large_index() {
        let mut bs = BitSet::new(130);
        bs.insert(0);
        bs.insert(64);
        bs.insert(129);

        let elems: Vec<_> = bs.iter().collect();
        assert_eq!(elems, vec![0, 64, 129]);
    }
}
