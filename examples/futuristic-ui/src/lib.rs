//#![deny(warnings)]
use frame::Frame;
use fui_button::FuiButton;
use sauron::html::attributes::{class, id, style};
use sauron::html::events::on_click;
use sauron::html::{div, text};
use sauron::prelude::*;
use sauron::{Cmd, Component, Node, Program};
use spinner::Spinner;
use web_sys::HtmlAudioElement;
use words::Words;

mod frame;
mod fui_button;
mod spinner;
mod words;

pub enum Msg {
    ToggleShow,
    FrameMsg(frame::Msg),
    FuiButtonMsg(fui_button::Msg),
    WordsMsg(Box<words::Msg<Self>>),
    AnimateWords,
    NoOp,
}

pub struct App {
    show: bool,
    frame: Frame,
    fui_button: FuiButton,
    spinner: Spinner<Msg>,
    words: Words<Msg>,
}

impl App {
    pub fn new() -> Self {
        App {
            show: true,
            frame: Frame::new(),
            fui_button: FuiButton::new(),
            spinner: Spinner::new(),
            words: Words::new(),
        }
    }
}

impl Component<Msg> for App {
    fn init(&self) -> Cmd<Self, Msg> {
        Cmd::none()
    }

    fn style(&self) -> Vec<String> {
        vec![r#"
        .container {
            color: #26dafd;
            font-size: 21px;
            line-height: 1.5;
            font-family: "Titillium Web", "sans-serif";
            margin: 100px;
        }
        .container ::selection {
            color: #021114;
            text-shadow: none;
            background-color: #26dafd;
        }
        "#
        .to_string()]
        .into_iter()
        .chain(self.frame.style().into_iter())
        .chain(self.fui_button.style().into_iter())
        .chain(self.spinner.style().into_iter())
        .chain(self.words.style().into_iter())
        .collect()
    }

    fn view(&self) -> Node<Msg> {
        div(
            vec![],
            vec![
                div(
                    vec![class("container")],
                    vec![
                        self.frame
                            .view()
                            .map_msg(|frame_msg| Msg::FrameMsg(frame_msg)),
                        div(
                            vec![
                                style("width", px(100)),
                                style("height", px(20)),
                            ],
                            vec![],
                        ),
                        button(
                            vec![
                                on_click(|_| Msg::ToggleShow),
                                style("margin", "20px"),
                                style("display", "block"),
                            ],
                            vec![text("Toggle")],
                        ),
                        self.fui_button.view().map_msg(Msg::FuiButtonMsg),
                        self.spinner.view(),
                        self.words.view().map_msg(|words_msg| {
                            Msg::WordsMsg(Box::new(words_msg))
                        }),
                        button(
                            vec![
                                on_click(|_| Msg::AnimateWords),
                                style("margin", "20px"),
                                style("display", "block"),
                            ],
                            vec![text("Animate words")],
                        ),
                    ],
                ),
                footer(
                    vec![],
                    vec![a(
                        vec![href("https://github.com/ivanceras/futureostech")],
                        vec![text("code")],
                    )],
                ),
            ],
        )
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::ToggleShow => {
                self.frame.update(frame::Msg::ToggleShow);
                Cmd::none()
            }
            Msg::FrameMsg(frame_msg) => {
                self.frame.update(frame_msg);
                Cmd::none()
            }
            Msg::FuiButtonMsg(fui_btn_msg) => {
                self.fui_button.update(fui_btn_msg);
                Cmd::none()
            }
            Msg::WordsMsg(word_msg) => {
                log::trace!("animating words..");
                {
                    self.words.update(*word_msg);
                    log::trace!("got a cmd..");
                }
                Cmd::none()
            }
            Msg::AnimateWords => {
                let cmd = self.words.update(words::Msg::AnimateIn);
                log::trace!("got a returned cmd from animate words");
                Cmd::none()
            }
            Msg::NoOp => Cmd::none(),
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}
