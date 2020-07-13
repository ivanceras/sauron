//! Provides struct and functions to create a representation of a virtual node
//!
#![deny(warnings)]
#![deny(clippy::all)]
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces
)]
mod callback;
mod diff;
mod patch;
pub mod util;
mod vnode;

pub use callback::Callback;
pub use diff::{
    diff,
    diff_with_key,
};
pub use patch::Patch;
pub use vnode::{
    attribute,
    builder,
    Element,
    Event,
    Node,
    Text,
    Value,
};

pub use vnode::{
    event,
    Attribute,
    Style,
};
