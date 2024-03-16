use crate::vdom::TreePath;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Marker{
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
            children: vec![]
        }
    }

    ///
    pub fn traverse(&self) -> Vec<TreePath> {
        self.traverse_recursive(TreePath::root())
    }

    /// traverse the skip diff and return a list of TreePath that will be evaluated
    /// by the program
    fn traverse_recursive(&self, current: TreePath) -> Vec<TreePath> {
        let mut paths = vec![];
        // if this SkipDiff evaluates to false, include it in the treepath to be diff
        if !self.shall {
            paths.push(current.clone());
        }

        for (i, eval) in self.children.iter().enumerate() {
            let more_paths = eval.traverse_recursive(current.traverse(i));
            paths.extend(more_paths);
        }
        paths
    }

    /// return true if this skip diff and its children can be skipped
    fn is_skippable_recursive(&self) -> bool {
        self.shall && self.children.iter().all(Self::is_skippable_recursive)
    }

    /// collapse into 1 skip_if if all the children is skippable
    pub fn collapse_children(self) -> Self {
        let Self { shall, children, marker} = self;
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
    SkipDiff{
        shall, 
        marker: None,
        children: children.into_iter().collect()
    }
}
