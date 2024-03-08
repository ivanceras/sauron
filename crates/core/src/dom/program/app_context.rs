#[cfg(feature = "with-measure")]
use crate::dom::Measurements;
use crate::dom::{Application, Cmd};
use crate::vdom;
use std::{
    cell::{Ref, RefCell},
    collections::VecDeque,
    rc::Rc,
    rc::Weak,
};

/// AppContext module pertains only to application state and manages objects that affects it.
/// It has no access to the dom, threads or any of the processing details that Program has to do.
pub(crate) struct AppContext<APP, MSG>
where
    MSG: 'static,
{
    /// holds the user application
    pub(crate) app: Rc<RefCell<APP>>,

    /// the current vdom representation
    pub(crate) current_vdom: Rc<RefCell<vdom::Node<MSG>>>,

    /// The MSG that hasn't been applied to the APP yet
    ///
    /// Note: MSG has to be executed in the same succession one by one
    /// since the APP's state may be affected by the previous MSG
    pub(crate) pending_msgs: Rc<RefCell<VecDeque<MSG>>>,

    /// pending cmds that hasn't been emited yet
    pub(crate) pending_cmds: Rc<RefCell<VecDeque<Cmd<APP, MSG>>>>,
}

pub(crate) struct WeakContext<APP, MSG>
where
    MSG: 'static,
{
    pub(crate) app: Weak<RefCell<APP>>,
    pub(crate) current_vdom: Weak<RefCell<vdom::Node<MSG>>>,
    pub(crate) pending_msgs: Weak<RefCell<VecDeque<MSG>>>,
    pub(crate) pending_cmds: Weak<RefCell<VecDeque<Cmd<APP, MSG>>>>,
}

impl<APP, MSG> WeakContext<APP, MSG>
where
    MSG: 'static,
{
    pub(crate) fn upgrade(&self) -> Option<AppContext<APP, MSG>> {
        let app = self.app.upgrade()?;
        let current_vdom = self.current_vdom.upgrade()?;
        let pending_msgs = self.pending_msgs.upgrade()?;
        let pending_cmds = self.pending_cmds.upgrade()?;
        Some(AppContext {
            app,
            current_vdom,
            pending_msgs,
            pending_cmds,
        })
    }
}

impl<APP, MSG> Clone for WeakContext<APP, MSG>
where
    MSG: 'static,
{
    fn clone(&self) -> Self {
        Self {
            app: Weak::clone(&self.app),
            current_vdom: Weak::clone(&self.current_vdom),
            pending_msgs: Weak::clone(&self.pending_msgs),
            pending_cmds: Weak::clone(&self.pending_cmds),
        }
    }
}

impl<APP, MSG> AppContext<APP, MSG>
where
    MSG: 'static,
{
    pub(crate) fn downgrade(this: &Self) -> WeakContext<APP, MSG> {
        WeakContext {
            app: Rc::downgrade(&this.app),
            current_vdom: Rc::downgrade(&this.current_vdom),
            pending_msgs: Rc::downgrade(&this.pending_msgs),
            pending_cmds: Rc::downgrade(&this.pending_cmds),
        }
    }
    pub fn strong_count(&self) -> usize {
        let c1 = Rc::strong_count(&self.app);
        let c2 = Rc::strong_count(&self.current_vdom);
        assert_eq!(c1, c2);
        c1
    }
    pub fn weak_count(&self) -> usize {
        let w1 = Rc::weak_count(&self.app);
        let w2 = Rc::weak_count(&self.current_vdom);
        assert_eq!(w1, w2);
        w1
    }
}

impl<APP, MSG> Clone for AppContext<APP, MSG>
where
    MSG: 'static,
{
    fn clone(&self) -> Self {
        Self {
            app: Rc::clone(&self.app),
            current_vdom: Rc::clone(&self.current_vdom),
            pending_msgs: Rc::clone(&self.pending_msgs),
            pending_cmds: Rc::clone(&self.pending_cmds),
        }
    }
}

impl<APP, MSG> AppContext<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG>,
{
    /// Creates a new app, the view is created
    pub fn new(app: APP) -> Self {
        let view = app.view();
        Self {
            app: Rc::new(RefCell::new(app)),
            current_vdom: Rc::new(RefCell::new(view)),
            pending_msgs: Rc::new(RefCell::new(VecDeque::new())),
            pending_cmds: Rc::new(RefCell::new(VecDeque::new())),
        }
    }

    pub fn init_app(&self) -> Cmd<APP, MSG> {
        self.app.borrow_mut().init()
    }

    pub fn view(&self) -> vdom::Node<MSG> {
        self.app.borrow().view()
    }
    pub fn dynamic_style(&self) -> String {
        self.app.borrow().style().join("")
    }

    pub fn static_style(&self) -> String {
        APP::stylesheet().join("")
    }

    pub fn set_current_dom(&mut self, new_vdom: vdom::Node<MSG>) {
        *self.current_vdom.borrow_mut() = new_vdom;
    }

    pub fn current_vdom(&self) -> Ref<'_, vdom::Node<MSG>> {
        self.current_vdom.borrow()
    }

    #[cfg(feature = "with-measure")]
    pub fn measurements(&self, measurements: Measurements) -> Cmd<APP, MSG> {
        self.app.borrow().measurements(measurements).no_render()
    }

    pub fn push_msgs(&mut self, msgs: impl IntoIterator<Item = MSG>) {
        self.pending_msgs.borrow_mut().extend(msgs);
    }

    pub fn update_app(&mut self, msg: MSG) -> Cmd<APP, MSG> {
        self.app.borrow_mut().update(msg)
    }

    /// return true if there are still pending msgs
    pub fn has_pending_msgs(&self) -> bool {
        !self.pending_msgs.borrow().is_empty()
    }

    /// return the current number of pending msgs
    pub fn pending_msgs_count(&self) -> usize {
        self.pending_msgs.borrow().len()
    }

    /// dispatch a single pending msg, return true successfully dispatch one
    /// false if there is no more pending msg
    pub fn dispatch_pending_msg(&mut self) -> bool {
        let pending_msg = self.pending_msgs.borrow_mut().pop_front();
        let cmd = if let Some(pending_msg) = pending_msg {
            // Note: each MSG needs to be executed one by one in the same order
            // as APP's state can be affected by the previous MSG
            let cmd = self.update_app(pending_msg);
            Some(cmd)
        } else {
            None
        };

        if let Some(cmd) = cmd {
            // we put the cmd in the pending_cmd queue
            self.pending_cmds.borrow_mut().push_back(cmd);
            true
        } else {
            false
        }
    }

    pub fn batch_pending_cmds(&mut self) -> Cmd<APP, MSG> {
        Cmd::batch(self.pending_cmds.borrow_mut().drain(..))
    }
}
