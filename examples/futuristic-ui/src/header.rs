use sauron::html::attributes::{class, id, style};
use sauron::html::events::on_click;
use sauron::html::{div, text};
use sauron::prelude::*;
use sauron::{Cmd, Component, Node, Program};
use web_sys::HtmlAudioElement;

pub enum Msg {
    ToggleShow,
    TriggerAnimation,
    NextAnimation,
    TransitionEnd,
}

pub struct Header {
    hide: bool,
    content: String,
}

impl Header {
    pub fn new_with_content(content: &str) -> Self {
        Header {
            hide: true,
            content: content.to_string(),
        }
    }

    fn play_sound(&self) {
        let audio = HtmlAudioElement::new_with_src("sounds/deploy.mp3")
            .expect("must not fail");
        let _ = audio.play().expect("must play");
    }

    fn child(&self) -> Node<Msg> {
        div(vec![], vec![text(&self.content)])
    }

    pub fn update(&mut self, msg: Msg) -> Cmd<crate::App, crate::Msg> {
        match msg {
            Msg::ToggleShow => {
                self.hide = !self.hide;
                self.play_sound();
                Cmd::none()
            }
            // we hide the borders then have delayed timeout
            // call before showing it using a Cmd with timed closure
            Msg::TriggerAnimation => {
                use sauron::wasm_bindgen::JsCast;
                log::trace!("header: animate in..");
                self.hide = true;
                Cmd::new(|program| {
                    let timed_closure: Closure<dyn Fn()> =
                        Closure::wrap(Box::new(move || {
                            program.dispatch(crate::Msg::HeaderMsg(
                                Msg::NextAnimation,
                            ));
                        }));

                    web_sys::window()
                        .expect("no global `window` exists")
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            timed_closure.as_ref().unchecked_ref(),
                            200,
                        )
                        .expect("Unable to start interval");
                    timed_closure.forget();
                })
            }
            Msg::NextAnimation => {
                log::trace!("header: animate out..");
                self.play_sound();
                self.hide = false;
                Cmd::none()
            }
            Msg::TransitionEnd => {
                log::trace!("animation end..");
                Cmd::none()
            }
        }
    }

    pub fn style(&self) -> Vec<String> {
        vec![r#"
        .header {
            display: block;
            padding: 1px;
            position: relative;
        }

        .header__border {
            border-color: #029dbb;
            box-shadow: 0 0 4px rgba(2,157,187,0.65);
        }

        .hide .header__border {
          height: 0;
          width: 0;
        }


        .header__border-anim {
            z-index: 1;
            opacity: 1;
            position: absolute;
            transition: all 250ms ease-in;
            border-style: solid;
        }

        .header__border-bottom {
            left: 50%;
            width: 100%;
            bottom: 0;
            transform: translate(-50%, 0);
            border-width: 4px 0 0 0;
        }

        .header-text{
            font-size: 32px;
            padding: 10px 0;
        }

        .header-text-anim {
            z-index: 3;
            display: block;
            position: relative;
            overflow: hidden;
            transition: background-color 250ms ease-in;
        }

        "#
        .to_string()]
    }

    pub fn view(&self) -> Node<Msg> {
        div(
            vec![classes_flag([("header", true), ("hide", self.hide)])],
            vec![
                div(
                    vec![class("header-text header-text-anim")],
                    vec![self.child()],
                ),
                div(vec![
                        class("header__border header__border-anim header__border-bottom"),
                        on_transitionend(|_| Msg::TransitionEnd),
                    ],
                    vec![],
                ),
            ],
        )
    }
}
