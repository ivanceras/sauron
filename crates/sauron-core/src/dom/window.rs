use crate::{
    dom::{self, util, Task, Program},
    vdom::Attribute,
    Application,
};
use wasm_bindgen::{self, prelude::*, JsCast};
use web_sys::EventTarget;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;
use std::rc::Rc;
use std::cell::RefCell;

impl<APP, MSG> Program<APP, MSG>
where
    MSG: 'static,
    APP: Application<MSG> + 'static,
{
    /// attach an event listender to the window
    /// TODO: maybe rename to add_window_event_listeners
    pub fn add_event_listeners(&self, event_listeners: Vec<Attribute<MSG>>) {
        let window = crate::window();
        let window: &EventTarget = window
            .dyn_ref()
            .expect("unable to cast window to event target");

        for event_attr in event_listeners.into_iter() {
            let event_str = event_attr.name();
            for event_cb in event_attr.value() {
                let listener = event_cb.as_event_listener().expect("expecting a callback");
                let listener = listener.clone();
                let program = self.clone();
                let closure: Closure<dyn FnMut(web_sys::Event)> =
                Closure::new(move|event: web_sys::Event| {
                    let msg = listener.emit(dom::Event::from(event));
                    program.dispatch(msg);
                });

                window
                    .add_event_listener_with_callback(
                        event_str,
                        closure.as_ref().unchecked_ref(),
                    )
                    .expect("Unable to attached event listener");
                self.event_closures.borrow_mut().push(closure);
            }
        }
    }


    /// Creates a Cmd in which the MSG will be emitted
    /// whenever the browser is resized
    pub fn on_resize<F>(&self, mut cb: F)
    where
        F: FnMut(i32, i32) -> MSG + Clone + 'static,
    {
        let program = self.clone();
        let closure: Closure<dyn FnMut(web_sys::Event)> =
            Closure::new(move|_| {
                let (window_width, window_height) = util::get_window_size();
                let msg = cb(window_width, window_height);
                program.dispatch(msg);
            });
        crate::window().set_onresize(Some(closure.as_ref().unchecked_ref()));
        self.event_closures.borrow_mut().push(closure);
    }

    /// TODO: only executed once, since the Task Future is droped once done
    /// TODO: this should be a stream, instead of just one-time future
    /// a variant of resize task, but instead of returning Cmd, it is returning Task
    pub fn on_resize_task<F>(cb: F) -> Task<MSG>
    where
        F: FnMut(i32, i32) -> MSG + Clone + 'static,
    {
        Task::new(async move{
            let msg_store: Rc<RefCell<Option<MSG>>> = Rc::new(RefCell::new(None));
            let msg_weak = Rc::downgrade(&msg_store);
            let promise = Promise::new(&mut |resolve, _reject|{
                let mut cb = cb.clone();
                let msg_store = Rc::clone(&msg_store);
                let resize_callback: Closure<dyn FnMut(web_sys::Event)> =
                    Closure::new(move|_| {
                        let (window_width, window_height) = util::get_window_size();
                        let msg = cb(window_width, window_height);
                        *msg_store.borrow_mut() = Some(msg);
                        resolve.call0(&JsValue::NULL).expect("must resolve");
                    });
                crate::window().set_onresize(Some(resize_callback.as_ref().unchecked_ref()));
                resize_callback.forget();
            });
            JsFuture::from(promise).await.expect("must await");
            let msg = msg_weak.upgrade()
                .expect("upgrade msg_weak")
                .borrow_mut()
                .take();
            msg.expect("must contain the MSG here")
        })
    }

    /// attached a callback and will be triggered when the hash portion of the window location
    /// url is changed
    pub fn on_hashchange<F>(&self, mut cb: F)
    where
        F: FnMut(String) -> MSG + 'static,
    {
        let program = self.clone();
        let closure: Closure<dyn FnMut(web_sys::Event)> =
            Closure::new(move |_| {
                let hash = util::get_location_hash();
                let msg = cb(hash);
                program.dispatch(msg);
            });
        crate::window().set_onhashchange(Some(closure.as_ref().unchecked_ref()));
        self.event_closures.borrow_mut().push(closure);
    }


}

