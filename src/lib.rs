//#![deny(warnings)]
//#![deny(clippy::all)]
//#![doc(
//    html_logo_url = "https://raw.githubusercontent.com/ivanceras/sauron/master/assets/sauron.png"
//)]
//#![deny(
//    missing_docs,
//    //missing_debug_implementations,
//    missing_copy_implementations,
//    trivial_casts,
//    trivial_numeric_casts,
//    unstable_features,
//    unused_import_braces
//)]
//!
//! [![Latest Version](https://img.shields.io/crates/v/sauron.svg)](https://crates.io/crates/sauron)
//! [![Build Status](https://travis-ci.org/ivanceras/sauron.svg?branch=master)](https://travis-ci.org/ivanceras/sauron)
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//!
//! ![sauron](https://raw.githubusercontent.com/ivanceras/sauron/master/assets/sauron.png)
//!
//!
//! > One crate to rule the DOM
//! >
//! > One crate to mind it
//! >
//! > One crate to bring JSON
//! >
//! > And in the Rust code bind it
//! >
//! >
//! >
//! > This code, no other, is made by code elves
//! >
//! > Who'd pawn parent process to get it themselves
//! >
//! > Ruler of net troll and mortal and hacker
//! >
//! > This code is a lib crate for Patreon backers
//! >
//! > If trashed or buggy it cannot be remade
//! >
//! > If found send to Ivan, the bandwidth is prepaid
//!
//! -- The Harvard Lampoon & [po8](https://www.reddit.com/user/po8/)
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
//! use wasm_bindgen::prelude::*;
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
//! **Note:** You need to use the nightly compiler with minimum version: rustc 1.37.0-nightly (17e62f77f 2019-07-01)
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
//! **Warning:** I repeat, You need to use the latest nightly compiler in order for this to work.
//!
//! ### Performance:
//! ![Benchmark](https://raw.githubusercontent.com/ivanceras/sauron/master/assets/perf.png)
//!
//! ### Please support this project:
//!  [![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/ivanceras)
//!
//!
//!
#[macro_use]
extern crate doc_comment;

use cfg_if::cfg_if;
cfg_if! {if #[cfg(feature = "with-dom")] {
    pub mod dom;
    pub use dom::*;
    /// Map the Event to DomEvent, which are browser events
    pub type Event = web_sys::Event;
}}

#[cfg(not(feature = "with-dom"))]
pub type Event = ();

#[macro_use]
pub mod html;

#[macro_use]
pub mod svg;

pub use render::Render;
pub mod render;

use mt_dom::diff_with_key;
use prelude::AttributeValue;

pub use mt_dom;

/// Prelude simplifies the imports from sauron
/// This imports the necessary functions to build
/// a basic sauron app.
pub mod prelude {
    pub use crate::{
        html::{
            attributes::{
                attr,
                *,
            },
            tags::{
                commons::*,
                *,
            },
            units::*,
            *,
        },
        svg::{
            attributes::*,
            tags::commons::*,
            *,
        },
        *,
    };
}

/// namespace type in node, which could be change to an enum
pub type Namespace = &'static str;
/// tags are using static str for now, can also be enum tags
pub type Tag = &'static str;
/// attribute keys
pub type AttributeKey = &'static str;

/// A simplified version of saurdon_vdom node, where we supplied the type for the tag
/// which is a &'static str. The missing type is now only MSG which will be supplied by the users
/// App code.
pub type Node<MSG> =
    mt_dom::Node<Namespace, Tag, AttributeKey, AttributeValue, Event, MSG>;

/// Element type with tag and attribute name type set to &'static str
pub type Element<MSG> =
    mt_dom::Element<Namespace, Tag, AttributeKey, AttributeValue, Event, MSG>;

/// Patch as result of diffing the current_vdom and the new vdom.
/// The tag and attribute name types is set to &'static str
pub type Patch<'a, MSG> =
    mt_dom::Patch<'a, Namespace, Tag, AttributeKey, AttributeValue, Event, MSG>;

/// Attribute type used in sauron where the type of the Attribute name is &'static str
pub type Attribute<MSG> =
    mt_dom::Attribute<Namespace, AttributeKey, AttributeValue, Event, MSG>;

/// Callback where Event type is supplied
pub type Callback<MSG> = mt_dom::Callback<Event, MSG>;

/// This is a sauron html specific functionality
/// diff 2 nodes with attribute using `&'static str` instead of generic ATT
pub fn diff<'a, MSG>(
    old: &'a Node<MSG>,
    new: &'a Node<MSG>,
) -> Vec<Patch<'a, MSG>>
where
    MSG: 'static,
{
    diff_with_key(old, new, &"key")
}
