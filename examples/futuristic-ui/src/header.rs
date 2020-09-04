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
    AnimateIn,
    StopAnimation,
    NextAnimation(f64, f64),
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
        h1(vec![], vec![text(&self.content)])
    }

    pub fn update(&mut self, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::AnimateIn => {
                log::trace!("frame animate in started...");
                self.hide = true;
                self.start_animation()
            }
            Msg::StopAnimation => {
                log::trace!("header stop_animation..");
                self.hide = false;
                None
            }
            Msg::NextAnimation(start, duration) => {
                log::trace!("next animationg executed..");
                self.next_animation(start, duration)
            }
        }
    }

    fn start_animation(&mut self) -> Option<Msg> {
        log::trace!("header starting animation");
        let duration = 200.0;
        let start = crate::dom::now();
        self.play_sound();
        Some(Msg::NextAnimation(start, duration))
    }

    fn next_animation(&mut self, start: f64, duration: f64) -> Option<Msg> {
        let timestamp = crate::dom::now();
        log::trace!("header next animation: started: {}, duration: {}, timestamp now: {}",start,duration,timestamp);
        let elapsed = timestamp - start;
        log::trace!("elapsed time: {}", elapsed);
        let continue_animation = elapsed < duration;
        log::trace!("continue animation: {}", continue_animation);
        if continue_animation {
            log::trace!("header continue animation");
            Some(Msg::NextAnimation(start, duration))
        } else {
            log::trace!("header stop the animation");
            Some(Msg::StopAnimation)
        }
    }

    pub fn style(&self) -> Vec<String> {
        vec![r#"
        .header {
            display: block;
            padding: 1px;
            position: relative;
            opacity: 1;
        }

        .hide .header{
            opacity: 0;
        }

        .header {
            color: #26dafd;
            font-size: 21px;
            line-height: 1.5;
            font-family: "Titillium Web", "sans-serif";
        }

        .header h1 {
            padding: 0 0;
            margin: 0 4px;
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

        .header-text-anim {
            color: #a1ecfb;
            transition: color 250ms ease-out;
            font-family: "Electrolize", "sans-serif";
            font-weight: bold;
            text-shadow: 0 0 4px rgba(161,236,251,0.65);
            text-transform: uppercase;
        }

        "#
        .to_string()]
    }

    pub fn view(&self) -> Node<Msg> {
        header(
            vec![class("header"), classes_flag([("hide", self.hide)])],
            vec![
                div(
                    vec![class("header-text header-text-anim")],
                    vec![self.child()],
                ),
                div(vec![
                        class("header__border header__border-anim header__border-bottom"),
                    ],
                    vec![],
                ),
            ],
        )
    }
}
