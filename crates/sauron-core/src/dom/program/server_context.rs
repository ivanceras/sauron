#[cfg(feature = "with-measure")]
use crate::dom::Measurements;
use crate::dom::{Application, Cmd};
use crate::vdom;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub(crate) struct ServerContext<APP, MSG>
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

impl<APP, MSG> Clone for ServerContext<APP, MSG>
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

impl<APP, MSG> ServerContext<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG> + 'static,
{
    pub fn init_app(&self) -> Vec<Cmd<APP, MSG>> {
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

    pub fn set_current_dom(&self, new_vdom: vdom::Node<MSG>) {
        *self.current_vdom.borrow_mut() = new_vdom;
    }

    #[cfg(feature = "with-measure")]
    pub fn measurements(&self, measurements: Measurements) -> Cmd<APP, MSG> {
        self.app.borrow().measurements(measurements).no_render()
    }

    pub fn dispatch_multiple(&self, msgs: impl IntoIterator<Item = MSG>) {
        self.pending_msgs.borrow_mut().extend(msgs);
    }

    pub fn update(&self, msg: MSG) -> Cmd<APP, MSG> {
        self.app.borrow_mut().update(msg)
    }

    /// return true if there are still pending msgs
    pub fn has_pending_msgs(&self) -> bool {
        !self.pending_msgs.borrow().is_empty()
    }

    /// dispatch a single pending msg, return true successfully dispatch one
    /// false if there is no more pending msg
    pub fn dispatch_pending_msg(&self) -> bool {
        if let Some(pending_msg) = self.pending_msgs.borrow_mut().pop_front() {
            // Note: each MSG needs to be executed one by one in the same order
            // as APP's state can be affected by the previous MSG
            let cmd = self.update(pending_msg);

            // we put the cmd in the pending_cmd queue
            self.pending_cmds.borrow_mut().push_back(cmd);
            true
        } else {
            false
        }
    }

    pub fn batch_pending_cmds(&self) -> Cmd<APP, MSG> {
        let mut all_cmd = vec![];
        let mut pending_cmds = self.pending_cmds.borrow_mut();
        while let Some(cmd) = pending_cmds.pop_front() {
            all_cmd.push(cmd);
        }
        log::trace!("batching {} cmds", all_cmd.len());
        // we can execute all the cmd here at once
        Cmd::batch(all_cmd)
    }
}
