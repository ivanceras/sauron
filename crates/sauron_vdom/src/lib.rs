#![deny(warnings)]
#![deny(clippy::all)]
mod callback;
mod diff;
mod patch;
pub mod util;
mod vnode;

pub use callback::Callback;
pub use diff::{diff, diff_with_key};
pub use patch::Patch;
pub use vnode::{builder, Element, Event, Node, Text, Value};

pub use vnode::{event, AttribValue, Attribute};
