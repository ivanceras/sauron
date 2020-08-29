//! This is useful only for testing
//! This is a simple component which just barely comply to being a component
//! use for doing component tests
//!
use log::*;
use sauron_core::{body, html::div, Cmd, Component, Node, Program};

/// This is a simple component for the puprpose of testing
#[derive(Copy, Clone, Debug)]
pub struct SimpleComponent;

impl Component<()> for SimpleComponent {
    fn update(&mut self, _msg: ()) -> Cmd<Self, ()> {
        trace!("updating in SimpleComponent");
        Cmd::none()
    }

    fn view(&self) -> Node<()> {
        div(vec![], vec![])
    }
}

/// creates a program from SimpleComponent
pub fn simple_program() -> Program<SimpleComponent, ()> {
    Program::new_append_to_mount(SimpleComponent, &body())
}
