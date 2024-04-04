//! This module provides functionalities for
//! manipulating the actual Document Object Model in the browser

pub use component::Component;
pub use effects::Effects;
pub use cmd::Cmd;

mod component;
mod effects;
mod cmd;

use cfg_if::cfg_if;

cfg_if! {if #[cfg(feature = "with-dom")] {
    pub use application::{Application, Measurements, SkipDiff, skip_if, skip_diff, SkipPath};
    #[cfg(feature = "custom_element")]
    pub use component::{register_web_component, WebComponent, WebComponentWrapper};
    pub use component::{stateful_component, StatefulComponent, StatefulModel, StatelessModel};
    pub use component::component;
    pub use dom_patch::{DomPatch, PatchVariant};
    pub use dom_attr::{DomAttr, DomAttrValue, GroupedDomAttrValues};
    pub use http::Http;
    pub use program::{MountAction, MountTarget, Program, MountProcedure};
    pub use util::{
        document, history, now, performance,
        spawn_local, window, inject_style,
    };
    pub use raf::{request_animation_frame, AnimationFrameHandle};
    pub use ric::{request_idle_callback, IdleCallbackHandle, IdleDeadline};
    pub use timeout::{delay, request_timeout_callback, TimeoutCallbackHandle};
    pub use dispatch::Dispatch;
    use crate::dom::events::MountEvent;
    pub use window::Window;
    pub use dom_node::DomNode;
    pub use document::Document;
    pub use time::Time;

    mod application;
    pub mod dispatch;
    mod dom_node;
    mod dom_patch;
    mod dom_attr;
    pub mod events;
    mod http;
    mod program;
    pub mod util;
    mod raf;
    mod ric;
    mod window;
    mod document;
    mod time;
    mod timeout;


    /// Map the Event to DomEvent, which are browser events
    #[derive(Debug, Clone)]
    pub enum Event {
        /// native dome events web_sys::Events
        WebEvent(web_sys::Event),
        /// custom event here follows
        MountEvent(MountEvent),
    }

}}

/// When event is not needed, such as just rendering the dom
/// tree in server side application
#[cfg(not(feature = "with-dom"))]
pub type Event = ();
