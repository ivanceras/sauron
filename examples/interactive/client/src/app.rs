use js_sys::Date;
use sauron::{html::{attributes::*,
                    events::*,
                    *},
             Event,
             Node,
             *};
use wasm_bindgen::{self,
                   prelude::*};

#[derive(Debug, Clone)]
pub enum Msg {
    Click,
    Clock,
    ChangeName(String),
    ChangeBiography(String),
    ChangeThought(String),
}

pub struct App {
    click_count: u32,
    date: Date,
    name: String,
    biography: String,
    thought: Option<String>,
}

impl App {
    pub fn new(click_count: u32) -> App {
        App { click_count,
              date: Date::new_0(),
              name: String::new(),
              biography: String::new(),
              thought: None }
    }
}

impl Component<Msg> for App {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Click => {
                self.click_count += 1;
            }
            Msg::Clock => {
                self.date = Date::new_0();
            }
            Msg::ChangeName(name) => {
                self.name = name;
            }
            Msg::ChangeBiography(bio) => {
                self.biography = bio;
            }
            Msg::ChangeThought(thought) => {
                if thought.len() > 0 {
                    self.thought = Some(thought);
                } else {
                    self.thought = None;
                }
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        let date_str: String =
            self.date
                .to_locale_string("en-GB", &JsValue::undefined())
                .into();
        div(
            [class("some-class"), id("some-id"), attr("data-id", 1)],
            [
            div(
                [id("current-time")],
                [text(format!("Today is {}", date_str))],
            ),
            div(
                [],
                [
                    text("Your name is: "),
                    input(
                        [
                            r#type("text"),
                            oninput(|v: Event| {
                                sauron::log(format!("input has input: {:#?}",
                                                    v));
                                if let Event::InputEvent(input) = v {
                                    Msg::ChangeName(input.value)
                                } else {
                                    panic!("This shouldn't happened");
                                }
                            }),
                            placeholder("John Smith"),
                        ],
                        [],
                    ),
                    button(
                        [onclick(|v: Event| {
                            sauron::log(format!(
                                "I've been clicked and the value is: {:#?}",
                                v
                            ));
                            Msg::Click
                        })],
                        [text("Click me!")],
                    ),
                ],
            ),
            p(
                [],
                [
                    text(format!("Hello {}!", self.name,)),
                    if self.click_count > 0 {
                        text(format!(
                                ", You've clicked on that button for {} time{}",
                                self.click_count,
                                if self.click_count > 1 { "s" } else { "" }
                            ))
                    } else {
                        span([], [])
                    },
                ],
            ),
            div(
                [],
                [
                    p([], [text("Tell us something about yourself:")]),
                    div(
                        [],
                        [textarea(
                            [
                                rows(10),
                                cols(80),
                                oninput(|v: Event| {
                                    if let Event::InputEvent(input) = v {
                                        Msg::ChangeBiography(input.value)
                                    } else {
                                        panic!("This shouldn't happened");
                                    }
                                }),
                                placeholder("I'm a..."),
                            ],
                            [],
                        )],
                    ),
                    p([], [text(&self.biography)]),
                ],
            ),
            div(
                [],
                [
                    text("What are you thinking right now?"),
                    input(
                        [
                            r#type("text"),
                            onchange(|v: Event| {
                                if let Event::InputEvent(input) = v {
                                    Msg::ChangeThought(input.value)
                                } else {
                                    panic!("This shouldn't happened");
                                }
                            }),
                            placeholder("Elephants..."),
                        ],
                        [],
                    ),
                    if let Some(thought) = &self.thought {
                        text(format!("Hmmn {}... Interesting.", thought))
                    } else {
                        span([], [])
                    },
                ],
            ),
        ],
        )
    }
}
