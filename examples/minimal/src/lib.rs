#![deny(warnings)]
#![deny(clippy::all)]
use log::trace;
use sauron::{
    html::{
        attributes::{attr, class, id, r#type, value},
        div,
        events::on_click,
        h1, input, text,
    },
    jss,
    prelude::*,
    Application, Cmd, Node, Program,
};

pub enum Msg {
    Click,
    NoOp,
}

#[derive(Default)]
pub struct App {
    click_count: u32,
}

#[async_trait(?Send)]
impl Application<Msg> for App {
    fn view(&self) -> Node<Msg> {
        sauron::html::main(
            [],
            [
                h1([], [text("Minimal example")]),
                div(
                    [class("some-class"), id("some-id"), attr("data-id", 1)],
                    [
                        input(
                            [
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
                            [],
                        ),
                        div(
                            [],
                            [text(format!("Clicked: {}", self.click_count))],
                        ),
                        input([r#type("text"), value(self.click_count)], []),
                    ],
                ),
            ],
        )
    }

    async fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Click => self.click_count += 1,
            Msg::NoOp => (),
        }
        Cmd::none()
    }

    fn style(&self) -> String {
        jss! {
            "body": {
                font_family: "Fira Sans, Courier New, Courier, Lucida Sans Typewriter, Lucida Typewriter, monospace",
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::default());
}
