//! 斐波那契堆，支持 decease-key
//!
//! # Features
//! - 插入 O(1) 摊还
//! - 取最小元素 O(1)
//! - 删除最小元素 O(log n) 摊还
//! - decrease-key O(1) 摊还
//!
//! # Examples
//! ```
//! use algods::collections::fibonacci_heap::FibonacciHeap;
//!
//! let mut heap = FibonacciHeap::new();
//! let h = heap.push(10);
//! heap.decrease_key(h, 5);
//! assert_eq!(heap.peek(), Some(&5));
//! ```
use std::ptr::NonNull;

pub struct FibonacciHeap<T: Ord> {
    min: Link<T>,
    len: usize,
    scratch: Vec<Link<T>>, // 临时数组，用于 consolidate
}

#[derive(Copy, Clone)]
pub struct Handle<T: Ord>(NonNull<Node<T>>);

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T: Ord> {
    elem: T,
    parent: Link<T>,
    child: Link<T>,          // 指向任意一个孩子
    left: NonNull<Node<T>>,  // 同级左兄弟
    right: NonNull<Node<T>>, // 同级右兄弟
    degree: usize,           // 子节点数
    marked: bool,            // 是否丢失过孩子
}

impl<T: Ord> Node<T> {
    fn new(elem: T) -> Box<Self> {
        // 初始化左右指针指向自身，形成循环链表
        let mut node = Box::new(Self {
            elem,
            parent: None,
            child: None,
            left: NonNull::dangling(),
            right: NonNull::dangling(),
            degree: 0,
            marked: false,
        });
        let ptr = unsafe { NonNull::new_unchecked(&mut *node) };
        node.left = ptr;
        node.right = ptr;
        node
    }

    fn reset_as_root(mut node: NonNull<Self>) {
        unsafe {
            node.as_mut().left = node;
            node.as_mut().right = node;
            node.as_mut().parent = None;
            node.as_mut().marked = false;
        }
    }
}

