#![deny(warnings)]
use sauron::html::attributes::*;
use sauron::html::events::*;
use sauron::html::*;
use sauron::Component;
use sauron::DomUpdater;
use sauron::Node;
use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Client {
    #[allow(unused)]
    dom_updater: DomUpdater<App, ()>,
}

/// Build using
/// ```sh
/// $ wasm-pack build --target no-modules
/// ```
///
#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Client {
        let html = div(
            [class("some-class"), id("some-id"), attr("data-id", 1)],
            [input(
                [
                    class("client"),
                    r#type("button"),
                    value("Click me!"),
                    onclick(|_| {
                        sauron::log("i've been clicked");
                    }),
                ],
                [],
            )],
        );
        sauron::log("hello from here!");
        let body = sauron::body();
        let app = Rc::new(RefCell::new(App {}));
        let dom_updater = DomUpdater::new_append_to_mount(app, html, &body);
        Client { dom_updater }
    }
}

pub struct App {}

impl Component<()> for App {
    fn update(&mut self, _: &()) {
        sauron::log("Nothing to update...");
    }

    fn view(&self) -> Node<()> {
        div(
            [class("some-class"), id("some-id"), attr("data-id", 1)],
            [input(
                [
                    class("client"),
                    r#type("button"),
                    value("Click me!"),
                    onclick(|_| {
                        sauron::log("i've been clicked");
                    }),
                ],
                [],
            )],
        )
    }
}
