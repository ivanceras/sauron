use crate::{
    dom::{dom_node::intern, util, window, document, Application, Program, Task},
    vdom::Attribute,
};
use js_sys::Promise;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;

impl<APP, MSG> Program<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG> + Clone + 'static,
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

    /// TODO: only executed once, since the Task Future is droped once done
    /// TODO: this should be a stream, instead of just one-time future
    /// a variant of resize task, but instead of returning Cmd, it is returning Task
    pub fn on_resize_task<F>(mut cb: F) -> Task<MSG>
    where
        F: FnMut(i32, i32) -> MSG + Clone + 'static,
    {
        Task::new(async move {
            let promise = Promise::new(&mut |resolve, _reject| {
                let resize_callback: Closure<dyn FnMut(web_sys::Event)> = Closure::new(move |_| {
                    resolve.call0(&JsValue::NULL).expect("must resolve");
                });
                window()
                    .add_event_listener_with_callback(
                        intern("resize"),
                        resize_callback.as_ref().unchecked_ref(),
                    )
                    .expect("add event callback");
                resize_callback.forget();
            });
            JsFuture::from(promise).await.expect("must await");
            let (window_width, window_height) = util::get_window_size();
            cb(window_width, window_height)
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
