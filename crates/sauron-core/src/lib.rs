#![deny(
    warnings,
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces
)]
//! The core components of sauron
#[macro_use]
extern crate doc_comment;

#[macro_use]
pub mod html;
#[macro_use]
pub mod svg;
pub mod dom;
mod render;
pub mod vdom;
#[doc(hidden)]
pub use mt_dom;

pub use dom::{Component, Container, Effects, Task};
pub use jss;
pub use render::Render;
pub use vdom::{
    diff,
    map_msg::{AttributeMapMsg, ElementMapMsg, NodeMapMsg},
    Attribute, Element, Listener, Node, Patch,
};

use cfg_if::cfg_if;

cfg_if! {if #[cfg(feature = "with-dom")] {
    pub use web_sys;
    pub use wasm_bindgen_futures;
    pub use js_sys;
    pub use wasm_bindgen;
    pub use wasm_bindgen::prelude::*;
    pub use dom::{Application, events, Program, document, now, window, CustomElement, Cmd};
    pub use serde_wasm_bindgen;
}}
