//! This module provides functionalities for
//! manipulating the actual Document Object Model in the browser
//!
mod application;
mod callback;
pub mod cmd;
mod component;
#[cfg(feature = "with-dom")]
mod created_node;
#[cfg(feature = "with-dom")]
mod custom_element;
mod dispatch;
#[cfg(feature = "with-dom")]
mod dom_patch;
mod effects;
#[cfg(feature = "with-dom")]
pub mod events;
#[cfg(feature = "with-dom")]
mod http;
#[cfg(feature = "with-dom")]
mod program;
#[cfg(feature = "with-dom")]
pub mod util;
#[cfg(feature = "with-dom")]
mod window;

pub use application::{Application, Measurements};
pub use callback::Callback;
pub use component::{Component, Container, CustomElement};
#[cfg(feature = "with-dom")]
pub use created_node::CreatedNode;
#[cfg(feature = "with-dom")]
pub use custom_element::register_custom_element;
pub use dispatch::Dispatch;
#[cfg(feature = "with-dom")]
pub use dom_patch::{DomPatch, PatchVariant};
pub use effects::Effects;
#[cfg(feature = "with-dom")]
pub use http::Http;
#[cfg(feature = "with-dom")]
pub use program::{MountAction, MountTarget, Program};
#[cfg(feature = "with-dom")]
pub use util::{
    async_delay, body, delay_exec, document, history, now, performance, request_animation_frame,
    request_idle_callback, spawn_local, window,
};
#[cfg(feature = "with-dom")]
pub use window::Window;

/// alias Cmd to use Program as the APP
#[cfg(feature = "with-dom")]
pub type Cmd<APP, MSG> = cmd::Cmd<Program<APP, MSG>>;

/// Map the Event to DomEvent, which are browser events
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(feature = "with-dom")]
pub enum Event {
    /// native dome events web_sys::Events
    WebEvent(web_sys::Event),
    /// custom event here follows
    MountEvent(crate::events::MountEvent),
}

/// When event is not needed, such as just rendering the dom
/// tree in server side application
#[cfg(not(feature = "with-dom"))]
pub type Event = ();
