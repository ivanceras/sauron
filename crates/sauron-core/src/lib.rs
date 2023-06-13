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
pub mod html;
#[macro_use]
pub mod svg;
mod render;
pub mod vdom;

#[doc(hidden)]
pub use mt_dom;

#[macro_use]
extern crate doc_comment;

pub use jss::{jss, jss_ns, jss_ns_pretty, jss_pretty};

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

pub mod dom;
pub use crate::dom::{
    body, document, events, now, window, Application, Cmd, Component, Container, CustomElement,
    Dispatch, Effects, Program,
};

use cfg_if::cfg_if;

cfg_if! {if #[cfg(feature = "with-dom")] {
    pub use web_sys;
    pub use wasm_bindgen_futures;
    pub use js_sys;
    pub use wasm_bindgen;
    pub use wasm_bindgen::prelude::*;
}}

cfg_if! {if #[cfg(not(feature = "with-dom"))] {
    /// When event is not needed, such as just rendering the dom
    /// tree in server side application
    pub type Event = ();
}}
