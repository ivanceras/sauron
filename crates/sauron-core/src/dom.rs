//! This module provides functionalities for
//! manipulating the actual Document Object Model in the browser

pub use callback::Callback;
pub use component::{Component, Container, Widget};
pub use effects::Effects;
pub use modifier::Modifier;
pub use task::Task;

mod callback;
mod component;
mod effects;
mod modifier;
mod task;

use cfg_if::cfg_if;

cfg_if! {if #[cfg(feature = "with-dom")] {
    pub use application::{Application, Measurements};
    pub use created_node::CreatedNode;
    pub use custom_element::{register_custom_element, CustomElement, WebComponent};
    pub use dom_patch::{DomPatch, PatchVariant};
    pub use http::Http;
    pub use program::{MountAction, MountTarget, Program};
    pub use util::{
        async_delay, body, delay_exec, document, history, now, performance, request_animation_frame,
        request_idle_callback, spawn_local, window, inject_style,
    };
    pub use window::Window;
    pub use cmd::Cmd;

    mod application;
    pub mod cmd;
    mod created_node;
    mod custom_element;
    mod dom_patch;
    pub mod events;
    mod http;
    mod program;
    pub mod util;
    mod window;


    /// Map the Event to DomEvent, which are browser events
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Event {
        /// native dome events web_sys::Events
        WebEvent(web_sys::Event),
        /// custom event here follows
        MountEvent(crate::events::MountEvent),
    }

}}

/// When event is not needed, such as just rendering the dom
/// tree in server side application
#[cfg(not(feature = "with-dom"))]
pub type Event = ();
