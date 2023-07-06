use crate::{
    dom::{created_node::create_closure_wrap, Cmd, Task},
    vdom::Attribute,
    Application,
};
use std::fmt::Debug;
use wasm_bindgen::{self, prelude::*, JsCast};
use web_sys::{EventTarget, ScrollToOptions};
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;
use std::rc::Rc;
use std::cell::RefCell;

/// Provides access to the Browser window
#[derive(Copy, Clone, Debug)]
pub struct Window;

impl Window {
    /// attach an event listender to the window
    pub fn add_event_listeners<APP, MSG>(event_listeners: Vec<Attribute<MSG>>) -> Cmd<APP, MSG>
    where
        APP: Application<MSG> + 'static,
        MSG: 'static,
    {
        Cmd::new(move |program| {
            let window = crate::window();
            let window: &EventTarget = window
                .dyn_ref()
                .expect("unable to cast window to event target");

            for event_attr in event_listeners.iter() {
                let event_str = event_attr.name();
                for event_cb in event_attr.value() {
                    let callback = event_cb.as_event_listener().expect("expecting a callback");

                    let callback_wrapped: Closure<dyn FnMut(web_sys::Event)> =
                        create_closure_wrap(&program, callback);
                    window
                        .add_event_listener_with_callback(
                            event_str,
                            callback_wrapped.as_ref().unchecked_ref(),
                        )
                        .expect("Unable to attached event listener");

                    callback_wrapped.forget();
                }
            }
        })
    }

    /// set the title of the document
    pub fn set_title(title: &str) {
        crate::document().set_title(title);
    }

    /// Creates a Cmd in which the MSG will be emitted
    /// whenever the browser is resized
    pub fn on_resize<F, APP, MSG>(cb: F) -> Cmd<APP, MSG>
    where
        F: Fn(i32, i32) -> MSG + Clone + 'static,
        MSG: 'static,
        APP: Application<MSG> + 'static,
    {
        let cmd: Cmd<APP, MSG> = Cmd::new(move |program| {
            let resize_callback: Closure<dyn Fn(web_sys::Event)> =
                Closure::wrap(Box::new(move |_| {
                    let (window_width, window_height) = Self::get_size();
                    let msg = cb(window_width, window_height);
                    program.dispatch(msg);
                }));
            crate::window().set_onresize(Some(resize_callback.as_ref().unchecked_ref()));
            resize_callback.forget();
        });
        cmd
    }

    /// a variant of resize task, but instead of returning Cmd, it is returning Task
    pub fn on_resize_task<F, MSG>(cb: F) -> Task<MSG>
    where
        F: Fn(i32, i32) -> MSG + Clone + 'static,
        MSG: 'static,
    {
        Task::new(async move{
            let stored: Rc<RefCell<Option<MSG>>> = Rc::new(RefCell::new(None));
            let stored_weak = Rc::downgrade(&stored);
            let promise = Promise::new(&mut move|resolve, _reject|{
                let cb = cb.clone();
                let stored = Rc::clone(&stored);
                let resize_callback: Closure<dyn Fn(web_sys::Event)> =
                    Closure::wrap(Box::new(move |_| {
                        let (window_width, window_height) = Self::get_size();
                        let msg = cb(window_width, window_height);
                        *stored.borrow_mut() = Some(msg);
                        resolve.call0(&JsValue::NULL).expect("must resolve");
                    }));
                crate::window().set_onresize(Some(resize_callback.as_ref().unchecked_ref()));
                resize_callback.forget();
            });
            JsFuture::from(promise).await.expect("must await");
            let refcell = stored_weak.upgrade().expect("upgrade stored_weak");
            let opt = refcell.borrow_mut().take();
            opt.expect("must contain the MSG here")
        })
    }

    /// attached a callback and will be triggered when the hash portion of the window location
    /// url is changed
    pub fn on_hashchange<F, APP, MSG>(cb: F) -> Cmd<APP, MSG>
    where
        F: Fn(String) -> MSG + Clone + 'static,
        MSG: 'static,
        APP: Application<MSG> + 'static,
    {
        let cmd: Cmd<APP, MSG> = Cmd::new(move |program| {
            let hashchange_callback: Closure<dyn Fn(web_sys::Event)> =
                Closure::wrap(Box::new(move |_| {
                    let hash = Self::get_hash();
                    let msg = cb(hash);
                    program.dispatch(msg);
                }));
            crate::window().set_onhashchange(Some(hashchange_callback.as_ref().unchecked_ref()));
            hashchange_callback.forget();
        });
        cmd
    }

    /// return the size of the browser at this moment
    pub fn get_size() -> (i32, i32) {
        let window = crate::window();
        let window_width = window
            .inner_width()
            .expect("unable to get window width")
            .as_f64()
            .expect("cant convert to f64");
        let window_height = window
            .inner_height()
            .expect("unable to get height")
            .as_f64()
            .expect("cant convert to f64");
        (window_width as i32, window_height as i32)
    }

    /// return the hash part of the browser current url location
    /// The hash part are the text right after the `#` sign
    pub fn get_hash() -> String {
        let window = crate::window();
        window.location().hash().expect("must have a hash")
    }

    /// scroll the browser to the top of the document
    pub fn scroll_to_top<APP, MSG>() -> Cmd<APP, MSG>
    where
        APP: Application<MSG> + 'static,
        MSG: 'static,
    {
        Cmd::new(|_program| {
            let mut options = ScrollToOptions::new();
            options.top(0.0);
            options.left(0.0);
            crate::window().scroll_to_with_scroll_to_options(&options);
        })
    }

    /// set the browser location hash
    pub fn set_location_hash(hash: &str) {
        let window = crate::window();
        let location = window.location();
        location.set_hash(hash).expect("must set the location hash");
    }
}
