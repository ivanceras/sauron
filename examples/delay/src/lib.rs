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
    Application, Cmd, Node, Program,
};
use wasm_bindgen_futures::spawn_local;

pub enum Msg {
    Click,
    NoOp,
}

pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }
}

impl Application<Msg> for App {
    fn view(&self) -> Node<Msg> {
        sauron::html::main(
            [],
            [
                h1([], [text("Delay example")]),
                div(
                    [],
                    [input(
                        [
                            class("client"),
                            r#type("button"),
                            value("Click me!"),
                            on_click(|_| {
                                trace!("Button is clicked");
                                Msg::Click
                            }),
                        ],
                        [],
                    )],
                ),
            ],
        )
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Click => spawn_local(some_async_function()),
            Msg::NoOp => (),
        }
        Cmd::none()
    }
}

async fn some_async_function(){
        let t1 = sauron::now();
        log::debug!("t1: {}", t1);
        async_delay(1000).await;
        let t2 = sauron::now();
        log::debug!("t2: {}", t2);
        log::debug!("elapsed: {}", t2 - t1);
        async_delay(5000).await;
        let t3 = sauron::now();
        log::debug!("t3: {}", t3);
        log::debug!("elapsed: {}", t3 - t2);
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}
