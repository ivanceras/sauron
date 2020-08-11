use crate::{
    dom::created_node::create_closure_wrap, Attribute, Cmd, Component,
};
use std::fmt::Debug;
use wasm_bindgen::{self, prelude::*, JsCast};
use web_sys::EventTarget;

/// Provides access to the Browser window
#[derive(Copy, Clone, Debug)]
pub struct Window;

impl Window {
    /// attach an event listender to the window
    pub fn add_event_listeners<APP, MSG>(
        event_listeners: Vec<Attribute<MSG>>,
    ) -> Cmd<APP, MSG>
    where
        APP: Component<MSG> + 'static,
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
                    let callback =
                        event_cb.get_callback().expect("expecting a callback");

                    let closure_wrap: Closure<dyn FnMut(web_sys::Event)> =
                        create_closure_wrap(&program, &callback);
                    window
                        .add_event_listener_with_callback(
                            event_str,
                            closure_wrap.as_ref().unchecked_ref(),
                        )
                        .expect("Unable to attached event listener");

                    closure_wrap.forget();
                }
            }
        })
    }

    /// set the title of the document
    pub fn set_title(title: &str) {
        crate::document().set_title(title);
    }
}
