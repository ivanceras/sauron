//!
//! [![Latest Version](https://img.shields.io/crates/v/sauron.svg)](https://crates.io/crates/sauron)
//! [![Build Status](https://travis-ci.org/ivanceras/sauron.svg?branch=master)](https://travis-ci.org/ivanceras/sauron)
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//!
//! ![sauron](https://raw.githubusercontent.com/ivanceras/sauron/master/assets/sauron.png)
//!
//!
//!  **Sauron** is an HTML web framework for building web-apps with the goal of
//!  closely adhering to [The Elm Architecture](https://guide.elm-lang.org/architecture/), a paragon of elegant design.
//!
//!  Sauron follow Elm's simplistic design of writing view code.
//!
//! ### Example
//! ```rust,no_run
//! use sauron::prelude::*;
//! use sauron::wasm_bindgen::prelude::*;
//! use log::*;
//!
//! #[derive(Debug, PartialEq, Clone)]
//! pub enum Msg {
//!     Click,
//! }
//!
//! pub struct App {
//!     click_count: u32,
//! }
//!
//! impl App {
//!     pub fn new() -> Self {
//!         App { click_count: 0 }
//!     }
//! }
//!
//! impl Component<Msg> for App {
//!
//!     fn view(&self) -> Node<Msg> {
//!         div!(
//!             [class("some-class"), id("some-id"), attr("data-id", 1)],
//!             [
//!                 input!(
//!                     [
//!                         class("client"),
//!                         type_("button"),
//!                         value("Click me!"),
//!                         on_click(|_| {
//!                             trace!("Button is clicked");
//!                             Msg::Click
//!                         }),
//!                     ],
//!                     [],
//!                 ),
//!                 text!("Clicked: {}", self.click_count),
//!             ],
//!         )
//!     }
//!
//!     fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
//!         trace!("App is updating from msg: {:?}", msg);
//!         match msg {
//!             Msg::Click => {
//!                 self.click_count += 1;
//!                 Cmd::none()
//!             }
//!         }
//!     }
//!
//! }
//!
//! #[wasm_bindgen(start)]
//! pub fn main() {
//!     Program::mount_to_body(App::new());
//! }
//! ```
//! index.html
//! ```html
//! <html>
//!   <head>
//!     <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
//!     <title>Minimal sauron app</title>
//!   </head>
//!   <body>
//!     <script src='pkg/minimal.js'></script>
//!     <script type=module>
//!         window.wasm_bindgen('pkg/minimal_bg.wasm')
//!             .catch(console.error);
//!     </script>
//!   </body>
//! </html>
//! ```
//! In Cargo.toml, specify the crate-type to be `cdylib`
//! ```toml
//! [lib]
//! crate-type = ["cdylib"]
//! ```
//!
//!
//! Build using
//! ```sh
//! $> wasm-pack build --target no-modules
//! ```
//! Look at the [examples](https://github.com/ivanceras/sauron/tree/master/examples)
//! and the build script for the details.
//!
//!
//! ### Demo examples
//! - [todomvc](https://ivanceras.github.io/todomvc/)
//! - [data-viewer](https://ivanceras.github.io/data-viewer/) - A resizable spreadsheet CSV data viewer
//! - [svg_clock](https://ivanceras.github.io/svg_clock/) - A clock drawn using SVG and window tick event.
//! - [svg_graph](https://ivanceras.github.io/svg_graph/) - A simple graph using SVG
//! - [tinki wiki](https://github.com/ivanceras/tinki) - My personal wiki, powering my [github
//! pages](https://ivanceras.github.io)
//!
//! ### Converting HTML into Sauron's syntax
//!
//! [html2sauron](https://ivanceras.github.io/html2sauron/) - A tool to easily convert html into
//! sauron node tree for your views.
//!
//! Note: When writing the view in sauron, just keep in mind that the function name is the element tag
//! you are creating and there is 2 arguments for it. The first argument is an array of the attributes of the element
//! and the second argument is an array of the children of the element.
//!
//! Example:
//! ```rust,ignore
//! div!([id("container"),class("hero")], [text("Content goes here")])
//! ```
//! `div` macro call is the element tag.
//! The 1st argument in the call is an array of attributes for the div element expressed in a
//! function call `id` and `class` which are valid attributes for a div.
//! The 2nd argument in the call is an array of the children elements, and you can nest as many as
//! you like.
//!
//! ### Prerequisite:
//!
//! ```sh
//! cargo install wasm-pack
//! cargo install basic-http-server
//! ```
//!
//! ### Performance:
//! Sauron is one of the fastest.
//!
//! ![Benchmark](https://raw.githubusercontent.com/ivanceras/sauron/master/assets/alt-sauron-0.28.png)
//! ![Benchmark](https://raw.githubusercontent.com/ivanceras/sauron/master/assets/sauron-0.27.png)
//!
//! ### Run the benchmark yourself:
//! [Benchmark 1](https://ivanceras.github.io/todo-mvc-bench/)
//! [Benchmark 2](https://ivanceras.github.io/todomvc-benchmark/)
//!
//! ### Please support this project:
//!  [![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/ivanceras)
//!
//!
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

pub use sauron_core::dom;
pub use sauron_core::dom::*;

/// reexport prelude from sauron core
pub mod prelude {
    pub use sauron_core::prelude::*;
    pub use sauron_node_macro::node;
}
pub use sauron_core::{
    diff, html, Attribute, Callback, Element, Node, Patch, Render,
};

pub use sauron_node_macro::node;
// reexport web_sys crate
pub use sauron_core::web_sys;
// reexport wasm_bindgen crate
pub use sauron_core::wasm_bindgen;
