use crate::vdom::TreePath;
use std::fmt;

/// if the expression evaluates to true,
/// diffing at this node will be skipped entirely
pub struct SkipDiff {
    shall: bool,
    children: Vec<SkipDiff>,
}

impl PartialEq for SkipDiff {
    fn eq(&self, other: &Self) -> bool {
        self.shall == other.shall && self.children == other.children
    }
}

impl fmt::Debug for SkipDiff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},", self.shall)?;
        f.debug_list().entries(self.children.iter()).finish();
        write!(f, ")")?;
        Ok(())
    }
}

impl SkipDiff {
    /// new
    pub fn new(shall: bool, children: impl IntoIterator<Item = Self>) -> Self {
        Self {
            shall,
            children: children.into_iter().collect(),
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
        let Self { shall, children } = self;
        let can_skip_children = children.iter().all(Self::is_skippable_recursive);
        Self {
            shall,
            children: if can_skip_children {
                vec![]
            } else {
                children.into_iter().map(Self::collapse_children).collect()
            },
        }
    }
}

/// skip diffing the node is the val is true
pub fn skip_if(val: bool, children: impl IntoIterator<Item = SkipDiff>) -> SkipDiff {
    SkipDiff::new(val, children)
}
