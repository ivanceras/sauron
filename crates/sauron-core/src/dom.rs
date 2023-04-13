//! This module provides functionalities for
//! manipulating the actual Document Object Model in the browser
//!
mod application;
mod callback;
pub mod cmd;
mod component;
mod created_node;
mod custom_element;
mod dispatch;
mod dom_patch;
mod effects;
pub mod events;
mod http;
mod program;
mod util;
mod window;

pub use application::{Application, Measurements};
pub use callback::Callback;
pub use component::{Component, Container, CustomElement};
pub use created_node::CreatedNode;
pub use custom_element::register_custom_element;
pub use dispatch::Dispatch;
pub use dom_patch::DomPatch;
pub use effects::Effects;
pub use events::*;
pub use http::Http;
pub use program::{MountAction, MountTarget, Program};
pub use util::{
    async_delay, body, clear_timeout_with_handle, delay_exec, document,
    history, now, performance, request_animation_frame, spawn_local, window,
};
pub use window::Window;

/// alias Cmd to use Program as the APP
pub type Cmd<APP, MSG> = cmd::Cmd<Program<APP, MSG>>;
