//! This module provides functionalities for
//! manipulating the actual Document Object Model in the browser
//!
pub mod apply_patches;
mod browser;
pub mod cmd;
mod component;
mod created_node;
mod dispatch;
mod dom_updater;
mod http;
mod program;
pub mod test_fixtures;
mod util;
mod window;

pub use browser::Browser;
pub use component::Component;
pub use created_node::CreatedNode;
pub use dispatch::Dispatch;
pub use dom_updater::DomUpdater;
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
pub use window::Window;

/// alias Cmd to use Program as the APP
pub type Cmd<APP, MSG> = cmd::Cmd<Program<APP, MSG>, MSG>;
