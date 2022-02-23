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
    html, jss::jss, jss::jss_ns, jss::units, mt_dom, svg, vdom::diff,
    vdom::Attribute, vdom::Element, vdom::Listener, vdom::Node, vdom::Patch,
    Render,
};
#[cfg(feature = "with-node-macro")]
pub use sauron_node_macro::node;
