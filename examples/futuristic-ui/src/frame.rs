use sauron::{
    html::{
        attributes::{
            class,
            id,
            style,
        },
        div,
        events::on_click,
        text,
    },
    prelude::*,
    Cmd,
    Component,
    Node,
    Program,
};
use web_sys::HtmlAudioElement;

#[derive(Clone, Debug)]
pub enum Msg {
    ToggleShow,
    TriggerAnimation,
    NextAnimation,
    TransitionEnd,
}

pub struct Frame {
    hide: bool,
    content: String,
}

impl Frame {
    pub fn new_with_content(content: &str) -> Self {
        Frame {
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
        div(
            vec![styles([("padding", "20px 40px"), ("font-size", "32px")])],
            vec![text(&self.content)],
        )
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
                log::trace!("frame: animate in..");
                self.hide = true;
                Cmd::new(|program| {
                    let timed_closure: Closure<dyn Fn()> =
                        Closure::wrap(Box::new(move || {
                            program.dispatch(crate::Msg::FrameMsg(
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
                log::trace!("frame: animate out..");
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
        .frame {
            display: block;
            padding: 1px;
            position: relative;
        }

        .border {
            border-color: #029dbb;
            box-shadow: 0 0 4px rgba(2,157,187,0.65);
        }

        .hide{
            opacity: 0;
        }

        .hide .border {
          height: 0;
          width: 0;
        }

        .border-left {
            top: 50%;
            left: 0;
            height: 100%;
            transform: translate(0, -50%);
            border-width: 0 0 0 1px;
        }


        .border-anim {
            z-index: 1;
            opacity: 1;
            position: absolute;
            transition: all 250ms ease-in;
            border-style: solid;
        }

        .border-right {
            top: 50%;
            right: 0;
            height: 100%;
            transform: translate(0, -50%);
            border-width: 0 0 0 1px;
        }


        .border-top {
            top: 0;
            left: 50%;
            width: 100%;
            transform: translate(-50%, 0);
            border-width: 1px 0 0 0;
        }


        .border-bottom {
            left: 50%;
            width: 100%;
            bottom: 0;
            transform: translate(-50%, 0);
            border-width: 1px 0 0 0;
        }


        .corner {
            width: 24px;
            height: 24px;
            border-color: #26dafd;
            box-shadow: 0 0 4px -2px rgba(38,218,253,0.65);
        }

        .hide .corner{
            width: 0;
            height: 0;
            opacity: 0;
        }

        .corner-anim {
            z-index: 2;
            opacity: 1;
            position: absolute;
            transition: all 250ms ease-in;
            border-style: solid;
        }

        .corner__top-left {
            left: -2px;
            top: -2px;
            border-width: 2px 0 0 2px;
        }


        .corner__bottom-left {
            left: -2px;
            bottom: -2px;
            border-width: 0 0 2px 2px;
        }


        .corner__top-right {
            right: -2px;
            top: -2px;
            border-width: 2px 2px 0 0;
        }


        .corner__bottom-right {
            right: -2px;
            bottom: -2px;
            border-width: 0 2px 2px 0;
        }


        .frame-text {
            background-color: rgba(4,35,41,0.65);
        }

        .hide .frame-text {
            background-color: transparent;
        }

        .frame-text-anim {
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
            vec![classes_flag([("frame", true), ("hide", self.hide)])],
            vec![
                div(
                    vec![
                        class("border border-anim border-left"),
                        on_transitionend(|_| Msg::TransitionEnd),
                    ],
                    vec![],
                ),
                div(vec![class("border border-anim border-right")], vec![]),
                div(vec![class("border border-anim border-top")], vec![]),
                div(vec![class("border border-anim border-bottom")], vec![]),
                div(vec![class("corner corner-anim corner__top-left")], vec![]),
                div(
                    vec![class("corner corner-anim corner__bottom-left")],
                    vec![],
                ),
                div(
                    vec![class("corner corner-anim corner__top-right")],
                    vec![],
                ),
                div(
                    vec![class("corner corner-anim corner__bottom-right")],
                    vec![],
                ),
                div(
                    vec![class("frame-text frame-text-anim")],
                    vec![self.child()],
                ),
            ],
        )
    }
}
