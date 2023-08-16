#[cfg(feature = "with-measure")]
use crate::dom::Measurements;
use crate::dom::{Application, Cmd};
use crate::vdom;
use std::collections::VecDeque;

/// AppContext module pertains only to application state and manages objects that affects it.
/// It has no access to the dom, threads or any of the processing details that Program has to do.
pub(crate) struct AppContext<APP, MSG>
where
    MSG: 'static,
{
    /// holds the user application
    pub(crate) app: APP,

    /// the current vdom representation
    pub(crate) current_vdom: vdom::Node<MSG>,

    /// The MSG that hasn't been applied to the APP yet
    ///
    /// Note: MSG has to be executed in the same succession one by one
    /// since the APP's state may be affected by the previous MSG
    pub(crate) pending_msgs: VecDeque<MSG>,

    /// pending cmds that hasn't been emited yet
    pub(crate) pending_cmds: VecDeque<Cmd<APP, MSG>>,
}

impl<APP, MSG> AppContext<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG> + 'static,
{
    pub fn new(app: APP) -> Self {
        let view = app.view();
        Self {
            app,
            current_vdom: view,
            pending_msgs: VecDeque::new(),
            pending_cmds: VecDeque::new(),
        }
    }
    pub fn init_app(&mut self) -> Cmd<APP, MSG> {
        self.app.init()
    }

    pub fn view(&self) -> vdom::Node<MSG> {
        self.app.view()
    }
    pub fn dynamic_style(&self) -> String {
        self.app.style().join("")
    }

    pub fn static_style(&self) -> String {
        APP::stylesheet().join("")
    }

    pub fn set_current_dom(&mut self, new_vdom: vdom::Node<MSG>) {
        self.current_vdom = new_vdom;
    }

    pub fn current_vdom(&self) -> &vdom::Node<MSG> {
        &self.current_vdom
    }

    #[cfg(feature = "with-measure")]
    pub fn measurements(&self, measurements: Measurements) -> Cmd<APP, MSG> {
        self.app.measurements(measurements).no_render()
    }

    pub fn push_msgs(&mut self, msgs: impl IntoIterator<Item = MSG>) {
        self.pending_msgs.extend(msgs);
    }

    pub fn update_app(&mut self, msg: MSG) -> Cmd<APP, MSG> {
        self.app.update(msg)
    }

    /// return true if there are still pending msgs
    pub fn has_pending_msgs(&self) -> bool {
        !self.pending_msgs.is_empty()
    }

    /// dispatch a single pending msg, return true successfully dispatch one
    /// false if there is no more pending msg
    pub fn dispatch_pending_msg(&mut self) -> bool {
        let pending_msg = self.pending_msgs.pop_front();
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
            self.pending_cmds.push_back(cmd);
            true
        } else {
            false
        }
    }

    pub fn batch_pending_cmds(&mut self) -> Cmd<APP, MSG> {
        Cmd::batch(self.pending_cmds.drain(..))
    }
}
