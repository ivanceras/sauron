use crate::sounds;
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

const COMPONENT_NAME: &str = "header";

#[derive(Clone, Debug)]
pub enum Msg {
    AnimateIn,
    StopAnimation,
    NextAnimation(f64, f64),
}

pub struct Header {
    audio: HtmlAudioElement,
    hide: bool,
    content: String,
}

impl Header {
    pub fn new_with_content(content: &str) -> Self {
        Header {
            audio: sounds::preload("sounds/deploy.mp3"),
            hide: false,
            content: content.to_string(),
        }
    }

    fn child(&self) -> Node<Msg> {
        h1(vec![], vec![text(&self.content)])
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
        sounds::play(&self.audio);
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
        let base = crate::Theme::base();
        let css = jss_ns!(COMPONENT_NAME, {
            ".": {
                "display": "block",
                "padding": "1px",
                "position": "relative",
                "opacity": 1,
                "color": base.secondary_color,
                "font-size": "21px",
                "line-height": 1.5,
                "font-family": "\"Titillium Web\", \"sans-serif\"",
            },

            ".hide": {
                "opacity": 0,
            },

            ".header h1": {
                "padding": "0 0",
                "margin": "0 4px",
            },

            ".border": {
                "border-color": base.controls.border_color,
                "box-shadow": format!("0 0 4px {}",base.controls.border_shadow),
                "z-index": 1,
                "opacity": 1,
                "position": "absolute",
                "transition": "all 250ms ease-in",
                "border-style": "solid",
            },

            ".hide .border": {
              "height": 0,
              "width": 0,
            },

            ".border-bottom": {
                "left": "50%",
                "width": "100%",
                "bottom": 0,
                "transform": "translate(-50%, 0)",
                "border-width": "4px 0 0 0",
            },

            ".text-anim": {
                "color": base.accent_color,
                "transition": "color 250ms ease-out",
                "font-family": "\"Electrolize\", \"sans-serif\"",
                "font-weight": "bold",
                "text-shadow": format!("0 0 4px {}",base.accent_shadow),
                "text-transform": "uppercase",
            },

        });

        vec![css]
    }

    pub fn view(&self) -> Node<Msg> {
        let class_ns =
            |class_names| jss::class_namespaced(COMPONENT_NAME, class_names);

        let classes_ns_flag = |class_name_flags| {
            jss::classes_namespaced_flag(COMPONENT_NAME, class_name_flags)
        };
        header(
            vec![
                class(COMPONENT_NAME),
                classes_ns_flag([("hide", self.hide)]),
            ],
            vec![
                div(vec![class_ns("text text-anim")], vec![self.child()]),
                div(vec![class_ns("border border-bottom")], vec![]),
            ],
        )
    }
}
