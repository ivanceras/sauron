//! This is useful only for testing
//! This is a simple component which just barely comply to being a component
//! use for doing component tests
//!
use crate::{
    html::div,
    Cmd,
    Component,
    Program,
};
use log::*;
use std::{
    cell::RefCell,
    rc::Rc,
};

/// This is a simple component for the puprpose of testing
#[derive(Copy, Clone, Debug)]
pub struct SimpleComponent;

impl Component<()> for SimpleComponent {
    fn update(&mut self, _msg: ()) -> Cmd<Self, ()> {
        trace!("updating in SimpleComponent");
        Cmd::none()
    }

    fn view(&self) -> crate::Node<()> {
        div(vec![], vec![])
    }
}

/// creates the SimpleComponent wraped in Rc and RefCell
pub fn simple_component() -> Rc<RefCell<SimpleComponent>> {
    Rc::new(RefCell::new(SimpleComponent))
}

/// creates a program from SimpleComponent
pub fn simple_program() -> Program<SimpleComponent, ()> {
    Program::new_append_to_mount(SimpleComponent, &crate::body())
}
