#![deny(warnings)]
#![deny(clippy::all)]
#![feature(type_alias_enum_variants)]
#![feature(arbitrary_self_types)]

//!
//! [![Latest Version](https://img.shields.io/crates/v/sauron.svg)](https://crates.io/crates/sauron)
//! [![Build Status](https://travis-ci.org/ivanceras/sauron.svg?branch=master)](https://travis-ci.org/ivanceras/sauron)
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//!
//! ![sauron](https://raw.githubusercontent.com/ivanceras/sauron/master/assets/sauron.jpg)
//!
//!```log,ignore
//!    One crate to rule the DOM
//!    One crate to find the elements
//!    One crate to bring JSON
//!    And in the Rust code bind Strings
//!    
//!    This code, no other, is made by code elves
//!    Who'd pawn parent process to get it themselves
//!    Ruler of net troll and mortal and hacker
//!    This code is a lib crate for Patreon backers
//!    If trashed or buggy it cannot be remade
//!    If found send to Ivan, the bandwidth is prepaid
//! ```
//!
//!
//!  Sauron is an html web framework for building web-apps with the goal to
//!  closely adhere to [The Elm Architecture](https://guide.elm-lang.org/architecture/), A true
//!  king for elegant design.
//!
//!  As with elm, sauron don't use macro to provide the view, instead just uses plain rust function calls to construct the view.
//!
//! ## Example
//! ```rust,no_run
//! use sauron::html::attributes::*;
//! use sauron::html::events::*;
//! use sauron::html::*;
//! use sauron::Component;
//! use sauron::Node;
//! use sauron::Program;
//! use wasm_bindgen::prelude::*;
//!
//! #[derive(Debug, Clone)]
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
//!         div(
//!             [class("some-class"), id("some-id"), attr("data-id", 1)],
//!             [
//!                 input(
//!                     [
//!                         class("client"),
//!                         r#type("button"),
//!                         value("Click me!"),
//!                         onclick(|_| {
//!                             sauron::log("Button is clicked");
//!                             Msg::Click
//!                         }),
//!                     ],
//!                     [],
//!                 ),
//!                 text(format!("Clicked: {}", self.click_count)),
//!             ],
//!         )
//!     }
//!
//!     fn update(&mut self, msg: Msg) {
//!         sauron::log!("App is updating from msg: {:?}", msg);
//!         match msg {
//!             Msg::Click => self.click_count += 1,
//!         }
//!     }
//!
//! }
//!
//! #[wasm_bindgen(start)]
//! pub fn main() {
//!     Program::new_append_to_mount(App::new(), &sauron::body());
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
//! Build using
//! ```sh
//! $> wasm-pack build --target no-modules
//! ```
//! Look at the [examples](https://github.com/ivanceras/sauron/tree/master/examples) and the build script for the details.
//!
//! Warning: You need to use the latest nightly compiler in order for this to work.
//!
//! ## Prerequisite:
//!
//! ```sh
//! cargo install wasm-pack
//! cargo install basic-http-server
//! ```
//!
//! * TIP: Use `indent_style = "Visual"`in your rustfmt.toml
//! This will visually align the view function in your code, which gives it a more pleasant
//! semantic look
//!
//! This project is based on the existing projects:
//!  - [percy](https://github.com/chinedufn/percy)
//!  - [yew](https://github.com/DenisKolodin/yew)
//!  - [willow](https://github.com/sindreij/willow)
//!
//! ## Performance: Is not too bad.
//! ![Benchmark](https://raw.githubusercontent.com/ivanceras/todomvc-perf-comparison/sauron-benchmark/sauron-benchmark.png)
//!
//! ## Please support this project:
//!  [![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/ivanceras)
//!
//!
//! ## Personal plug:
//! I'm actively looking for a job that has to do with rust.
//! Please contact me: ivanceras[at]gmail.com
pub mod dom;
#[macro_use]
pub mod html;
mod component;
mod dispatch;
pub mod html_extra;
mod program;
#[macro_use]
pub mod svg;
pub mod svg_extra;
pub mod test_fixtures;
mod util;

pub use component::Component;
pub use dispatch::Dispatch;
pub use dom::DomUpdater;
pub use program::Program;
pub use sauron_vdom::{diff,
                      Text};
pub use util::{body,
               document,
               log,
               performance,
               request_animation_frame,
               window};

pub use sauron_vdom::{Event,
                      InputEvent,
                      KeyEvent,
                      MouseEvent};

/// A simplified version of saurdon_vdom node, where we supplied the type for the tag
/// which is a &'static str. The missing type is now only MSG which will be supplied by the users
/// App code.
pub type Node<MSG> = sauron_vdom::Node<&'static str, Event, MSG>;
pub type Element<MSG> = sauron_vdom::Element<&'static str, Event, MSG>;
pub type Patch<'a, MSG> = sauron_vdom::Patch<'a, &'static str, Event, MSG>;
pub type Attribute<MSG> = sauron_vdom::builder::Attribute<Event, MSG>;
