use crate::dom::SkipDiff;
use crate::vdom::Node;
use std::fmt;
use std::rc::Rc;

/// Templated view
pub struct TemplatedView<MSG> {
    /// The view node
    pub view: Box<Node<MSG>>,
    /// The skip_diff generated from the view node
    pub skip_diff: Rc<dyn Fn() -> SkipDiff>,
}

impl<MSG> Clone for TemplatedView<MSG> {
    fn clone(&self) -> Self {
        Self {
            view: self.view.clone(),
            skip_diff: Rc::clone(&self.skip_diff),
        }
    }
}

impl<MSG> fmt::Debug for TemplatedView<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TemplatedView")
            .field("view", &self.view)
            .field("skip_diff", &(self.skip_diff)())
            .finish()
    }
}
