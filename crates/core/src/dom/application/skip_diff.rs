use crate::vdom::TreePath;

/// specifies how attributes will be skipped
#[derive(Debug, PartialEq, Clone)]
pub enum SkipAttrs {
    /// all attributes are skipped
    All,
    /// skip only the listed indices
    Indices(Vec<usize>),
}

impl SkipAttrs {
    /// dont skip anything
    pub fn none() -> Self {
        Self::Indices(vec![])
    }
}

/// if the expression evaluates to true,
/// diffing at this node will be skipped entirely
#[derive(Debug, PartialEq, Clone)]
pub struct SkipDiff {
    /// shall skip or not
    pub skip_attrs: SkipAttrs,
    /// children skip diff
    pub children: Vec<SkipDiff>,
}

impl SkipDiff {
    /// the skip diff is a block
    pub fn block() -> Self {
        Self {
            skip_attrs: SkipAttrs::none(),
            children: vec![],
        }
    }

    /// return SkipDiff in this path location
    pub fn in_path(&self, path: &TreePath) -> Option<&Self> {
        let mut path = path.clone();
        if path.is_empty() {
            Some(&self)
        } else {
            let idx = path.remove_first();
            if let Some(child) = self.children.get(idx) {
                child.in_path(&path)
            } else {
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
        self.skip_attrs == SkipAttrs::All
    }

    /// return true if this skip diff and its children can be skipped
    pub fn is_skippable_recursive(&self) -> bool {
        self.shall_skip_attributes() && self.children.iter().all(Self::is_skippable_recursive)
    }

    ///
    pub fn shall_skip_node(&self) -> bool {
        self.shall_skip_attributes() && self.children.is_empty()
    }

    /// collapse into 1 skip_if if all the children is skippable
    pub fn collapse_children(self) -> Self {
        let Self {
            skip_attrs,
            children,
        } = self;
        let can_skip_children = children.iter().all(Self::is_skippable_recursive);
        Self {
            skip_attrs,
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
        skip_attrs: if shall {
            SkipAttrs::All
        } else {
            SkipAttrs::none()
        },
        children: children.into_iter().collect(),
    }
}

/// combination of TreePath and SkipDiff
#[derive(Debug)]
pub struct SkipPath {
    pub(crate) path: TreePath,
    pub(crate) skip_diff: Option<SkipDiff>,
}

impl SkipPath {
    pub(crate) fn new(path: TreePath, skip_diff: SkipDiff) -> Self {
        Self {
            path,
            skip_diff: Some(skip_diff),
        }
    }

    pub(crate) fn traverse(&self, idx: usize) -> Self {
        Self {
            path: self.path.traverse(idx),
            skip_diff: if let Some(skip_diff) = self.skip_diff.as_ref() {
                skip_diff.traverse(idx).cloned()
            } else {
                None
            },
        }
    }

    pub(crate) fn backtrack(&self) -> Self {
        Self {
            path: self.path.backtrack(),
            //TODO: here the skip_diff can not back track as we lose that info already
            skip_diff: None,
        }
    }
}
