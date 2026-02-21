use std::{marker::PhantomData, ptr::NonNull};

/// 红黑树
pub struct RBTree<K, V> {
    pub(crate) root: Link<K, V>,
    len: usize,
    pub(crate) nil: Link<K, V>,
    _boo: PhantomData<Box<(K, V)>>,
}

pub type Link<K, V> = Option<RawLink<K, V>>;
type RawLink<K, V> = NonNull<Node<K, V>>;

pub(crate) enum EntrySearch<K, V> {
    Occupied(Link<K, V>),
    Vacant(VacantPos<K, V>),
}

pub(crate) struct VacantPos<K, V> {
    pub(crate) parent: Link<K, V>,
    pub(crate) insert_left: bool,
}

trait LinkExt<K, V> {
    fn ptr(self) -> *mut Node<K, V>;
}

impl<K, V> LinkExt<K, V> for Link<K, V> {
    #[inline(always)]
    fn ptr(self) -> *mut Node<K, V> {
        self.expect("link should never be None here").as_ptr()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Black,
}

#[derive(Debug)]
pub struct Node<K, V> {
    pub(crate) key: K,
    pub(crate) val: V,
    pub(crate) lch: Link<K, V>,
    pub(crate) rch: Link<K, V>,
    pub(crate) parent: Link<K, V>,
    color: Color,
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V, color: Color, nil: Link<K, V>) -> Self {
        Self {
            key,
            val,
            lch: nil,
            rch: nil,
            parent: nil,
            color,
        }
    }

    fn is_red(&self) -> bool {
        self.color == Color::Red
    }

    fn is_black(&self) -> bool {
        self.color == Color::Black
    }
}

impl<K, V> RBTree<K, V> {
    /// 创建红黑树，哨兵键值需要传入
    pub fn new(nil_key: K, nil_val: V) -> Self {
        unsafe {
            let nil = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                key: nil_key,
                val: nil_val,
                lch: None,
                rch: None,
                parent: None,
                color: Color::Black,
            })));
            Self {
                root: Some(nil),
                len: 0,
                nil: Some(nil),
                _boo: PhantomData,
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 清空整棵树，但保留 nil 哨兵
    pub fn clear(&mut self) {
        unsafe {
            Self::free_all(self, self.root);
            self.root = self.nil;
            self.len = 0;
        }
    }

    unsafe fn free_all(tree: &mut RBTree<K, V>, node: Link<K, V>) {
        if node == tree.nil {
            return;
        }
        let n = node.ptr();
        unsafe {
            Self::free_all(tree, (*n).lch);
            Self::free_all(tree, (*n).rch);
            let _ = Box::from_raw(n); // 释放节点
        }
    }
}

impl<K: Ord, V> RBTree<K, V> {
    pub(crate) fn search_entry(&self, key: &K) -> EntrySearch<K, V> {
        unsafe {
            let mut curr = self.root;
            let mut parent = self.nil;
            let mut insert_left = false;

            while curr != self.nil {
                parent = curr;
                let node = curr.unwrap().as_ref();

                if *key < node.key {
                    curr = node.lch;
                    insert_left = true;
                } else if *key > node.key {
                    curr = node.rch;
                    insert_left = false;
                } else {
                    return EntrySearch::Occupied(curr);
                }
            }

            EntrySearch::Vacant(VacantPos {
                parent,
                insert_left,
            })
        }
    }

    pub(crate) fn insert_vacant(
        &mut self,
        key: K,
        val: V,
        parent: Link<K, V>,
        insert_left: bool,
    ) -> Link<K, V> {
        unsafe {
            let mut z = Node::new(key, val, Color::Red, self.nil);
            z.parent = parent;
            let z_link = NonNull::new(Box::into_raw(Box::new(z)));

            if parent == self.nil {
                self.root = z_link;
            } else if insert_left {
                (*parent.ptr()).lch = z_link;
            } else {
                (*parent.ptr()).rch = z_link;
            }

            self.insert_fixup(z_link);
            self.len += 1;
            z_link
        }
    }

    /// 查找节点
    pub fn search_tree(&self, key: &K) -> Link<K, V> {
        match self.search_entry(key) {
            EntrySearch::Occupied(link) => link,
            EntrySearch::Vacant(_) => None,
        }
    }

