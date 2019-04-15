use sauron::html::attributes::*;
use sauron::html::events::*;
use sauron::html::*;
use sauron::Component;
use sauron::Node;
use sauron::View;
use sauron::Widget;
use std::cell::RefCell;
use std::rc::Rc;

use store::Msg;
use store::Store;

mod store;

pub struct App {
    store: Rc<RefCell<Store>>,
}

impl App {
    pub fn new() -> Self {
        App {
            store: Rc::new(RefCell::new(Store::default())),
        }
    }
}

impl View for App {
    fn view(&self) -> Node {
        let click_count = self.store.borrow().click_count();
        let store = self.store.clone();
        div(
            [class("some-class"), id("some-id"), attr("data-id", 1)],
            [
                input(
                    [
                        class("client"),
                        r#type("button"),
                        value("Click me!"),
                        onclick(move |_| {
                            sauron::log("Button is clicked");
                            store.borrow_mut().msg(&Msg::Click);
                        }),
                    ],
                    [],
                ),
                text(format!("Clicked: {}", click_count)),
            ],
        )
    }
}

impl Widget for App {
    fn update(&mut self) {}
}

impl Component for App {
    fn subscribe(&mut self, callback: Box<Fn()>) {
        self.store.borrow_mut().subscribe(callback);
    }
}
