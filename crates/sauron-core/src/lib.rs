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

pub use jss;

/// prelude
pub mod prelude {
    pub use crate::html;
    pub use crate::html::{
        attributes::commons::*,
        attributes::key,
        attributes::{attr, class_namespaced, classes_flag, classes_flag_namespaced},
        commons::*,
        events::*,
        text,
        units::{ch, cm, deg, ex, grad, mm, ms, percent, pt, px, rad, rgb, s, turn, vh, vw},
        view_if,
    };
    pub use crate::render::Render;
    pub use crate::style;
    pub use crate::svg;
    pub use crate::svg::attributes::commons::*;
    pub use crate::svg::attributes::special::*;
    pub use crate::svg::commons::*;
    pub use crate::svg::special::*;
    pub use crate::vdom::{
        diff,
        map_msg::{AttributeMapMsg, ElementMapMsg, NodeMapMsg},
        Attribute, Element, Listener, Node, Patch,
    };
    pub use jss as jss_crate;
    pub use jss::{jss, jss_ns, jss_ns_pretty, jss_pretty};
    pub use wasm_bindgen::prelude::*;

    use cfg_if::cfg_if;
    cfg_if! {if #[cfg(feature = "with-dom")] {
        pub use web_sys;
        pub use wasm_bindgen_futures;
        pub use js_sys;
        pub use wasm_bindgen;
        pub use wasm_bindgen::prelude::*;
        pub use serde_wasm_bindgen;
        pub use crate::dom::{Application, events, Program, document, now, window, CustomElement, Cmd,
            AnimationFrameHandle, Callback, Component, Container, Effects, Measurements, MountAction,
            MountTarget, Task, TimeoutCallbackHandle,
        };
    }}
}

#[macro_use]
pub mod html;
#[macro_use]
pub mod svg;
pub mod dom;
mod render;
pub mod vdom;
#[doc(hidden)]
pub use mt_dom;
