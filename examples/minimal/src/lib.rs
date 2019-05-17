#![deny(warnings)]
use sauron::{html::{attributes::*,
                    events::*,
                    *},
             Cmd,
             Component,
             Node,
             Program};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
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
        div([class("some-class"), id("some-id"), attr("data-id", 1)],
            [input([class("client"),
                    r#type("button"),
                    value("Click me!"),
                    onclick(|_| {
                        sauron::log("Button is clicked");
                        Msg::Click
                    })],
                   []),
             text(format!("Clicked: {}", self.click_count))])
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        sauron::log!("App is updating from msg: {:?}", msg);
        match msg {
            Msg::Click => self.click_count += 1,
        }
        Cmd::none()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    Program::new_append_to_mount(App::new(), &sauron::body());
}
