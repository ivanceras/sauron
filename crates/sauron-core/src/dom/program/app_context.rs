#[cfg(feature = "with-measure")]
use crate::dom::Measurements;
use crate::dom::{Application, Cmd};
use crate::vdom;
use std::{
    cell::{Ref, RefCell},
    collections::VecDeque,
    sync::Arc,
    sync::Weak,
    sync::RwLock,
};
use std::sync::RwLockReadGuard;

/// AppContext module pertains only to application state and manages objects that affects it.
/// It has no access to the dom, threads or any of the processing details that Program has to do.
pub(crate) struct AppContext<APP, MSG>
where
    MSG: 'static,
{
    /// holds the user application
    pub(crate) app: Arc<RwLock<APP>>,

    /// the current vdom representation
    pub(crate) current_vdom: Arc<RwLock<vdom::Node<MSG>>>,

    /// The MSG that hasn't been applied to the APP yet
    ///
    /// Note: MSG has to be executed in the same succession one by one
    /// since the APP's state may be affected by the previous MSG
    pub(crate) pending_msgs: Arc<RwLock<VecDeque<MSG>>>,

    /// pending cmds that hasn't been emited yet
    pub(crate) pending_cmds: Arc<RwLock<VecDeque<Cmd<APP, MSG>>>>,
}

pub(crate) struct WeakContext<APP, MSG>
where
    MSG: 'static,
{
    pub(crate) app: Weak<RwLock<APP>>,
    pub(crate) current_vdom: Weak<RwLock<vdom::Node<MSG>>>,
    pub(crate) pending_msgs: Weak<RwLock<VecDeque<MSG>>>,
    pub(crate) pending_cmds: Weak<RwLock<VecDeque<Cmd<APP, MSG>>>>,
}

impl<APP, MSG> WeakContext<APP, MSG>
where
    MSG: 'static,
{
    pub(crate) fn upgrade(&self) -> Option<AppContext<APP, MSG>> {
        if let Some(app) = self.app.upgrade() {
            if let Some(current_vdom) = self.current_vdom.upgrade() {
                if let Some(pending_msgs) = self.pending_msgs.upgrade() {
                    if let Some(pending_cmds) = self.pending_cmds.upgrade() {
                        return Some(AppContext {
                            app,
                            current_vdom,
                            pending_msgs,
                            pending_cmds,
                        });
                    }
                }
            }
        }
        None
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
            app: Arc::downgrade(&this.app),
            current_vdom: Arc::downgrade(&this.current_vdom),
            pending_msgs: Arc::downgrade(&this.pending_msgs),
            pending_cmds: Arc::downgrade(&this.pending_cmds),
        }
    }
    pub fn strong_count(&self) -> usize {
        let c1 = Arc::strong_count(&self.app);
        let c2 = Arc::strong_count(&self.current_vdom);
        assert_eq!(c1, c2);
        c1
    }
    pub fn weak_count(&self) -> usize {
        let w1 = Arc::weak_count(&self.app);
        let w2 = Arc::weak_count(&self.current_vdom);
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
            app: Arc::clone(&self.app),
            current_vdom: Arc::clone(&self.current_vdom),
            pending_msgs: Arc::clone(&self.pending_msgs),
            pending_cmds: Arc::clone(&self.pending_cmds),
        }
    }
}

impl<APP, MSG> AppContext<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG> + Clone + 'static,
{
    pub fn new(app: APP) -> Self {
        let view = app.view();
        Self {
            app: Arc::new(RwLock::new(app)),
            current_vdom: Arc::new(RwLock::new(view)),
            pending_msgs: Arc::new(RwLock::new(VecDeque::new())),
            pending_cmds: Arc::new(RwLock::new(VecDeque::new())),
        }
    }
    pub fn init_app(&self) -> Cmd<APP, MSG> {
        let mut app = self.app.write().expect("poisoned");
            app.init()
    }

    pub fn view(&self) -> vdom::Node<MSG> {
        self.app.read().expect("poisoned").view()
    }
    pub fn dynamic_style(&self) -> String {
        self.app.read().expect("poisoned").style().join("")
    }

    pub fn static_style(&self) -> String {
        APP::stylesheet().join("")
    }

    pub fn set_current_dom(&mut self, new_vdom: vdom::Node<MSG>) {
        let mut current_vdom = self.current_vdom.write().expect("poisoned");
            *current_vdom = new_vdom;
    }

    pub fn current_vdom(&self) -> RwLockReadGuard<'_, vdom::Node<MSG>> {
        self.current_vdom.read().expect("poisoned")
    }

    #[cfg(feature = "with-measure")]
    pub fn measurements(&self, measurements: Measurements) -> Cmd<APP, MSG> {
        self.app.borrow().measurements(measurements).no_render()
    }

    pub fn push_msgs(&mut self, msgs: impl IntoIterator<Item = MSG>) {
        let mut pending_msgs = self.pending_msgs.write().expect("poisoned");
        pending_msgs.extend(msgs);
    }

    pub fn update_app(&mut self, msg: MSG) -> Cmd<APP, MSG> {
        let mut app = self.app.write().expect("poisoned");
        app.update(msg)
    }

    /// return true if there are still pending msgs
    pub fn has_pending_msgs(&self) -> bool {
        !self.pending_msgs.read().expect("poisoned").is_empty()
    }

    /// dispatch a single pending msg, return true successfully dispatch one
    /// false if there is no more pending msg
    pub fn dispatch_pending_msg(&mut self) -> bool {
        let pending_msg = {
            let mut pending_msgs = self.pending_msgs.write().expect("poisoned");
            pending_msgs.pop_front()
        };
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
            let mut pending_cmds = self.pending_cmds.write().expect("poisoned");
                pending_cmds.push_back(cmd);
            true
        } else {
            false
        }
    }

    pub fn batch_pending_cmds(&mut self) -> Cmd<APP, MSG> {
        Cmd::batch(self.pending_cmds.write().expect("poisoned").drain(..))
    }
}
