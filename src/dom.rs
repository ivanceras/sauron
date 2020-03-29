pub mod apply_patches;
mod browser;
mod component;
mod created_node;
mod dom_updater;
mod http;
mod program;
pub mod test_fixtures;
mod util;
mod window;
mod dispatch;
pub mod cmd;

pub use dispatch::Dispatch;
pub use browser::Browser;
pub use component::Component;
pub use created_node::CreatedNode;
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

pub type Cmd<APP, MSG> = cmd::Cmd<Program<APP, MSG>, MSG>;
