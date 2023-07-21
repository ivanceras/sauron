#![deny(warnings)]
//! This is useful only for testing
//! This is a simple component which just barely comply to being a component
//! use for doing component tests
//!
use log::*;
use sauron::{html::div, Application, Cmd, Node, Program};

/// This is a simple component for the puprpose of testing
#[derive(Copy, Clone, Debug)]
pub struct SimpleComponent;

impl Application<()> for SimpleComponent {
    fn init(&mut self) -> Vec<Cmd<Self, ()>> {
        vec![]
    }

    fn update(&mut self, _msg: ()) -> Cmd<Self, ()> {
        trace!("updating in SimpleComponent");
        Cmd::none()
    }

    fn view(&self) -> Node<()> {
        div(vec![], vec![])
    }

    fn stylesheet() -> Vec<String> {
        vec![]
    }
}

/// creates a program from SimpleComponent
pub fn simple_program() -> Program<SimpleComponent, ()> {
    Program::mount_to_body(SimpleComponent)
}
