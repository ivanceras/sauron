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

const COMPONENT_NAME: &str = "frame";

#[derive(Clone, Debug)]
pub enum Msg {
    AnimateIn,
    StopAnimation,
    HoverIn,
    HoverOut,
    NextAnimation(f64, f64),
}
pub struct Frame {
    hide: bool,
    hover: bool,
    content: String,
}

impl Frame {
    pub fn new_with_content(content: &str) -> Self {
        Frame {
            hide: false,
            hover: false,
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
            Msg::HoverIn => {
                self.hover = true;
                None
            }
            Msg::HoverOut => {
                self.hover = false;
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
        let base = crate::Theme::base();

        let css = jss_ns!(COMPONENT_NAME,{
            // the ROOT component style
            ".": {
                "display": "block",
                "padding": "1px",
                "position": "relative",
                "opacity": 1,
            },

            ".border": {
                "border-color": base.border_color,
                "box-shadow": format!("0 0 4px {}", base.border_shadow),
                "z-index": 1,
                "opacity": 1,
                "position": "absolute",
                "transition": "all 250ms ease-in",
                "border-style": "solid",
            },

            ".hide": {
                "opacity": 0,
            },

            ".hide .border": {
                "height": 0,
                "width": 0,
            },

            ".border-left": {
                "top": "50%",
                "left": 0,
                "height": "100%",
                "transform": "translate(0, -50%)",
                "border-width": "0 0 0 1px",
            },

            ".border-right": {
                "top": "50%",
                "right": 0,
                "height": "100%",
                "transform": "translate(0, -50%)",
                "border-width": "0 0 0 1px",
            },

            ".border-top": {
                "top": 0,
                "left": "50%",
                "width": "100%",
                "transform": "translate(-50%, 0)",
                "border-width": "1px 0 0 0",
            },

            ".border-bottom": {
                "left": "50%",
                "width": "100%",
                "bottom": 0,
                "transform": "translate(-50%, 0)",
                "border-width": "1px 0 0 0",
            },

            ".corner": {
                "width": "24px",
                "height": "24px",
                "border-color": base.corner_color,
                "box-shadow": format!("0 0 4px -2px {}",base.corner_shadow),
                "z-index": 2,
                "opacity": 1,
                "position": "absolute",
                "transition": "all 250ms ease-in",
                "border-style": "solid",
            },

            ".hide .corner": {
                "width": 0,
                "height": 0,
                "opacity": 0,
            },

            ".corner__top-left": {
                "left": "-2px",
                "top": "-2px",
                "border-width": "2px 0 0 2px",
            },

            ".corner__bottom-left": {
                "left": "-2px",
                "bottom": "-2px",
                "border-width": "0 0 2px 2px",
            },

            ".corner__top-right": {
                "right": "-2px",
                "top": "-2px",
                "border-width": "2px 2px 0 0",
            },

            ".corner__bottom-right": {
                "right": "-2px",
                "bottom": "-2px",
                "border-width": "0 2px 2px 0",
            },

            ".content": {
                "background-color": base.content_background_color,
                "z-index": 3,
                "display": "block",
                "position": "relative",
                "overflow": "hidden",
                "transition": "background-color 250ms ease-in",
            },

            ".hide .content": {
                "background-color": "transparent",
            },

        });

        // if expand_corners is enabled
        // the fui_button corners will EXPAND when hovered.
        //
        // CSS Notes:
        // - `.class1.class2 child` means if both class1 and class2 is specified in the
        // parent, the properties will be applied to this child element
        //
        //  - `.class1,.class2 child` means either if either class1 or class2 is specified in the
        // parent, the properties will be applied to this child element
        //
        let expand_corner_css = jss_ns! (COMPONENT_NAME, {
            ".expand_corners.hovered .corner__top-left": {
                "left": "-8px",
                "top": "-8px",
            },

            ".expand_corners.hovered .corner__bottom-left": {
                "left": "-8px",
                "bottom": "-8px",
            },

            ".expand_corners.hovered .corner__top-right": {
                "right": "-8px",
                "top": "-8px",
            },

            ".expand_corners.hovered .corner__bottom-right": {
                "right": "-8px",
                "bottom": "-8px",
            },
        });

        vec![css, expand_corner_css]
    }

    pub fn view(&self) -> Node<Msg> {
        let class_ns =
            |class_names| jss::class_namespaced(COMPONENT_NAME, class_names);

        let classes_ns_flag = |class_name_flags| {
            jss::classes_namespaced_flag(COMPONENT_NAME, class_name_flags)
        };

        div(
            vec![
                class(COMPONENT_NAME),
                classes_ns_flag([
                    ("hide", self.hide),
                    ("expand_corners", true),
                    ("hovered", self.hover),
                ]),
                on_mouseover(|_| Msg::HoverIn),
                on_mouseout(|_| Msg::HoverOut),
            ],
            vec![
                div(vec![class_ns("border border-left")], vec![]),
                div(vec![class_ns("border border-right")], vec![]),
                div(vec![class_ns("border border-top")], vec![]),
                div(vec![class_ns("border border-bottom")], vec![]),
                div(vec![class_ns("corner corner__top-left")], vec![]),
                div(vec![class_ns("corner corner__bottom-left")], vec![]),
                div(vec![class_ns("corner corner__top-right")], vec![]),
                div(vec![class_ns("corner corner__bottom-right")], vec![]),
                div(vec![class_ns("content")], vec![self.child()]),
            ],
        )
    }
}
