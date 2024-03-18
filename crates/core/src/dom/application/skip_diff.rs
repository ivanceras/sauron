use crate::vdom::TreePath;

///
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Marker {
    /// anything else in the valid block
    Block,
}

/// if the expression evaluates to true,
/// diffing at this node will be skipped entirely
#[derive(Debug, PartialEq, Clone)]
pub struct SkipDiff {
    /// shall skip or not
    pub shall: bool,
    /// marker for template blocks
    pub marker: Option<Marker>,
    /// children skip diff
    pub children: Vec<SkipDiff>,
}

impl SkipDiff {
    /// new
    pub fn new(shall: bool, children: impl IntoIterator<Item = Self>) -> Self {
        Self {
            shall,
            marker: None,
            children: children.into_iter().collect(),
        }
    }

    /// the skip diff is a block
    pub fn block() -> Self {
        Self {
            shall: false,
            marker: Some(Marker::Block),
            children: vec![],
        }
    }


    /// return SkipDiff in this path location
    pub fn in_path(&self, path: &TreePath) -> Option<&Self> {
        let mut path = path.clone();
        if path.is_empty(){
            Some(&self)
        }else{
            let idx = path.remove_first();
            if let Some(child) = self.children.get(idx){
                child.in_path(&path)
            }else{
                None
            }
        }
    }


    /// get the skip diff at this child index
    pub fn traverse(&self, idx: usize) -> Option<&Self> {
        self.children.get(idx)
    }

    /// check if shall skip diffing attributes at this path
    /// if the path does not coincide in this skip diff, then by default it is skipped
    pub fn shall_skip_attributes(&self) -> bool {
        self.shall
    }


    /// return true if this skip diff and its children can be skipped
    pub fn is_skippable_recursive(&self) -> bool {
        self.shall && self.children.iter().all(Self::is_skippable_recursive)
    }

    /// 
    pub fn shall_skip_node(&self) -> bool {
        self.shall && self.children.is_empty()
    }

    /// collapse into 1 skip_if if all the children is skippable
    pub fn collapse_children(self) -> Self {
        let Self {
            shall,
            children,
            marker,
        } = self;
        let can_skip_children = children.iter().all(Self::is_skippable_recursive);
        Self {
            shall,
            marker,
            children: if can_skip_children {
                vec![]
            } else {
                children.into_iter().map(Self::collapse_children).collect()
            },
        }
    }
}

/// skip diffing the node is the val is true
pub fn skip_if(shall: bool, children: impl IntoIterator<Item = SkipDiff>) -> SkipDiff {
    SkipDiff {
        shall,
        marker: None,
        children: children.into_iter().collect(),
    }
}


