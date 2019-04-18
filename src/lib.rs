#![deny(warnings)]
#![deny(clippy::all)]
#![feature(type_alias_enum_variants)]

//!
//!  Sauron is an html web framework for building web-apps.
//!  It is heavily inspired by elm.
//!
//!
pub mod dom;
#[macro_use]
pub mod html;
pub mod svg;
mod program;

mod util;

pub use dom::DomUpdater;
use sauron_vdom::Callback;
pub use sauron_vdom::Event;
pub use program::Program;

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

pub mod test_fixtures {
    use crate::html::div;
    use crate::Component;
    use crate::Node;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Clone)]
    pub struct SimpleComponent;

    impl Component<()> for SimpleComponent {
        fn update(&mut self, _msg: &()) {
            crate::log("updating in SimpleComponent");
        }
        fn view(&self) -> Node<()> {
            div([], [])
        }
    }

    pub fn simple_component() -> Rc<RefCell<SimpleComponent>> {
        Rc::new(RefCell::new(SimpleComponent))
    }

}
