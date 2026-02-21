/// 节点索引
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeIndex<Idx = usize>(pub Idx);

/// 边索引
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EdgeIndex<Idx = usize>(pub Idx);

/// 方向
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Outgoing, // 出边
    Incoming, // 入边
}

// 图类型标记
pub trait EdgeType {
    const DIRECTED: bool;
}

pub struct Directed;
#[allow(dead_code)]
pub struct Undirected;

impl EdgeType for Directed {
    const DIRECTED: bool = true;
}

impl EdgeType for Undirected {
    const DIRECTED: bool = false;
}

impl<Idx> NodeIndex<Idx>
where
    Idx: From<usize> + Into<usize>,
{
    pub fn end() -> Self {
        NodeIndex(Idx::from(usize::MAX))
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0.into()
    }
}

impl<Idx> EdgeIndex<Idx>
where
    Idx: From<usize> + Into<usize>,
{
    pub fn end() -> Self {
        EdgeIndex(Idx::from(usize::MAX))
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0.into()
    }
}
