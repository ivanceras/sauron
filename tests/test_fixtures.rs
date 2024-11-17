//! This is useful only for testing
//! This is a simple component which just barely comply to being a component
//! use for doing component tests
//!
use log::*;
use sauron::*;
use std::mem::ManuallyDrop;

/// This is a simple component for the puprpose of testing
#[derive(Copy, Clone, Debug)]
pub struct SimpleComponent;

impl Application for SimpleComponent {
    type MSG = ();

    fn update(&mut self, _msg: ()) -> Cmd<Self::MSG> {
        trace!("updating in SimpleComponent");
        Cmd::none()
    }

    fn view(&self) -> Node<()> {
        div(vec![], vec![])
    }
}

/// creates a program from SimpleComponent
pub fn simple_program() -> ManuallyDrop<Program<SimpleComponent>> {
    console_log::init_with_level(log::Level::Trace).ok();
    Program::mount_to_body(SimpleComponent)
}
