//#![deny(warnings)]
#![deny(clippy::all)]
#![feature(type_alias_enum_variants)]

pub mod dom;
#[macro_use]
pub mod html;
pub mod svg;

mod util;

pub use dom::DomUpdater;
pub use sauron_vdom::builder::Attribute;
pub use sauron_vdom::Event;
pub use util::{body, document, log, window};

pub type Node = sauron_vdom::Node<&'static str>;
pub type Element = sauron_vdom::Element<&'static str>;
pub type Patch<'a> = sauron_vdom::Patch<'a, &'static str>;
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
