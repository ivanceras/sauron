use crate::Component;
use crate::DomUpdater;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use web_sys::Node;

pub struct Program<APP, MSG> {
    pub app: Rc<RefCell<APP>>,
    pub dom_updater: Rc<RefCell<DomUpdater<APP, MSG>>>,
}

impl<APP, MSG> Program<APP, MSG>
where
    MSG: Clone + Debug + 'static,
    APP: Component<MSG> + 'static,
{
    fn new(app: APP, root_node: &Node) -> Rc<Self> {
        let dom_updater: DomUpdater<APP, MSG> = DomUpdater::new(app.view(), root_node);
        let program = Program {
            app: Rc::new(RefCell::new(app)),
            dom_updater: Rc::new(RefCell::new(dom_updater)),
        };
        Rc::new(program)
    }
    pub fn new_replace_mount(app: APP, root_node: &Node) -> Rc<Self> {
        let program = Self::new(app, root_node);
        program.start_replace_mount();
        program
    }

    pub fn new_append_mount(app: APP, root_node: &Node) -> Rc<Self> {
        let program = Self::new(app, root_node);
        program.start_append_mount();
        program
    }

    fn start_append_mount(self: &Rc<Self>) {
        self.dom_updater.borrow_mut().append_mount(&self)
    }

    fn start_replace_mount(self: &Rc<Self>) {
        self.dom_updater.borrow_mut().replace_mount(&self)
    }
}
