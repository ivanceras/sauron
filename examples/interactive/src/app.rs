#![deny(warnings)]
use js_sys::Date;
use sauron::{html::attributes::*, html::events::*, html::*, jss, web_sys::MouseEvent, *};

pub enum Msg {
    Click,
    DoubleClick,
    Clock,
    ChangeName(String),
    ChangeBiography(String),
    ChangeThought(String),
}

#[derive(Clone)]
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

impl Application<Msg> for App {
    fn init(&mut self) -> Cmd<Self, Msg> {
        Cmd::new(|mut program| {
            let clock: Closure<dyn FnMut()> = Closure::new(move || {
                program.dispatch(Msg::Clock);
            });
            window()
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    clock.as_ref().unchecked_ref(),
                    1000,
                )
                .expect("Unable to start interval");
            clock.forget();
        })
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Click => {
                self.click_count += 1;
                log::info!("click count: {}", self.click_count);
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
                if !thought.is_empty() {
                    self.thought = Some(thought);
                } else {
                    self.thought = None;
                }
            }
        }
        Cmd::none()
    }

    fn pre_diff(&self, old: &Self) -> Option<Vec<PreDiff>> {
        Some(vec![
            diff_if(false,
                [
                    diff_if(false, [diff_if(true, [])]),
                    diff_if(false,
                        [
                            diff_if(false, []),
                            diff_if(false, []),
                            diff_if(false, []),
                            diff_if(false,
                                [diff_if(self.double_clicks != old.double_clicks, [])],
                            ),
                        ],
                    ),
                    diff_if(false,
                        [
                            diff_if(self.name != old.name, []),
                            diff_if(false, []), // separator for in between text here
                            diff_if(self.click_count != old.click_count, [])
                        ],
                    ),
                    diff_if(false,
                        [
                            diff_if(false, []),
                            diff_if(false, []),
                            diff_if(false, [diff_if(self.biography != old.biography, [])]),
                        ],
                    ),
                    diff_if(false,
                        [
                            diff_if(false, []),
                            diff_if(false, []),
                            diff_if(self.thought != old.thought, []),
                        ],
                    ),
                ],
            )]
       )
    }

    fn view(&self) -> Node<Msg> {
        let date_str: String = self
            .date
            .to_locale_string("en-GB", &JsValue::undefined())
            .into();
        div(
            [class("some-class"), id("some-id"), attr("data-id", 1)],
            [
                div([id("current-time")], [text!("Today is {}", date_str)]),
                div(
                    [],
                    [
                        text("Your name is: "),
                        input(
                            [
                                r#type("text"),
                                on_input(|event: InputEvent| Msg::ChangeName(event.value())),
                                placeholder("John Smith"),
                            ],
                            [],
                        ),
                        button(
                            [on_click(|event: MouseEvent| {
                                trace!("Clicked at ({},{})", event.x(), event.y());
                                Msg::Click
                            })],
                            [text!("Click me!")],
                        ),
                        button(
                            [on_dblclick(|event: MouseEvent| {
                                trace!("Double clicked at ({},{})", event.x(), event.y());
                                Msg::DoubleClick
                            })],
                            [text!("DoubleClicks {}", self.double_clicks)],
                        ),
                    ],
                ),
                p(
                    [],
                    [
                        text!("Hello {}!", self.name),
                        if self.click_count > 0 {
                            text!(
                                ", You've clicked on that button for {} time{}",
                                self.click_count,
                                if self.click_count > 1 { "s" } else { "" }
                            )
                        } else {
                            text("here..")
                        },
                    ],
                ),
                div(
                    [],
                    [
                        p([], [text!("Tell us something about yourself:")]),
                        div(
                            [],
                            [textarea(
                                [
                                    rows(10),
                                    cols(80),
                                    on_input(|event: InputEvent| {
                                        Msg::ChangeBiography(event.value())
                                    }),
                                    placeholder("I'm a..."),
                                ],
                                [],
                            )],
                        ),
                        p([], [text!("{}", self.biography)]),
                    ],
                ),
                div(
                    [],
                    [
                        text("What are you thinking right now?"),
                        input(
                            [
                                r#type("text"),
                                on_change(|event: InputEvent| Msg::ChangeThought(event.value())),
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

    fn stylesheet() -> Vec<String> {
        vec![jss! {
            "body": {
                font_family: "Fira Sans, Courier New, Courier, Lucida Sans Typewriter, Lucida Typewriter, monospace",
            }
        }]
    }
}
