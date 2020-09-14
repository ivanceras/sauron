#![deny(warnings)]
use log::trace;
use sauron::{
    html::{
        attributes::{
            attr,
            class,
            id,
            key,
            type_,
            value,
        },
        div,
        events::on_click,
        h1,
        input,
        text,
    },
    prelude::*,
    Cmd,
    Component,
    Node,
    Program,
};

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
        sauron::html::main(
            vec![],
            vec![
                h1(vec![], vec![text("Minimal example")]),
                div(
                    vec![
                        class("some-class"),
                        id("some-id"),
                        attr("data-id", 1),
                    ],
                    vec![
                        input(
                            vec![
                                class("client"),
                                type_("button"),
                                value("Click me!"),
                                key(1),
                                on_click(|_| {
                                    trace!("Button is clicked");
                                    Msg::Click
                                }),
                            ],
                            vec![],
                        ),
                        div(
                            vec![],
                            vec![text(format!(
                                "Clicked: {}",
                                self.click_count
                            ))],
                        ),
                        input(
                            vec![type_("text"), value(self.click_count)],
                            vec![],
                        ),
                    ],
                ),
            ],
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
