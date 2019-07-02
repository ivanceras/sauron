#![deny(warnings)]
#![deny(clippy::all)]
#![feature(arbitrary_self_types)]
mod callback;
mod cmd;
mod diff;
mod dispatch;
mod patch;
pub(in crate) mod util;
mod vnode;

pub use callback::Callback;
pub use cmd::Cmd;
pub use diff::diff;
pub use dispatch::Dispatch;
pub use patch::Patch;
pub use vnode::{
    builder,
    Element,
    Event,
    Node,
    Text,
    Value,
};

pub use vnode::{
    event,
    AttribValue,
    Attribute,
};
