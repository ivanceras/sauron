use sauron::html::attributes::{class, id, style};
use sauron::html::events::on_click;
use sauron::html::{div, text};
use sauron::prelude::*;
use sauron::{Cmd, Component, Node, Program};
use std::marker::PhantomData;
use web_sys::HtmlAudioElement;

pub enum Msg<MSG> {
    AnimateIn,
    StopAnimation,
    NextAnimation(bool, String, f64, f64),
    ParamMsg(MSG),
}

pub struct Words<MSG> {
    _phantom: PhantomData<MSG>,
    child: String,
    text: String,
    animating: bool,
}

impl<MSG> Words<MSG> {
    pub fn new_with_content(content: &str) -> Self {
        Words {
            child: content.to_string(),
            text: "".to_string(),
            animating: false,
            _phantom: PhantomData,
        }
    }

    fn play_sound(&self) {
        let audio = HtmlAudioElement::new_with_src("sounds/typing.mp3")
            .expect("must not fail");
        let _ = audio.play().expect("must play");
    }

    pub fn animate_in(&mut self) -> Cmd<crate::App, crate::Msg> {
        self.play_sound();
        self.start_animation(true)
    }

    fn stop_animation(&mut self) -> Cmd<crate::App, crate::Msg> {
        self.animating = false;
        Cmd::none()
    }

    fn start_animation(&mut self, is_in: bool) -> Cmd<crate::App, crate::Msg> {
        use wasm_bindgen::JsCast;

        let text_len = self.child.len();

        if text_len == 0 {
            return Cmd::none();
        }

        let interval = 1_000.0 / 60.0;
        let real_duration = interval * text_len as f64;
        let timeout = 250.0;
        let duration = real_duration.min(timeout);
        let start = crate::dom::now();

        self.animating = true;
        if is_in {
            self.text = self.child.to_string();
        }
        let child_text = self.child.clone();

        log::trace!("returning a cmd for next animation..");
        Cmd::new(move |program| {
            program.dispatch(crate::Msg::WordsMsg(Box::new(
                Msg::NextAnimation(is_in, child_text.clone(), start, duration),
            )))
        })
    }

    fn next_animation(
        &mut self,
        is_in: bool,
        child_text: String,
        start: f64,
        duration: f64,
    ) -> Cmd<crate::App, crate::Msg> {
        let timestamp = crate::dom::now();
        let text_len = child_text.len();

        let mut anim_progress = (timestamp - start).max(0.0);
        if !is_in {
            anim_progress = duration - anim_progress;
        }

        log::trace!("duration: {}", duration);
        log::trace!("timestamp: {}", timestamp);
        log::debug!("text_len: {}", text_len);
        log::debug!("animation progress: {}", anim_progress);

        let new_length =
            (anim_progress * text_len as f64 / duration).round() as usize;

        log::trace!("new_length: {}", new_length);
        self.text = child_text.to_string();
        self.text.truncate(new_length);
        log::trace!("{}", self.text);

        let continue_animation = if is_in {
            new_length <= text_len
        } else {
            new_length > 0
        };

        if continue_animation {
            log::trace!("continue animation");
            Cmd::new(move |program| {
                program.dispatch(crate::Msg::WordsMsg(Box::new(
                    Msg::NextAnimation(
                        is_in,
                        child_text.clone(),
                        start,
                        duration,
                    ),
                )))
            })
        } else {
            log::trace!("stop the animation");
            Cmd::new(move |program| {
                program.dispatch(crate::Msg::WordsMsg(Box::new(
                    Msg::StopAnimation,
                )))
            })
        }
    }

    pub fn update_external(
        &mut self,
        msg: Msg<MSG>,
    ) -> Cmd<crate::App, crate::Msg> {
        log::trace!("words updating..");
        match msg {
            Msg::AnimateIn => {
                log::trace!("animate in started...");
                self.animate_in()
            }
            Msg::StopAnimation => {
                log::trace!("words stop_animation..");
                self.stop_animation();
                Cmd::none()
            }
            Msg::NextAnimation(is_in, child_text, start, duration) => {
                log::trace!("next animationg executed..");
                self.next_animation(is_in, child_text, start, duration)
            }
            Msg::ParamMsg(msg) => Cmd::none(),
        }
    }

    pub fn style(&self) -> Vec<String> {
        vec![r#"
            .words {
                display: inline-block;
                position: relative;
            }

            .text {
              position: absolute;
              left: 0;
              right: 0;
              top: 0;
              overflow: hidden;
              display: inline-block;
              opacity: 0;
            }

            .blink {
              position: relative;
              width: 0;
              height: 0;
              display: inline-block;
              animation: words_blink-anim 250ms step-end infinite;
            }

            .hide {
              opacity: 0;
            }

            .animating .children {
                opacity: 0
             }

            .animating .text {
                opacity: 1
            }

            @keyframes words_blink-anim {
              0%, 100% {
                color: transparent;
              }

              50% {
                color: inherit;
              }
            }
            "#
        .to_string()]
    }

    pub fn view(&self) -> Node<MSG> {
        div(
            vec![],
            vec![span(
                vec![
                    class("words"),
                    classes_flag([("animating", self.animating)]),
                ],
                vec![
                    span(vec![class("children")], vec![text(&self.child)]),
                    view_if(
                        true,
                        span(
                            vec![class("text")],
                            vec![
                                text(&self.text),
                                span(vec![class("blink")], vec![text("█")]),
                            ],
                        ),
                    ),
                ],
            )],
        )
    }
}
