#![deny(warnings)]
#![deny(clippy::all)]
#![feature(type_alias_enum_variants)]

//!
//!  Sauron is an html web framework for building web-apps.
//!  It is heavily inspired by elm.
//!
//! # Example
//! ```
//! use sauron::html::attributes::*;
//! use sauron::html::events::*;
//! use sauron::html::*;
//! use sauron::DomUpdater;
//!
//! use wasm_bindgen::prelude::*;
//!
//! #[wasm_bindgen]
//! pub struct Client {
//!     #[allow(unused)]
//!     dom_updater: DomUpdater,
//! }
//!
//! /// Build using
//! /// ```sh
//! /// $ wasm-pack build --target no-modules
//! /// ```
//! ///
//! #[wasm_bindgen]
//! impl Client {
//!
//!     #[wasm_bindgen(constructor)]
//!     pub fn new() -> Client {
//!         let html = div(
//!             [class("some-class"), id("some-id"), attr("data-id", 1)],
//!             [input(
//!                 [
//!                     class("client"),
//!                     r#type("button"),
//!                     value("Click me!"),
//!                     onclick(|_| {
//!                         sauron::log("i've been clicked");
//!                     }),
//!                 ],
//!                 [],
//!             )],
//!         );
//!         sauron::log("hello from here!");
//!         let body = sauron::body();
//!         let dom_updater = DomUpdater::new_append_to_mount(html, &body);
//!         Client { dom_updater }
//!     }
//! }
//!
//! ```
//!
pub mod dom;
#[macro_use]
pub mod html;
pub mod svg;

mod util;

pub use dom::DomUpdater;
use sauron_vdom::Callback;
pub use sauron_vdom::Event;
pub use util::{body, document, log, request_animation_frame, window};

pub type Node = sauron_vdom::Node<&'static str, Callback<Event>>;
pub type Element = sauron_vdom::Element<&'static str, Callback<Event>>;
pub type Patch<'a> = sauron_vdom::Patch<'a, &'static str, Callback<Event>>;
pub type Attribute<'a> = sauron_vdom::builder::Attribute<'a, Callback<Event>>;
pub use sauron_vdom::diff;
pub use sauron_vdom::Text;

pub trait Component: Widget {
    fn subscribe(&mut self, callback: Box<Fn()>);
}

pub trait Widget: View {
    fn update(&mut self);
}

pub trait View {
    fn view(&self) -> Node;
}
