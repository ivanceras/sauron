#![deny(warnings)]
use sauron::{html::attributes::attr, prelude::*, Node};
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate log;

pub enum Msg {
    Click,
}

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
        div!(
            [class("some-class"), id("some-id"), attr("data-id", 1),],
            [
                input!(
                    [
                        class("client"),
                        type_("button"),
                        value("Click me!"),
                        key(1),
                        on_click(|_| {
                            trace!("Button is clicked");
                            Msg::Click
                        }),
                    ],
                    []
                ),
                div!([], [text(format!("Clicked: {}", self.click_count))]),
                input!([type_("text"), value(self.click_count)], [])
            ]
        )
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Click => self.click_count += 1,
        }
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}
