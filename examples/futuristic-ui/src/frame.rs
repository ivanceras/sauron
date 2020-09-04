use sauron::{
    html::{
        attributes::class,
        div,
        text,
    },
    prelude::*,
    Node,
};
use web_sys::HtmlAudioElement;

#[derive(Clone, Debug)]
pub enum Msg {
    AnimateIn,
    StopAnimation,
    NextAnimation(f64, f64),
}
pub struct Frame {
    hide: bool,
    content: String,
}

impl Frame {
    pub fn new_with_content(content: &str) -> Self {
        Frame {
            hide: false,
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

    pub fn update(&mut self, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::AnimateIn => {
                self.hide = true;
                self.start_animation()
            }
            Msg::StopAnimation => {
                self.hide = false;
                None
            }
            Msg::NextAnimation(start, duration) => {
                self.next_animation(start, duration)
            }
        }
    }

    fn start_animation(&mut self) -> Option<Msg> {
        let duration = 200.0;
        let start = crate::dom::now();
        self.play_sound();
        Some(Msg::NextAnimation(start, duration))
    }

    fn next_animation(&mut self, start: f64, duration: f64) -> Option<Msg> {
        let timestamp = crate::dom::now();
        let elapsed = timestamp - start;
        let continue_animation = elapsed < duration;
        if continue_animation {
            Some(Msg::NextAnimation(start, duration))
        } else {
            Some(Msg::StopAnimation)
        }
    }

    pub fn style(&self) -> Vec<String> {
        vec![r#"
        .frame {
            display: block;
            padding: 1px;
            position: relative;
            opacity: 1;
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
                div(vec![class("border border-anim border-left")], vec![]),
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
