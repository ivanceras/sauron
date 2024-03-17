use crate::vdom::TreePath;

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


    #[allow(unused)]
    /// check if this path is in the skip diff way of patch
    pub(crate) fn in_path(&self, path: &TreePath) -> bool {
        let mut path = path.clone();
        if path.is_empty(){
            true
        }else{
            let idx = path.remove_first();
            if let Some(child) = self.children.get(idx){
                child.in_path(&path)
            }else{
                false
            }
        }
    }

    /// return the skip diff at this path
    pub fn get_skip_diff(&self, path: &TreePath) -> Option<&Self> {
        let mut path = path.clone();
        if path.is_empty(){
            Some(self)
        }else{
            let idx = path.remove_first();
            if let Some(child) = self.children.get(idx){
                child.get_skip_diff(&path)
            }else{
                None
            }
        }
    }

    /// check if shall skip diffing attributes at this path
    /// if the path does not coincide in this skip diff, then by default it is skipped
    pub fn shall_skip_attributes(&self, path: &TreePath) -> bool {
        let mut path = path.clone();
        if path.is_empty(){
            self.shall
        }else{
            let idx = path.remove_first();
            if let Some(child) = self.children.get(idx){
                child.shall_skip_attributes(&path)
            }else{
                true
            }
        }
    }

    /// skip this node if can skip the attributes and the children is empty
    /// NOTE: we are not evaluating if all the children can be skip, since it is already dealt
    /// (collapsed) in the extraction of skip_diff
    pub fn shall_skip_node(&self, path: &TreePath) -> bool {
        self.shall_skip_attributes(&path) && self.children.is_empty()
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