    /// 最小节点
    pub fn min(&self, mut x: Link<K, V>) -> Link<K, V> {
        unsafe {
            while x != self.nil {
                let node = x.unwrap().as_ref();
                if node.lch == self.nil {
                    break;
                }
                x = node.lch;
            }
            x
        }
    }

    /// 最大节点
    pub fn max(&self, mut x: Link<K, V>) -> Link<K, V> {
        unsafe {
            while x != self.nil {
                let node = x.unwrap().as_ref();
                if node.rch == self.nil {
                    break;
                }
                x = node.rch;
            }
            x
        }
    }

    /// 后继节点
    pub fn successor(&self, mut x: Link<K, V>) -> Link<K, V> {
        unsafe {
            if x == self.nil {
                return self.nil;
            }

            let node = x.unwrap().as_ref();
            if node.rch != self.nil {
                return self.min(node.rch);
            }

            let mut parent = node.parent;
            while parent != self.nil && Some(x.unwrap()) == parent.unwrap().as_ref().rch {
                x = parent;
                parent = parent.unwrap().as_ref().parent;
            }
            parent
        }
    }

    /// 前驱节点
    pub fn predecessor(&self, mut x: Link<K, V>) -> Link<K, V> {
        unsafe {
            if x == self.nil {
                return self.nil;
            }

            let node = x.unwrap().as_ref();
            if node.lch != self.nil {
                return self.max(node.lch);
            }

            let mut parent = node.parent;
            while parent != self.nil && Some(x.unwrap()) == parent.unwrap().as_ref().lch {
                x = parent;
                parent = parent.unwrap().as_ref().parent;
            }
            parent
        }
    }

    /// 插入 TODO: 这里的insert需要配合entry吗
    pub fn insert(&mut self, key: K, val: V) -> Link<K, V> {
        unsafe {
            let mut z = Node::new(key, val, Color::Red, self.nil);
            let mut y = self.nil;
            let mut x = self.root;

            while x != self.nil {
                y = x;
                if z.key < (*x.ptr()).key {
                    x = (*x.ptr()).lch;
                } else {
                    x = (*x.ptr()).rch;
                }
            }

            z.parent = y;
            let z_link = NonNull::new(Box::into_raw(Box::new(z)));
            if y == self.nil {
                self.root = z_link;
            } else if (*z_link.ptr()).key < (*y.ptr()).key {
                (*y.ptr()).lch = z_link;
            } else {
                (*y.ptr()).rch = z_link;
            }

            self.insert_fixup(z_link);
            self.len += 1;
            z_link
        }
    }

    /// 插入节点后调整
    ///
    /// ## Notes
    /// Case 1. z的叔节点y是红色的：将父/叔/爷节点变色，当前节点指向爷节点，继续调整
    /// Case 2. z的叔节点y是黑色的且z是一个右孩子：左旋，转换为 case 3
    /// Case 3. z的叔节点y是黑色的且z是一个左孩子：将父/爷节点变色，对爷节点右旋
    fn insert_fixup(&mut self, z: Link<K, V>) {
        unsafe {
            let mut z = z;
            while (*(*z.ptr()).parent.ptr()).is_red() {
                if (*z.ptr()).parent == (*(*(*z.ptr()).parent.ptr()).parent.unwrap().as_ptr()).lch {
                    let y = (*(*(*z.ptr()).parent.ptr()).parent.unwrap().as_ptr()).rch; // 叔结点
                    if (*y.ptr()).is_red() {
                        // Case 1: 叔节点为红色
                        (*(*z.ptr()).parent.ptr()).color = Color::Black;
                        (*y.ptr()).color = Color::Black;
                        (*(*(*z.ptr()).parent.ptr()).parent.unwrap().as_ptr()).color = Color::Red;
                        z = (*(*z.ptr()).parent.ptr()).parent;
                    } else {
                        // Case 2/3: 叔节点为黑色
                        if z == (*(*z.ptr()).parent.ptr()).rch {
                            // Case 2: z是右孩子
                            z = (*z.ptr()).parent;
                            self.left_rotate(z);
                        }
                        // Case 3: z是左孩子
                        (*(*z.ptr()).parent.ptr()).color = Color::Black;
                        (*(*(*z.ptr()).parent.ptr()).parent.unwrap().as_ptr()).color = Color::Red;
                        self.right_rotate((*(*z.ptr()).parent.ptr()).parent);
                    }
                } else {
                    // 对称情况
                    let y = (*(*(*z.ptr()).parent.ptr()).parent.unwrap().as_ptr()).lch;
                    if (*y.ptr()).is_red() {
                        (*(*z.ptr()).parent.ptr()).color = Color::Black;
                        (*y.ptr()).color = Color::Black;
                        (*(*(*z.ptr()).parent.ptr()).parent.unwrap().as_ptr()).color = Color::Red;
                        z = (*(*z.ptr()).parent.ptr()).parent;
                    } else {
                        if z == (*(*z.ptr()).parent.ptr()).lch {
                            z = (*z.ptr()).parent;
                            self.right_rotate(z);
                        }
                        (*(*z.ptr()).parent.ptr()).color = Color::Black;
                        (*(*(*z.ptr()).parent.ptr()).parent.unwrap().as_ptr()).color = Color::Red;
                        self.left_rotate((*(*z.ptr()).parent.ptr()).parent);
                    }
                }
            }
            (*self.root.ptr()).color = Color::Black; // 确保root为黑
        }
    }

