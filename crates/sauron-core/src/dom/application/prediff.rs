use mt_dom::TreePath;
use std::fmt;

///
pub struct PreDiff {
    expr: Box<dyn Fn() -> bool>,
    children: Vec<PreDiff>,
}

impl fmt::Debug for PreDiff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}", (self.expr)())?;
        write!(f, ",[")?;
        for child in &self.children {
            child.fmt(f)?;
        }
        write!(f, "])")?;
        Ok(())
    }
}

impl PreDiff {
    /// new
    pub fn new(val: bool, children: impl IntoIterator<Item = Self>) -> Self {
        Self {
            expr: Box::new(move || val),
            children: children.into_iter().collect(),
        }
    }

    ///
    pub fn none() -> Self {
        PreDiff {
            expr: Box::new(|| false),
            children: vec![],
        }
    }

    ///
    pub fn traverse(evals: &[PreDiff]) -> Vec<TreePath> {
        let root = TreePath::root();
        if evals.len() == 1 {
            Self::traverse_recursive(&evals[0], root)
        } else {
            Self::traverse_list(evals, root)
        }
    }

    ///
    fn traverse_list(evals: &[PreDiff], current: TreePath) -> Vec<TreePath> {
        let mut paths = vec![];
        for (i, eval) in evals.iter().enumerate() {
            let more_paths = eval.traverse_recursive(current.traverse(i));
            paths.extend(more_paths);
        }
        paths
    }

    fn traverse_recursive(&self, current: TreePath) -> Vec<TreePath> {
        let mut paths = vec![];
        if (self.expr)() {
            paths.push(current.clone());
        }
        let more_paths = Self::traverse_list(&self.children, current);
        paths.extend(more_paths);
        paths
    }
}

/// evaluate check
pub fn diff_if(val: bool, children: impl IntoIterator<Item = PreDiff>) -> PreDiff {
    PreDiff::new(val, children)
}
