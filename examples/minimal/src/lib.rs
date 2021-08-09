//#![deny(warnings)]
use log::trace;
use sauron::{
    html::{
        attributes::{attr, class, id, r#type, value},
        div,
        events::on_click,
        h1, input, text,
    },
    prelude::*,
    Cmd, Component, Node, Program,
};

pub enum Msg {
    Click,
    NoOp,
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
                                r#type("button"),
                                value("Click me!"),
                                on_click(|_| {
                                    trace!("Button is clicked");
                                    Msg::Click
                                }),
                                on_mount(|m| {
                                    log::trace!(
                                        "input button is mounted into: {:?}",
                                        m.target_node
                                    );
                                    Msg::NoOp
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
                            vec![r#type("text"), value(self.click_count)],
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
            Msg::NoOp => (),
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
