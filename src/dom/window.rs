use crate::{
    dom::created_node::create_closure_wrap, Attribute, Cmd, Component,
};
use wasm_bindgen::{self, prelude::*, JsCast};
use web_sys::EventTarget;

pub struct Window;

impl Window {
    pub fn add_event_listeners<APP, MSG>(
        event_listeners: Vec<Attribute<MSG>>,
    ) -> Cmd<APP, MSG>
    where
        APP: Component<MSG> + 'static,
    {
        Cmd::new(move |program| {
            let window = crate::window();
            let window: &EventTarget = window
                .dyn_ref()
                .expect("unable to cast window to event target");

            for event_attr in event_listeners.iter() {
                let event_str = event_attr.name;
                let callback = event_attr
                    .value
                    .get_callback()
                    .expect("expecting a callback");

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
        })
    }
}
