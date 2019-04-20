use js_sys::Date;
use sauron::html::attributes::*;
use sauron::html::events::*;
use sauron::html::*;
use sauron::Event;
use sauron::Node;
use sauron::*;
use wasm_bindgen;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub enum Msg {
    Click,
    Clock,
}

pub struct App {
    click_count: u32,
    date: Date,
}

impl App {
    pub fn new(click_count: u32) -> App {
        App {
            click_count,
            date: Date::new_0(),
        }
    }
}

impl Component<Msg> for App {
    fn subscribe(&self) {}

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Click => {
                self.click_count += 1;
            }
            Msg::Clock => {
                self.date = Date::new_0();
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        let date_str = self.date.to_locale_string("en-GB", &JsValue::undefined());
        div(
            [class("some-class"), id("some-id"), attr("data-id", 1)],
            [
                div([], [text(format!("Hello world! {}", self.click_count))]),
                div([id("current-time")], [text(date_str)]),
                div(
                    [],
                    [button(
                        [onclick(move |v: Event| {
                            sauron::log(format!("I've been clicked and the value is: {:#?}", v));
                            Msg::Click
                        })],
                        [text("Click me!")],
                    )],
                ),
                div(
                    [],
                    [
                        text("Using oninput"),
                        input(
                            [
                                r#type("text"),
                                oninput(|v: Event| {
                                    sauron::log(format!("input has input: {:#?}", v));
                                    Msg::Click
                                }),
                                placeholder("Type here..."),
                            ],
                            [],
                        ),
                    ],
                ),
                div(
                    [],
                    [
                        text("using oninput on a textarea"),
                        textarea(
                            [
                                oninput(|v: Event| {
                                    sauron::log(format!("textarea has changed: {:#?}", v));
                                    Msg::Click
                                }),
                                placeholder("Description here..."),
                            ],
                            [],
                        ),
                    ],
                ),
                div(
                    [],
                    [
                        text("Using onchange"),
                        input(
                            [
                                r#type("text"),
                                onchange(|v: Event| {
                                    sauron::log(format!("input has changed: {:#?}", v));
                                    Msg::Click
                                }),
                                placeholder("Description here..."),
                            ],
                            [],
                        ),
                    ],
                ),
            ],
        )
    }
}
