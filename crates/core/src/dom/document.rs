use crate::dom::document;
use crate::dom::{dom_node::intern, Cmd};
use futures::channel::mpsc;
use wasm_bindgen::{prelude::*, JsCast};

/// Provides function for document related functions
#[derive(Clone, Copy)]
pub struct Document;

impl Document {
    ///
    pub fn on_selectionchange<F, MSG>(mut cb: F) -> Cmd<MSG>
    where
        F: FnMut(Option<web_sys::Selection>) -> MSG + Clone + 'static,
        MSG: 'static,
    {
        let (mut tx, rx) = mpsc::unbounded();
        let closure_cb: Closure<dyn FnMut(web_sys::Event)> =
            Closure::new(move |_event: web_sys::Event| {
                let selection = document().get_selection().ok().flatten();
                let msg = cb(selection);
                tx.start_send(msg).expect("send");
            });
        document()
            .add_event_listener_with_callback(
                intern("selectionchange"),
                closure_cb.as_ref().unchecked_ref(),
            )
            .expect("add event callback");
        Cmd::recurring(rx, closure_cb)
    }
}
