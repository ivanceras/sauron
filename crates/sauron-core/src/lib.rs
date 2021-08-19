#![deny(
    warnings,
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces
)]
//!  Core component of sauron

#[macro_use]
extern crate doc_comment;

use cfg_if::cfg_if;
cfg_if! {if #[cfg(feature = "with-dom")] {
    pub mod dom;
    pub use dom::*;
    pub use web_sys;
    pub use wasm_bindgen;
    pub use js_sys;
}}

cfg_if! {if #[cfg(not(feature = "with-dom"))] {
    /// When event is not needed, such as just rendering the dom
    /// tree in server side application
    pub type Event = ();
}}

#[macro_use]
pub mod html;
#[macro_use]
pub mod svg;
mod render;
pub mod vdom;

pub use render::Render;

#[doc(hidden)]
pub use jss;
#[doc(hidden)]
pub use mt_dom;

/// Prelude simplifies the imports from sauron
/// This imports the necessary functions to build
/// a basic sauron app.
pub mod prelude {
    pub use crate::{
        html::{
            attributes::{attr, *},
            tags::{commons::*, *},
            units::*,
            *,
        },
        svg::{attributes::*, tags::commons::*, *},
        vdom::*,
        *,
    };
    pub use render::Render;
    pub use vdom::map_msg::{AttributeMapMsg, ElementMapMsg, NodeMapMsg};
    #[cfg(feature = "with-dom")]
    pub use wasm_bindgen::prelude::*;
    #[cfg(feature = "with-dom")]
    pub use web_sys;
}

pub use mt_dom::Text;
pub use vdom::*;
