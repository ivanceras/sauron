use crate::dom::task::RecurringTask;
use crate::{
    dom::{document, dom_node::intern, util, window, Application, Program, Task},
    vdom::Attribute,
};
use futures::channel::mpsc;
use wasm_bindgen::{prelude::*, JsCast};

/// Provides function for window related functions
#[derive(Clone, Copy)]
pub struct Window;

impl Window {
    /// Create a recurring Task which will be triggered
    /// everytime the window is resized
    pub fn on_resize<F, MSG>(mut cb: F) -> Task<MSG>
    where
        F: FnMut(i32, i32) -> MSG + Clone + 'static,
        MSG: 'static,
    {
        let (mut tx, rx) = mpsc::unbounded();
        let resize_callback: Closure<dyn FnMut(web_sys::Event)> =
            Closure::new(move |e: web_sys::Event| {
                log::info!("event: {}", e.type_());
                let (w, h) = util::get_window_size();
                let msg = cb(w, h);
                tx.start_send(msg).unwrap();
            });
        window()
            .add_event_listener_with_callback(
                intern("resize"),
                resize_callback.as_ref().unchecked_ref(),
            )
            .expect("add event callback");
        //TODO: this needs to be managed in the Program
        // but components may not have access to the Program
        // TODO: maybe put them in the Task
        // which will get drop together when Task is dropped as well
        resize_callback.forget();

        Task::Recurring(RecurringTask { receiver: rx })
    }
}

impl<APP> Program<APP>
where
    APP: Application,
{
    /// attach event listeners to the window object
    pub fn add_window_event_listeners(&self, event_listeners: Vec<Attribute<APP::MSG>>) {
        self.add_event_listeners(&window(), event_listeners)
            .expect("must add to event listener");
    }

    /// attach event listeners to the document object
    pub fn add_document_event_listeners(&self, event_listeners: Vec<Attribute<APP::MSG>>) {
        self.add_event_listeners(&document(), event_listeners)
            .expect("must add to event listener");
    }

    /// attached a callback and will be triggered when the hash portion of the window location
    /// url is changed
    pub fn on_hashchange<F>(&self, mut cb: F)
    where
        F: FnMut(String) -> APP::MSG + 'static,
    {
        let program = Program::downgrade(self);
        let closure: Closure<dyn FnMut(web_sys::Event)> = Closure::new(move |_| {
            let hash = util::get_location_hash();
            let msg = cb(hash);
            let mut program = program.upgrade().expect("must upgrade");
            program.dispatch(msg);
        });
        window().set_onhashchange(Some(closure.as_ref().unchecked_ref()));
        self.event_closures.borrow_mut().push(closure);
    }
}
