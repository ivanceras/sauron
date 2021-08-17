//!
//!  **Sauron** is an web framework for creating fast and interactive client side web application,
//!  as well as server-side rendering for back-end web applications.
//!
//!
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ivanceras/sauron/master/assets/sauron.png"
)]
#![deny(clippy::all)]
#![deny(
    warnings,
    missing_docs,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces
)]
use cfg_if::cfg_if;

cfg_if! {if #[cfg(feature = "with-dom")] {
    pub use sauron_core::dom;
    pub use sauron_core::dom::*;
    pub use sauron_core::web_sys;
    pub use sauron_core::wasm_bindgen;
    pub use sauron_core::{Component, Cmd, Program};
    pub use sauron_core::js_sys;
}}

/// reexport prelude from sauron core
pub mod prelude {
    pub use sauron_core::prelude::*;
    #[cfg(feature = "with-node-macro")]
    pub use sauron_node_macro::node;
}
pub use sauron_core::{
    diff, html, jss, mt_dom, svg, Attribute, Callback, Element, Node, Patch,
    Render, Text,
};
#[cfg(feature = "sauron-parse")]
pub use sauron_parse::parser;

#[cfg(any(feature = "with-markdown", feature = "with-lite-markdown"))]
pub use sauron_markdown::markdown;
#[cfg(feature = "with-node-macro")]
pub use sauron_node_macro::node;
