use sauron::html::attributes::*;
use sauron::html::events::*;
use sauron::html::*;
use sauron::Node;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

use sauron::Component;
use sauron::DomUpdater;
pub use store::Msg;
use store::Store;

mod store;

pub struct App {
    click_count: u32,
}

impl App {
    pub fn new() -> Self {
        App { click_count: 0 }
    }
}

impl Component<Msg> for App {
    fn view(&self) -> Node<Msg> {
        div(
            [class("some-class"), id("some-id"), attr("data-id", 1)],
            [
                input(
                    [
                        class("client"),
                        r#type("button"),
                        value("Click me!"),
                        onclick(move |_| Msg::Click),
                    ],
                    [],
                ),
                text(format!("Clicked: {}", self.click_count)),
            ],
        )
    }

    fn update(&mut self, msg: &Msg) {
        sauron::log!("This is the update in App with msg: {:?}", msg);
        match msg {
            Msg::Click => self.click_count += 1,
        }
        sauron::log!("click_count is now: {}", self.click_count);
    }
}
