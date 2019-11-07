use js_sys::Date;
use sauron::{
    html::{
        attributes::*,
        events::*,
        *,
    },
    Cmd,
    Node,
    *,
};
use wasm_bindgen::{
    self,
    prelude::*,
};

pub enum Msg {
    Click,
    DoubleClick,
    Clock,
    ChangeName(String),
    ChangeBiography(String),
    ChangeThought(String),
}

pub struct App {
    click_count: u32,
    double_clicks: u32,
    date: Date,
    name: String,
    biography: String,
    thought: Option<String>,
}

impl App {
    pub fn new(click_count: u32) -> App {
        App {
            click_count,
            double_clicks: 0,
            date: Date::new_0(),
            name: String::new(),
            biography: String::new(),
            thought: None,
        }
    }
}

impl Component<Msg> for App {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Click => {
                self.click_count += 1;
            }
            Msg::DoubleClick => {
                self.double_clicks += 1;
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
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        let date_str: String = self
            .date
            .to_locale_string("en-GB", &JsValue::undefined())
            .into();
        div(
            vec![class("some-class"), id("some-id"), attr("data-id", 1)],
            vec![
                div(
                    vec![id("current-time")],
                    vec![text(format!("Today is {}", date_str))],
                ),
                div(
                    vec![],
                    vec![
                        text("Your name is: "),
                        input(
                            vec![
                                r#type("text"),
                                oninput(|event: InputEvent| {
                                    Msg::ChangeName(event.value)
                                }),
                                placeholder("John Smith"),
                            ],
                            vec![],
                        ),
                        button(
                            vec![onclick(|event: MouseEvent| {
                                trace!(
                                    "Clicked at ({},{})",
                                    event.x(),
                                    event.y()
                                );
                                Msg::Click
                            })],
                            vec![text("Click me!")],
                        ),
                        button(
                            vec![ondblclick(|event: MouseEvent| {
                                trace!(
                                    "Double clicked at ({},{})",
                                    event.x(),
                                    event.y()
                                );
                                Msg::DoubleClick
                            })],
                            vec![text(format!(
                                "DoubleClicks {}",
                                self.double_clicks
                            ))],
                        ),
                    ],
                ),
                p(
                    vec![],
                    vec![
                        text(format!("Hello {}!", self.name,)),
                        if self.click_count > 0 {
                            text(format!(
                                ", You've clicked on that button for {} time{}",
                                self.click_count,
                                if self.click_count > 1 { "s" } else { "" }
                            ))
                        } else {
                            span(vec![], vec![])
                        },
                    ],
                ),
                div(
                    vec![],
                    vec![
                        p(
                            vec![],
                            vec![text("Tell us something about yourself:")],
                        ),
                        div(
                            vec![],
                            vec![textarea(
                                vec![
                                    rows(10),
                                    cols(80),
                                    oninput(|event: InputEvent| {
                                        Msg::ChangeBiography(event.value)
                                    }),
                                    placeholder("I'm a..."),
                                ],
                                vec![],
                            )],
                        ),
                        p(vec![], vec![text(&self.biography)]),
                    ],
                ),
                div(
                    vec![],
                    vec![
                        text("What are you thinking right now?"),
                        input(
                            vec![
                                r#type("text"),
                                onchange(|event: InputEvent| {
                                    Msg::ChangeThought(event.value)
                                }),
                                placeholder("Elephants..."),
                            ],
                            vec![],
                        ),
                        if let Some(thought) = &self.thought {
                            text(format!("Hmmn {}... Interesting.", thought))
                        } else {
                            span(vec![], vec![])
                        },
                    ],
                ),
            ],
        )
    }
}
