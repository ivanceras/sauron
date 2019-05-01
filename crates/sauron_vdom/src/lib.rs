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
pub use vnode::{Element,
                Event,
                Node,
                Text,
                Value};

//TODO: expose only event
pub use vnode::event::{Buttons,
                       Coordinate,
                       InputEvent,
                       KeyEvent,
                       Modifier,
                       MouseEvent};

pub use vnode::event;