    /// 左旋
    ///
    /// ## 示例
    /// ```text
    ///      |              |
    ///      y    l_rot     x
    ///     / \   <====    / \
    ///    x   c          a   y
    ///   / \     ====>      / \
    ///  a   b    r_rot     b   c
    /// ```
    /// ## Return
    /// 若 x 为 None 或 nil，则返回 None
    /// 若 x.rch 为 nil，则返回 None
    unsafe fn left_rotate(&mut self, x: Link<K, V>) -> Option<()> {
        if x == self.nil {
            return None;
        }
        unsafe {
            if let Some(x_p) = x {
                if (*x_p.as_ptr()).rch == self.nil {
                    return None;
                }
                // set y
                let y = (*x_p.as_ptr()).rch;
                // 将y的左子树变为x的右子树
                (*x_p.as_ptr()).rch = (*y.ptr()).lch;
                if (*y.ptr()).lch != self.nil {
                    (*(*y.ptr()).lch.ptr()).parent = x;
                }
                // 连接父级
                (*y.ptr()).parent = (*x_p.as_ptr()).parent;
                if (*x_p.as_ptr()).parent == self.nil {
                    self.root = y;
                } else if x == (*(*x_p.as_ptr()).parent.ptr()).lch {
                    (*(*x_p.as_ptr()).parent.ptr()).lch = y;
                } else {
                    (*(*x_p.as_ptr()).parent.ptr()).rch = y;
                }
                // 将x放在y的左边
                (*y.ptr()).lch = x;
                (*x_p.as_ptr()).parent = y;
                return Some(());
            }
        }
        None // x为空
    }

    /// 右旋
    ///
    /// ## 示例
    /// ```text
    ///      |              |
    ///      y    l_rot     x
    ///     / \   <====    / \
    ///    x   c          a   y
    ///   / \     ====>      / \
    ///  a   b    r_rot     b   c
    /// ```
    /// ## Return
    /// 若 y 为 None 或 nil，则返回 None
    /// 若 y.lch 为 nil，则返回 None
    unsafe fn right_rotate(&mut self, y: Link<K, V>) -> Option<()> {
        if y == self.nil {
            return None;
        }
        unsafe {
            if let Some(y_p) = y {
                if (*y_p.as_ptr()).lch == self.nil {
                    return None;
                }

                // set x
                let x = (*y_p.as_ptr()).lch;
                // 将x的右子树变为y的左子树
                (*y_p.as_ptr()).lch = (*x.ptr()).rch;
                if (*x.ptr()).rch != self.nil {
                    (*(*x.ptr()).rch.ptr()).parent = y;
                }
                // 连接父级
                (*x.ptr()).parent = (*y_p.as_ptr()).parent;
                if (*y_p.as_ptr()).parent == self.nil {
                    self.root = x;
                } else if y == (*(*y_p.as_ptr()).parent.ptr()).lch {
                    (*(*y_p.as_ptr()).parent.ptr()).lch = x;
                } else {
                    (*(*y_p.as_ptr()).parent.ptr()).rch = x;
                }
                // 将y放在x的右边
                (*x.ptr()).rch = y;
                (*y_p.as_ptr()).parent = x;
                return Some(());
            }
        }
        None // y为空
    }

