use crate::Component;
use crate::DomUpdater;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[allow(unused)]
pub struct Program<APP, MSG> {
    pub dom_updater: Rc<RefCell<DomUpdater<APP, MSG>>>,
    pub app: Rc<RefCell<APP>>,
}

impl<APP, MSG> Program<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
}
