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
//!     dom_updater: DomUpdater<()>,
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

pub type Node<MSG> = sauron_vdom::Node<&'static str, Callback<Event, MSG>>;
pub type Element<MSG> = sauron_vdom::Element<&'static str, Callback<Event, MSG>>;
pub type Patch<'a, MSG> = sauron_vdom::Patch<'a, &'static str, Callback<Event, MSG>>;
pub type Attribute<'a, MSG> = sauron_vdom::builder::Attribute<'a, Callback<Event, MSG>>;
pub use sauron_vdom::diff;
pub use sauron_vdom::Text;

pub trait Component<MSG> {
    fn update(&mut self, msg: &MSG);

    fn view(&self) -> Node<MSG>;
}