    /// 删除节点
    pub fn remove(&mut self, z: Link<K, V>) -> Option<(K, V)> {
        if self.len == 0 || z == self.nil {
            return None;
        }

        unsafe {
            let mut y = z;
            let mut y_original_color = (*y.ptr()).color;

            let x; // y的原始位置
            if (*z.ptr()).lch == self.nil {
                x = (*z.ptr()).rch;
                self.transplant(z, (*z.ptr()).rch);
            } else if (*z.ptr()).rch == self.nil {
                x = (*z.ptr()).lch;
                self.transplant(z, (*z.ptr()).lch);
            } else {
                y = self.min((*z.ptr()).rch);
                y_original_color = (*y.ptr()).color;

                x = (*y.ptr()).rch;
                if (*y.ptr()).parent == z {
                    (*x.ptr()).parent = y;
                } else {
                    self.transplant(y, (*y.ptr()).rch);
                    (*y.ptr()).rch = (*z.ptr()).rch;
                    (*(*y.ptr()).rch.ptr()).parent = y;
                }

                self.transplant(z, y);
                (*y.ptr()).lch = (*z.ptr()).lch;
                (*(*y.ptr()).lch.ptr()).parent = y;
                (*y.ptr()).color = (*z.ptr()).color;
            }

            if y_original_color == Color::Black {
                self.remove_fixup(x);
            }

            self.len -= 1;
            let node = Box::from_raw(z.ptr());
            let Node { key, val, .. } = *node;
            Some((key, val))
        }
    }

    unsafe fn transplant(&mut self, u: Link<K, V>, v: Link<K, V>) {
        unsafe {
            if (*u.ptr()).parent == self.nil {
                self.root = v;
            } else if u == (*(*u.ptr()).parent.ptr()).lch {
                (*(*u.ptr()).parent.ptr()).lch = v;
            } else {
                (*(*u.ptr()).parent.ptr()).rch = v;
            }
            (*v.ptr()).parent = (*u.ptr()).parent;
        }
    }

