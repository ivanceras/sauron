use crate::{
    dom::{document, dom_node::intern, util, window, Application, Program, Task},
    vdom::Attribute,
};
use wasm_bindgen::{prelude::*, JsCast};
use futures::channel::mpsc;
use crate::dom::task::RecurringTask;

impl<APP, MSG> Program<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG>,
{
    /// attach event listeners to the window object
    pub fn add_window_event_listeners(&self, event_listeners: Vec<Attribute<MSG>>) {
        self.add_event_listeners(&window(), event_listeners)
            .expect("must add to event listener");
    }

    /// attach event listeners to the document object
    pub fn add_document_event_listeners(&self, event_listeners: Vec<Attribute<MSG>>) {
        self.add_event_listeners(&document(), event_listeners)
            .expect("must add to event listener");
    }

    /// Creates a Cmd in which the MSG will be emitted
    /// whenever the browser is resized
    pub fn on_resize<F>(&self, mut cb: F)
    where
        F: FnMut(i32, i32) -> MSG + Clone + 'static,
    {
        let program = Program::downgrade(&self);
        let closure: Closure<dyn FnMut(web_sys::Event)> = Closure::new(move |_| {
            let (window_width, window_height) = util::get_window_size();
            let msg = cb(window_width, window_height);
            let mut program = program.upgrade().expect("must upgrade");
            program.dispatch(msg);
        });
        window()
            .add_event_listener_with_callback(intern("resize"), closure.as_ref().unchecked_ref())
            .expect("resize callback");
        self.event_closures.borrow_mut().push(closure);
    }


    /// a recurring task
    pub fn on_resize_task<F>(mut cb: F) -> Task<MSG>
    where
        F: FnMut(i32, i32) -> MSG + Clone + 'static,
    {
        let (mut tx, rx) = mpsc::unbounded();
        let resize_callback: Closure<dyn FnMut(web_sys::Event)> = Closure::new(move |e: web_sys::Event| {
            log::info!("event: {}",e.type_());
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
        resize_callback.forget();

        Task::Recurring(
            RecurringTask{
                receiver: rx,
        })
    }

    /// attached a callback and will be triggered when the hash portion of the window location
    /// url is changed
    pub fn on_hashchange<F>(&self, mut cb: F)
    where
        F: FnMut(String) -> MSG + 'static,
    {
        let program = Program::downgrade(&self);
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
