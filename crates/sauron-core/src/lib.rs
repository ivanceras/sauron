#![deny(
    warnings,
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces
)]
#![allow(deprecated)]

//! The core components of sauron

#[macro_use]
pub mod html;
#[macro_use]
pub mod svg;
mod render;
pub mod vdom;

#[doc(hidden)]
pub use mt_dom;

#[macro_use]
extern crate doc_comment;

pub use crate::render::Render;
pub use crate::vdom::{
    diff,
    map_msg::{AttributeMapMsg, ElementMapMsg, NodeMapMsg},
    Attribute, Element, Listener, Node, Patch,
};
pub use crate::{
    html::{
        attributes,
        tags::commons::*,
        units::{self, ch, em, percent, pt, px, rem},
    },
    svg::tags::commons::*,
};

pub use jss;

pub mod dom;
pub use crate::dom::{Component, Container, Effects, Task};

use cfg_if::cfg_if;

cfg_if! {if #[cfg(feature = "with-dom")] {
    pub use web_sys;
    pub use wasm_bindgen_futures;
    pub use js_sys;
    pub use wasm_bindgen;
    pub use wasm_bindgen::prelude::*;
    pub use crate::dom::{Application, events, Program, document, now, window, CustomElement, Cmd};
    pub use serde_wasm_bindgen;
}}
