#![deny(
    warnings,
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces
)]
#![forbid(unsafe_code)]
#![deny(clippy::all)]
//! The core components of sauron
#[macro_use]
extern crate doc_comment;

/// prelude
pub mod prelude {
    pub use crate::html;
    pub use crate::html::{
        attributes::commons::*,
        attributes::key,
        attributes::{
            attr, checked, class, classes, classes_flag, disabled, empty_attr, r#type, styles_flag,
        },
        br, comment,
        commons::*,
        hr, img, input, safe_html, text,
        units::{ch, cm, deg, ex, grad, mm, ms, percent, pt, px, rad, rgb, s, turn, vh, vw},
        view_if,
    };

    pub use crate::render::Render;
    pub use crate::svg;
    pub use crate::svg::attributes::commons::*;
    pub use crate::svg::attributes::special::*;
    pub use crate::svg::commons::*;
    pub use crate::svg::special::*;
    pub use crate::vdom::{
        diff,
        map_msg::{AttributeMapMsg, ElementMapMsg, NodeMapMsg},
        Attribute, AttributeValue, Element, Listener, Node, NodeTrait, Patch,
    };
    pub use mt_dom::TreePath;

    use cfg_if::cfg_if;
    cfg_if! {if #[cfg(feature = "with-dom")] {
        pub use web_sys;
        pub use wasm_bindgen_futures;
        pub use js_sys;
        pub use wasm_bindgen;
        pub use wasm_bindgen::prelude::*;
        pub use serde_wasm_bindgen;
        pub use crate::html::events::*;
        pub use crate::dom::{Application, events, Program, document, now, window, Cmd,
            AnimationFrameHandle, Callback, Component, Container, Effects, Measurements, MountAction,
            MountTarget, Task, TimeoutCallbackHandle,
        };
        #[cfg(feature = "custom_element")]
        pub use crate::dom::WebComponent;
        #[cfg(feature = "custom_element")]
        pub use crate::dom::WebComponentWrapper;
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
