#![deny(warnings)]
use sauron::html::attributes::{class, id, style};
use sauron::html::events::on_click;
use sauron::html::{div, text};
use sauron::prelude::*;
use sauron::{Cmd, Component, Node, Program};
use web_sys::HtmlAudioElement;

pub enum Msg {
    ToggleShow,
}

pub struct App {
    show: bool,
}

impl App {
    pub fn new() -> Self {
        App { show: true }
    }

    fn play_sound(&self) {
        let audio = HtmlAudioElement::new_with_src("/sounds/deploy.mp3")
            .expect("must not fail");
        let _ = audio.play().expect("must play");
    }
}

impl Component<Msg> for App {
    fn init(&self) -> Cmd<Self, Msg> {
        self.play_sound();
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        div(
            vec![],
            vec![
                div(
                    vec![class("container")],
                    vec![
                        div(vec![classes_flag([("frame",true), ("hide", !self.show)]),id("frame1"),],vec![
                            div(vec![class("border border-anim border-left")],vec![]),
                            div(vec![class("border border-anim border-right")],vec![]),
                            div(vec![class("border border-anim border-top")],vec![]),
                            div(vec![class("border border-anim border-bottom")],vec![]),
                            div(vec![class("corner corner-anim corner__top-left")],vec![]),
                            div(vec![class("corner corner-anim corner__bottom-left")],vec![]),
                            div(vec![class("corner corner-anim corner__top-right")],vec![]),
                            div(vec![class("corner corner-anim corner__bottom-right")],vec![]),
                            div(vec![class("frame-text frame-text-anim")],vec![
                                div(vec![styles([("padding", "20px 40px"), ("font-size", "32px")])],vec![text("FutureosTech")]),
                            ]),
                        ]),
                        button(vec![on_click(|_|Msg::ToggleShow),style("margin-top","20px")],vec![text("Toggle")]),
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
                self.show = !self.show;
                self.play_sound();
            }
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
