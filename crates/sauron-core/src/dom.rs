//! This module provides functionalities for
//! manipulating the actual Document Object Model in the browser
//!
pub mod apply_patches;
pub mod cmd;
mod component;
mod created_node;
mod dispatch;
mod dom_updater;
pub mod events;
mod http;
mod program;
mod util;
mod window;

pub use component::{Application, Component, Measurements, Widget};
pub use created_node::CreatedNode;
pub use dispatch::Dispatch;
pub use dom_updater::DomUpdater;
pub use events::*;
pub use http::Http;
pub use program::Program;
pub use util::{
    body, document, execute_in_request_animation_frame, history, now,
    performance, request_animation_frame, window,
};
pub use window::Window;

/// alias Cmd to use Program as the APP
pub type Cmd<APP, MSG> = cmd::Cmd<Program<APP, MSG>>;