    /// 删除节点后调整
    ///
    /// ## Notes
    /// Case 1. 兄弟节点w是红色的：兄父变色，对父节点左旋，转换为其他情况
    /// Case 2. 兄弟节点w是黑色，且w的两个子节点都是黑色的：兄弟变红，双黑上移
    /// Case 3. 兄弟节点w是黑色，w的左孩子是红色的，右孩子是黑色的：交换兄弟与其左孩子的颜色，对兄弟右旋，转换为 Case 4
    /// Case 4. 兄弟节点w是黑色，w的右孩子是红色的：变色，对父节点左旋
    fn remove_fixup(&mut self, x: Link<K, V>) {
        unsafe {
            let mut x = x;
            while x != self.root && (*x.ptr()).is_black() {
                if x == (*(*x.ptr()).parent.ptr()).lch {
                    let mut w = (*(*x.ptr()).parent.ptr()).rch; // 兄弟节点

                    if (*w.ptr()).is_red() {
                        // Case 1: 兄弟节点w是红色的
                        (*w.ptr()).color = Color::Black;
                        (*(*x.ptr()).parent.ptr()).color = Color::Red;
                        self.left_rotate((*x.ptr()).parent);
                        w = (*(*x.ptr()).parent.ptr()).rch;
                    }

                    if (*(*w.ptr()).lch.ptr()).is_black() && (*(*w.ptr()).rch.ptr()).is_black() {
                        // Case 2: 兄弟节点w是黑色，且w的两个子节点都是黑色的
                        (*w.ptr()).color = Color::Red;
                        x = (*x.ptr()).parent;
                        continue;
                    }

                    if (*(*w.ptr()).rch.ptr()).is_black() {
                        // Case 3: 兄弟节点w是黑色，w的左孩子是红色，右孩子是黑色
                        (*(*w.ptr()).lch.ptr()).color = Color::Black;
                        (*w.ptr()).color = Color::Red;
                        self.right_rotate(w);
                        w = (*(*x.ptr()).parent.ptr()).rch;
                    }
                    // Case 4: 兄弟节点w是黑色，w的右孩子是红色
                    (*w.ptr()).color = (*(*x.ptr()).parent.ptr()).color;
                    (*(*x.ptr()).parent.ptr()).color = Color::Black;
                    (*(*w.ptr()).rch.ptr()).color = Color::Black;
                    self.left_rotate((*x.ptr()).parent);
                    x = self.root; // 终止循环
                } else {
                    // 对称情况
                    let mut w = (*(*x.ptr()).parent.ptr()).lch;

                    if (*w.ptr()).is_red() {
                        (*w.ptr()).color = Color::Black;
                        (*(*x.ptr()).parent.ptr()).color = Color::Red;
                        self.right_rotate((*x.ptr()).parent);
                        w = (*(*x.ptr()).parent.ptr()).lch;
                    }

                    if (*(*w.ptr()).lch.ptr()).is_black() && (*(*w.ptr()).rch.ptr()).is_black() {
                        (*w.ptr()).color = Color::Red;
                        x = (*x.ptr()).parent;
                        continue;
                    }

                    if (*(*w.ptr()).lch.ptr()).is_black() {
                        (*(*w.ptr()).rch.ptr()).color = Color::Black;
                        (*w.ptr()).color = Color::Red;
                        self.left_rotate(w);
                        w = (*(*x.ptr()).parent.ptr()).lch;
                    }
                    (*w.ptr()).color = (*(*x.ptr()).parent.ptr()).color;
                    (*(*x.ptr()).parent.ptr()).color = Color::Black;
                    (*(*w.ptr()).lch.ptr()).color = Color::Black;
                    self.right_rotate((*x.ptr()).parent);
                    x = self.root;
                }
            }
            (*x.ptr()).color = Color::Black;
        }
    }
}

