#![deny(warnings)]
#![deny(clippy::all)]
#![feature(type_alias_enum_variants)]
#![feature(arbitrary_self_types)]

//!
//!  Sauron is an html web framework for building web-apps.
//!  It is heavily inspired by elm.
//!
//!
pub mod dom;
#[macro_use]
pub mod html;
mod program;
pub mod svg;

mod util;

pub use dom::DomUpdater;
pub use program::Program;
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

pub mod test_fixtures;
