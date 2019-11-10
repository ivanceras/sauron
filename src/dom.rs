use sauron_vdom;

use wasm_bindgen::{
    JsCast,
    JsValue,
};
use web_sys;

pub mod apply_patches;
mod browser;
mod component;
mod created_node;
mod dom_updater;
mod dumb_patch;
mod http;
mod program;
pub mod test_fixtures;
mod util;

pub use browser::Browser;
pub use component::Component;
pub use created_node::CreatedNode;
pub use dom_updater::DomUpdater;
pub use dumb_patch::{
    create_dumb_node,
    dumb_patch,
};
pub use http::Http;
pub use program::Program;
pub use util::{
    body,
    document,
    history,
    now,
    performance,
    request_animation_frame,
    window,
};

impl std::ops::Deref for DomEvent {
    type Target = web_sys::Event;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// This needs wrapping only so that we can implement
/// PartialEq for testing purposes
#[derive(Clone, Debug)]
pub struct DomEvent(pub web_sys::Event);

pub type Cmd<APP, MSG> = sauron_vdom::Cmd<Program<APP, MSG>, MSG>;

pub type Event = DomEvent;

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        let js_value: Option<&JsValue> = self.0.dyn_ref();
        let other_value: Option<&JsValue> = other.0.dyn_ref();
        js_value == other_value
    }
}