impl<T: Ord> FibonacciHeap<T> {
    pub fn new() -> Self {
        Self {
            min: None,
            len: 0,
            scratch: Vec::new(),
        }
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[must_use]
    pub fn peek(&self) -> Option<&T> {
        unsafe { self.min.map(|p| &p.as_ref().elem) }
    }

    /// 插入新元素，返回 handle 用于 decrease-key
    pub fn push(&mut self, elem: T) -> Handle<T> {
        let node = Node::new(elem);
        let ptr = unsafe { NonNull::new_unchecked(Box::into_raw(node)) };
        self.insert_root(ptr);
        self.len += 1;
        Handle(ptr)
    }

    /// 删除最小元素并返回
    pub fn pop(&mut self) -> Option<T> {
        let min = self.min?;
        unsafe {
            let z = min.as_ptr();

            // 将所有子节点提升到根列表
            if let Some(start) = (*z).child {
                let mut children = Vec::new();
                let mut curr = start;
                loop {
                    children.push(curr);
                    curr = curr.as_ref().right;
                    if curr == start {
                        break;
                    }
                }

                for mut child in children {
                    child.as_mut().parent = None;
                    child.as_mut().left = child;
                    child.as_mut().right = child;
                    self.insert_root(child);
                }

                (*z).child = None;
            }

            self.remove_from_root(z);

            let boxed = Box::from_raw(z);
            let ret = boxed.elem;

            self.len -= 1;
            if self.len == 0 {
                self.min = None;
            } else {
                self.consolidate();
            }
            Some(ret)
        }
    }

    /// 将 handle 指向的节点减小到 new_val
    pub fn decrease_key(&mut self, handle: Handle<T>, new_val: T) {
        unsafe {
            let x = handle.0.as_ptr();
            assert!(new_val <= (*x).elem);
            (*x).elem = new_val;

            if let Some(y) = (*x).parent
                && (*x).elem < (*y.as_ptr()).elem
            {
                self.cut(x, y);
                self.cascading_cut(y);
            }

            if let Some(min) = self.min
                && (*x).elem < (*min.as_ptr()).elem
            {
                self.min = Some(handle.0);
            }
        }
    }

    /// 将节点插入根列表
    fn insert_root(&mut self, mut node: NonNull<Node<T>>) {
        unsafe {
            Node::reset_as_root(node);

            match self.min {
                Some(mut min) => {
                    // 插入 min 节点的左边，保持循环链
                    let mut left = min.as_ref().left;
                    node.as_mut().right = min;
                    node.as_mut().left = left;
                    left.as_mut().right = node;
                    min.as_mut().left = node;

                    if node.as_ref().elem < min.as_ref().elem {
                        self.min = Some(node);
                    }
                }
                None => {
                    node.as_mut().left = node;
                    node.as_mut().right = node;
                    self.min = Some(node);
                }
            }
        }
    }

    /// 从根列表删除节点
    unsafe fn remove_from_root(&mut self, node: *mut Node<T>) {
        unsafe {
            let mut left = (*node).left;
            let mut right = (*node).right;
            left.as_mut().right = right;
            right.as_mut().left = left;

            if self.min == Some(NonNull::new_unchecked(node)) {
                self.min = if right.as_ptr() == node {
                    None
                } else {
                    Some(right)
                };
            }
        }
    }

    /// 合并同度根节点，维护斐波那契堆性质
    fn consolidate(&mut self) {
        let max = (usize::BITS - self.len.leading_zeros()) as usize + 1; // logn + 1
        self.scratch.resize(max, None);

        // 收集所有根节点
        let mut roots = vec![];
        unsafe {
            if let Some(start) = self.min {
                let mut curr = start;
                loop {
                    roots.push(curr);
                    curr = curr.as_ref().right;
                    if curr == start {
                        break;
                    }
                }
            }
        }

        // 按度数合并树
        for w in roots {
            let mut x = w;
            let mut d = unsafe { x.as_ref().degree };
            loop {
                if self.scratch[d].is_none() {
                    self.scratch[d] = Some(x);
                    break;
                }
                let mut y = self.scratch[d].take().unwrap();
                unsafe {
                    if x.as_ref().elem > y.as_ref().elem {
                        std::mem::swap(&mut x, &mut y);
                    }
                    self.link(y, x);
                }
                d += 1;
            }
        }

        // 重新插入根列表
        self.min = None;
        let slots: Vec<_> = self.scratch.iter_mut().filter_map(Option::take).collect();
        for slot in slots {
            Node::reset_as_root(slot);
            self.insert_root(slot);
        }
    }

    /// 将 y 链接到 x 成为 x 的子节点
    unsafe fn link(&mut self, mut y: NonNull<Node<T>>, mut x: NonNull<Node<T>>) {
        unsafe {
            self.remove_from_root(y.as_ptr());
            y.as_mut().parent = Some(x);
            y.as_mut().marked = false;

            if let Some(mut child) = x.as_ref().child {
                let mut right = child.as_ref().right;
                y.as_mut().left = child;
                y.as_mut().right = right;
                child.as_mut().right = y;
                right.as_mut().left = y;
            } else {
                x.as_mut().child = Some(y);
                y.as_mut().left = y;
                y.as_mut().right = y;
            }
            x.as_mut().degree += 1;
        }
    }

    /// 从父节点切出节点到根列表
    unsafe fn cut(&mut self, x: *mut Node<T>, mut y: NonNull<Node<T>>) {
        unsafe {
            let mut left = (*x).left;
            let mut right = (*x).right;
            left.as_mut().right = right;
            right.as_mut().left = left;

            if y.as_ref().child == Some(NonNull::new_unchecked(x)) {
                y.as_mut().child = if right.as_ptr() == x {
                    None
                } else {
                    Some(right)
                };
            }

            y.as_mut().degree -= 1;

            Node::reset_as_root(NonNull::new_unchecked(x));
            self.insert_root(NonNull::new_unchecked(x));
        }
    }

    /// 级联切割父节点
    unsafe fn cascading_cut(&mut self, mut y: NonNull<Node<T>>) {
        unsafe {
            if let Some(z) = y.as_ref().parent {
                if !y.as_ref().marked {
                    y.as_mut().marked = true;
                } else {
                    self.cut(y.as_ptr(), z);
                    self.cascading_cut(z);
                }
            }
        }
    }
}

impl<T: Ord> Drop for FibonacciHeap<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T: Ord> Default for FibonacciHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::FibonacciHeap;

    #[test]
    fn push_pop_peek() {
        let mut heap = FibonacciHeap::new();
        assert!(heap.is_empty());

        heap.push(3);
        heap.push(1);
        heap.push(5);
        assert_eq!(heap.len(), 3);
        assert_eq!(heap.peek(), Some(&1));

        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn decrease_key() {
        let mut heap = FibonacciHeap::new();
        let _h1 = heap.push(5);
        let h2 = heap.push(10);
        let h3 = heap.push(7);

        heap.decrease_key(h2, 3); // 10 -> 3
        heap.decrease_key(h3, 2); // 7 -> 2

        assert_eq!(heap.peek(), Some(&2));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(5));
        assert!(heap.is_empty());
    }

    #[test]
    fn multiple_decrease_key() {
        let mut heap = FibonacciHeap::new();
        let h1 = heap.push(20);
        let _h2 = heap.push(15);
        let h3 = heap.push(30);
        let h4 = heap.push(25);

        heap.decrease_key(h3, 10);
        heap.decrease_key(h1, 5);
        heap.decrease_key(h4, 2);

        assert_eq!(heap.peek(), Some(&2));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), Some(15));
        assert!(heap.is_empty());
    }

    #[test]
    fn pop_empty() {
        let mut heap = FibonacciHeap::<i32>::new();
        assert_eq!(heap.pop(), None);
    }
}
