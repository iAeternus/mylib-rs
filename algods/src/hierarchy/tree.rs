use std::fmt;

use crate::error::AlgodsResult;

/// 树节点 ID
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId {
    /// arena 下标
    pub(crate) index: usize,
    /// 防止悬垂引用或复用旧节点
    pub(crate) generation: u32,
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeId({}, gen={})", self.index, self.generation)
    }
}

/// 有根树接口
pub trait Tree {
    type Value;

    /// 获取根节点 ID
    ///
    /// ## 返回
    /// 根节点的 NodeId
    fn root(&self) -> NodeId;

    /// 检查节点是否仍然有效
    ///
    /// ## 参数
    /// * `node` - 待检查的节点 ID
    ///
    /// ## 返回
    /// 若节点仍在树中存活返回 true，否则 false
    fn contains(&self, node: NodeId) -> bool;

    /// 获取节点的父节点
    ///
    /// ## 参数
    /// * `node` - 待查询节点
    ///
    /// ## 返回
    /// - Ok(Some(parent))：存在父节点  
    /// - Ok(None)：节点是根节点  
    /// - Err(_)：节点无效或已被删除
    fn parent(&self, node: NodeId) -> AlgodsResult<Option<NodeId>>;

    /// 获取节点的直接子节点列表
    ///
    /// ## 参数
    /// * `node` - 待查询节点
    ///
    /// ## 返回
    /// - Ok(&[NodeId])：节点的所有直接子节点  
    /// - Err(_)：节点无效或已被删除
    fn children(&self, node: NodeId) -> AlgodsResult<&[NodeId]>;

    /// 不可变访问节点值
    ///
    /// ## 参数
    /// * `node` - 待访问节点
    ///
    /// ## 返回
    /// - Ok(&Value)：节点的值引用  
    /// - Err(_)：节点无效或已被删除
    fn value(&self, node: NodeId) -> AlgodsResult<&Self::Value>;

    /// 可变访问节点值
    ///
    /// ## 参数
    /// * `node` - 待访问节点
    ///
    /// ## 返回
    /// - Ok(&mut Value)：节点的可变值引用  
    /// - Err(_)：节点无效或已被删除
    fn value_mut(&mut self, node: NodeId) -> AlgodsResult<&mut Self::Value>;

    /// 向节点添加子节点
    ///
    /// ## 参数
    /// * `parent` - 父节点 ID
    /// * `value` - 子节点值
    ///
    /// ## 返回
    /// - Ok(NodeId)：新建子节点的 ID  
    /// - Err(_)：父节点无效或已被删除
    fn add_child(&mut self, parent: NodeId, value: Self::Value) -> AlgodsResult<NodeId>;

    /// 删除节点及其整个子树
    ///
    /// ## 参数
    /// * `node` - 待删除节点
    ///
    /// ## 返回
    /// - Ok(())：删除成功  
    /// - Err(_)：节点无效或尝试删除根节点
    fn remove_subtree(&mut self, node: NodeId) -> AlgodsResult<()>;

    /// 获取当前存活节点数（不含已删除节点）
    fn size(&self) -> usize;

    /// 获取节点父节点（unchecked，节点必须处于存活状态）
    ///
    /// ## Safety
    /// 调用者必须保证节点仍然存活，否则行为未定义
    #[doc(hidden)]
    fn parent_unchecked(&self, node: NodeId) -> Option<NodeId>;

    /// 获取节点子节点（unchecked，节点必须处于存活状态）
    ///
    /// ## Safety
    /// 调用者必须保证节点仍然存活，否则行为未定义
    #[doc(hidden)]
    fn children_unchecked(&self, node: NodeId) -> &[NodeId];
}
