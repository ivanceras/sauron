#![deny(warnings)]
#![deny(clippy::all)]
#![feature(type_alias_enum_variants)]
#![feature(arbitrary_self_types)]

//!
//! [![Latest Version](https://img.shields.io/crates/v/sauron.svg)](https://crates.io/crates/sauron)
//! [![Build Status](https://travis-ci.org/ivanceras/sauron.svg?branch=master)](https://travis-ci.org/ivanceras/sauron)
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//!
//!
//! ![sauron](https://raw.githubusercontent.com/ivanceras/sauron/master/assets/sauron.jpg)
//!
//!  Sauron is an html web framework for building web-apps.
//!  It is heavily inspired by elm.
//!
//!  Sauron doesn't use macro to provide the view, instead it is using rust syntax to construct the
//!  html view.
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
//! Look at the examples code and the build script for the details.
//!
//! Warning: You need to use the latest nightly compiler in order for this to work.
//!
//!
//! This project is based on the existing projects:
//!  - [percy](https://github.com/chinedufn/percy)
//!  - [yew](https://github.com/DenisKolodin/yew)
//!  - [willow](https://github.com/sindreij/willow)
//!
//!
pub mod dom;
#[macro_use]
pub mod html;
mod component;
mod dispatch;
pub mod html_extra;
mod program;
pub mod svg;
pub mod test_fixtures;
mod util;

pub use component::Component;
pub use dispatch::Dispatch;
pub use dom::DomUpdater;
pub use program::Program;
pub use sauron_vdom::diff;
use sauron_vdom::Callback;
pub use sauron_vdom::Event;
pub use sauron_vdom::Text;
pub use util::{body, document, log, performance, request_animation_frame, window};

/// A simplified version of saurdon_vdom node, where we supplied the type for the tag
/// which is a &'static str. The missing type is now only MSG which will be supplied by the users
/// App code.
pub type Node<MSG> = sauron_vdom::Node<&'static str, Callback<Event, MSG>>;
pub type Element<MSG> = sauron_vdom::Element<&'static str, Callback<Event, MSG>>;
pub type Patch<'a, MSG> = sauron_vdom::Patch<'a, &'static str, Callback<Event, MSG>>;
pub type Attribute<'a, MSG> = sauron_vdom::builder::Attribute<'a, Callback<Event, MSG>>;