impl<K, V> Drop for RBTree<K, V> {
    fn drop(&mut self) {
        unsafe {
            Self::free_all(self, self.root);
            let _ = Box::from_raw(self.nil.ptr()); // 释放 nil
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Display;

    #[test]
    fn test_find() {
        let tree = build_test_tree();

        unsafe {
            for &k in &[5, 10, 15, 20, 25, 30, 35] {
                let node = tree.search_tree(&k);
                assert!(node.is_some(), "Key {} should exist", k);
                assert_eq!((*node.ptr()).key, k);
                assert_eq!((*node.ptr()).val, k + 100);
            }

            assert!(tree.search_tree(&0).is_none(), "Key 0 should not exist");
            assert!(tree.search_tree(&40).is_none(), "Key 40 should not exist");
        }
    }

    #[test]
    fn test_min() {
        let tree = build_test_tree();
        unsafe {
            let min_node = tree.min(tree.root);
            assert_eq!((*min_node.ptr()).key, 5);
        }
    }

    #[test]
    fn test_max() {
        let tree = build_test_tree();
        unsafe {
            let max_node = tree.max(tree.root);
            assert_eq!((*max_node.ptr()).key, 35);
        }
    }

    #[test]
    fn test_successor() {
        let tree = build_test_tree();
        unsafe {
            let link_10 = tree.search_tree(&10);
            let succ_10 = tree.successor(link_10);
            assert_eq!((*succ_10.ptr()).key, 15);

            let link_35 = tree.search_tree(&35);
            let succ_35 = tree.successor(link_35);
            assert_eq!(succ_35, tree.nil, "Successor of max should be nil");
        }
    }

    #[test]
    fn test_predecessor() {
        let tree = build_test_tree();
        unsafe {
            let link_25 = tree.search_tree(&25);
            let pred_25 = tree.predecessor(link_25);
            assert_eq!((*pred_25.ptr()).key, 20);

            let link_5 = tree.search_tree(&5);
            let pred_5 = tree.predecessor(link_5);
            assert_eq!(pred_5, tree.nil, "Predecessor of min should be nil");
        }
    }

    #[test]
    fn test_insert() {
        let mut tree = RBTree::<i32, i32>::new(0, 0);
        let keys = vec![17, 18, 23, 34, 27, 15, 9, 6, 8, 5, 25]; // 强数据

        for (idx, &key) in keys.iter().enumerate() {
            let val = (idx + 1) as i32;
            let node = tree.insert(key, val);
            assert_eq!(tree.len(), idx + 1);
            unsafe {
                let found = tree.search_tree(&key);
                assert_eq!(
                    found.unwrap(),
                    node.unwrap(),
                    "Inserted node not found correctly"
                );
                assert_eq!((*found.ptr()).key, key);
                assert_eq!((*found.ptr()).val, val);
            }

            check_red_black_properties(&tree);
        }
    }

    #[test]
    fn test_remove() {
        let mut tree = RBTree::<i32, i32>::new(0, 0);
        let initial_keys = vec![15, 9, 18, 6, 13, 17, 27, 10, 23, 34, 25, 37]; // 强数据
        let remove_keys = vec![18, 25, 15, 6, 13, 37, 27, 17, 34, 9, 10, 23]; // 删除顺序

        for &k in &initial_keys {
            tree.insert(k, k + 1);
        }

        for &key in &remove_keys {
            unsafe {
                let node = tree.search_tree(&key);
                assert_eq!((*node.ptr()).key, key);

                let removed = tree.remove(Some(node.unwrap())).expect("removed");
                assert_eq!(removed.0, key);
                assert_eq!(removed.1, key + 1);

                assert!(
                    tree.search_tree(&key).is_none(),
                    "key {} should be removed",
                    key
                );
            }

            check_red_black_properties(&tree);
        }

        assert_eq!(tree.len, 0);
        assert_eq!(tree.root, tree.nil, "Root should be nil after all removals");
    }

    fn build_test_tree() -> RBTree<i32, i32> {
        let mut tree = RBTree::<i32, i32>::new(0, 0);
        let keys = vec![20, 10, 30, 5, 15, 25, 35];
        for &k in &keys {
            tree.insert(k, k + 100);
        }
        tree
    }

    /// 检查红黑性质
    fn check_red_black_properties<K: Ord + Display, V: Display>(tree: &RBTree<K, V>) {
        unsafe {
            if tree.root == tree.nil {
                return; // 空树Ok
            }

            // 根必须为黑色
            assert_eq!((*tree.root.ptr()).color, Color::Black, "Root must be black");

            // 递归检查
            fn dfs<K: Ord + Display, V: Display>(tree: &RBTree<K, V>, node: Link<K, V>) -> usize {
                if node == tree.nil {
                    return 1; // 空节点黑高为1
                }

                unsafe {
                    let n = node.unwrap().as_ref();

                    // 红色节点的子节点必须是黑色
                    if n.color == Color::Red {
                        if n.lch != tree.nil {
                            assert_eq!(
                                (*n.lch.ptr()).color,
                                Color::Black,
                                "Red node {} has red left child",
                                n.key
                            );
                        }
                        if n.rch != tree.nil {
                            assert_eq!(
                                (*n.rch.ptr()).color,
                                Color::Black,
                                "Red node {} has red right child",
                                n.key
                            );
                        }
                    }

                    // 父子关系一致性
                    if n.lch != tree.nil {
                        assert_eq!(
                            (*n.lch.ptr()).parent,
                            node,
                            "Left child {} parent mismatch",
                            (*n.lch.ptr()).key
                        );
                    }
                    if n.rch != tree.nil {
                        assert_eq!(
                            (*n.rch.ptr()).parent,
                            node,
                            "Right child {} parent mismatch",
                            (*n.rch.ptr()).key
                        );
                    }

                    let left_black = dfs(tree, n.lch);
                    let right_black = dfs(tree, n.rch);
                    assert_eq!(
                        left_black, right_black,
                        "Black-height mismatch at node {}",
                        n.key
                    );

                    left_black + if n.color == Color::Black { 1 } else { 0 }
                }
            }

            dfs(tree, tree.root);
        }
    }
}
