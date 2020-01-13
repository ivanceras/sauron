use sauron_vdom;

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
mod window;

pub use browser::Browser;
pub use component::Component;
pub use created_node::CreatedNode;
pub use dom_updater::DomUpdater;
pub use dumb_patch::{
    apply_dumb_patch,
    create_dumb_node,
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
pub use window::Window;

pub type Cmd<APP, MSG> = sauron_vdom::Cmd<Program<APP, MSG>, MSG>;
