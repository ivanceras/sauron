//! This module provides functionalities for
//! manipulating the actual Document Object Model in the browser

pub use cmd::Cmd;
pub use component::Component;
pub use effects::Effects;

mod cmd;
mod component;
mod effects;

use cfg_if::cfg_if;

cfg_if! {if #[cfg(feature = "with-dom")] {
    mod application;
    mod dom_node;
    mod dom_patch;
    mod dom_attr;
    mod http;
    mod program;
    mod raf;
    mod ric;
    mod window;
    mod document;
    mod time;
    mod timeout;

    pub mod events;
    pub mod dispatch;
    pub mod util;

    pub use application::{Application, Measurements, SkipDiff, skip_if, skip_diff, SkipPath};
    pub use component::{stateful_component, StatefulComponent, StatefulModel, StatelessModel};
    pub use component::component;
    pub use dispatch::Dispatch;
    pub use document::Document;
    pub use dom_patch::{DomPatch, PatchVariant, apply_dom_patches, convert_patches};
    pub use dom_attr::{DomAttr, DomAttrValue, GroupedDomAttrValues};
    pub use dom_node::DomNode;
    pub use dom_node::create_dom_node;
    pub use http::Http;
    pub use program::{MountAction, MountTarget, Program, MountProcedure};
    pub use util::{
        document, history, now, performance,
        spawn_local, window, inject_style,
    };
    pub use raf::{request_animation_frame, AnimationFrameHandle};
    pub use ric::{request_idle_callback, IdleCallbackHandle, IdleDeadline};
    pub use timeout::{delay, request_timeout_callback, TimeoutCallbackHandle};
    pub use window::Window;
    pub use time::Time;

    use crate::dom::events::MountEvent;

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
