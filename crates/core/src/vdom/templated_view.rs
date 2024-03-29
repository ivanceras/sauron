use crate::dom::SkipDiff;
use std::rc::Rc;
use std::fmt;
use crate::vdom::Node;

/// Templated view
pub struct TemplatedView<MSG>{
    pub(crate) view: Box<Node<MSG>>,
    pub(crate) template: Rc<dyn Fn() -> Node<MSG>>,
    pub(crate) skip_diff: Rc<dyn Fn() -> SkipDiff>,
}

impl<MSG> Clone for TemplatedView<MSG>{
    
    fn clone(&self) -> Self {
        Self {
            view: self.view.clone(),
            template: Rc::clone(&self.template),
            skip_diff: Rc::clone(&self.skip_diff),
        }
    }
}

impl<MSG> fmt::Debug for TemplatedView<MSG>{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        f.debug_struct("TemplatedView")
            .field("view", &self.view)
            .field("template", &(&self.template)())
            .field("skip_diff", &(&self.skip_diff)())
            .finish()
    }
}
