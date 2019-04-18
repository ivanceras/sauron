use sauron::html::attributes::*;
use sauron::html::events::*;
use sauron::html::*;
use sauron::Node;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

use sauron::Component;
pub use store::Msg;
use store::Store;
use sauron::DomUpdater;

mod store;

pub struct App {
    pub dom_updater: Option<Weak<RefCell<DomUpdater<Self,Msg>>>>,
    click_count: u32,
}

impl App {
    pub fn new() -> Self {
        App {
            dom_updater: None,
            click_count: 0,
        }
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
                        onclick(move |_| {
                            Msg::Click
                        }),
                    ],
                    [],
                ),
                text(format!("Clicked: {}", self.click_count)),
            ],
        )
    }

    fn update(&mut self, msg: &Msg) {
        sauron::log!("This is the update in App with msg: {:?}", msg);
        match msg{
            Msg::Click => self.click_count += 1,
        }
        sauron::log!("click_count is now: {}", self.click_count);
    }


}
