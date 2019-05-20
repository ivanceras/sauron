#![deny(warnings)]
#![deny(clippy::all)]
mod callback;
mod diff;
mod patch;
mod vnode;

pub use vnode::builder;

pub use callback::Callback;
pub use diff::diff;
pub use patch::Patch;
pub use vnode::{
    Attribute,
    Element,
    Event,
    Node,
    Text,
    Value,
};

pub use vnode::event;
