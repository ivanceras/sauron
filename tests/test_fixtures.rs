#![deny(warnings)]
//! This is useful only for testing
//! This is a simple component which just barely comply to being a component
//! use for doing component tests
//!
use log::*;
use sauron::{
    html::div,
    Application,
    Cmd,
    Node,
    Program,
};
use async_trait::async_trait;

/// This is a simple component for the puprpose of testing
#[derive(Copy, Clone, Debug)]
pub struct SimpleComponent;

#[async_trait(?Send)]
impl Application<()> for SimpleComponent {
    async fn update(&mut self, _msg: ()) -> Cmd<Self, ()> {
        trace!("updating in SimpleComponent");
        Cmd::none()
    }

    fn view(&self) -> Node<()> {
        div(vec![], vec![])
    }

    fn style(&self) -> String {
        String::new()
    }
}

/// creates a program from SimpleComponent
pub fn simple_program() -> Program<SimpleComponent, ()> {
    Program::mount_to_body(SimpleComponent)
}
