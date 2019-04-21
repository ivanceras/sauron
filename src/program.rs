use crate::Component;
use crate::DomUpdater;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use web_sys::Node;

/// Holds the app and the dom updater
/// This is passed into the event listener and the dispatch program
/// will be called after the event is triggered.
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
        self.dom_updater.borrow_mut().append_mount(self)
    }

    fn start_replace_mount(self: &Rc<Self>) {
        self.dom_updater.borrow_mut().replace_mount(self)
    }

    /// Do the dispatch in request animation frame
    /// to improve performance
    pub fn dispatch(self: &Rc<Self>, msg: MSG) {
        let program_clone = Rc::clone(self);
        let closure_raf: Closure<FnMut() + 'static> = Closure::once(move || {
            program_clone.dispatch_inner(msg);
        });
        crate::request_animation_frame(&closure_raf);
        closure_raf.forget();
    }

    /// This is called when an event is triggered in the html DOM.
    fn dispatch_inner(self: &Rc<Self>, msg: MSG) {
        self.app.borrow_mut().update(msg);
        let view = self.app.borrow().view();
        self.dom_updater
            .borrow_mut()
            .update(self, view);
    }
}
