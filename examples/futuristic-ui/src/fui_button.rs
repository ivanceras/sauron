use sauron::{
    html::{
        attributes::class,
        div,
        events::on_click,
        text,
    },
    jss,
    prelude::*,
    Node,
};
use web_sys::HtmlAudioElement;

#[derive(Clone, Debug)]
pub enum Msg<PMSG> {
    Click,
    HoverIn,
    HoverOut,
    HighlightEnd,
    ParamMsg(PMSG),
}

pub struct FuiButton<PMSG> {
    label: String,
    click: bool,
    hover: bool,
    skewed: bool,
    /// whether to use the alt color
    use_alt: bool,
    /// has corners
    has_corners: bool,
    /// enable/disable hover effect
    has_hover: bool,
    disabled: bool,
    event_listeners: Vec<Attribute<Msg<PMSG>>>,
}

impl<PMSG> FuiButton<PMSG>
where
    PMSG: 'static,
{
    pub fn new_with_label(label: &str) -> Self {
        FuiButton {
            label: label.to_string(),
            click: false,
            hover: false,
            skewed: false,
            use_alt: false,
            has_corners: true,
            has_hover: true,
            disabled: false,
            event_listeners: vec![],
        }
    }

    pub fn skewed(&mut self, skewed: bool) {
        self.skewed = skewed;
    }

    pub fn use_alt(&mut self, use_alt: bool) {
        self.use_alt = use_alt;
    }

    pub fn has_corners(&mut self, has_corners: bool) {
        self.has_corners = has_corners;
    }

    pub fn disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
        self.has_corners(false);
        self.has_hover(false);
    }

    pub fn has_hover(&mut self, has_hover: bool) {
        self.has_hover = has_hover;
    }

    pub fn add_event_listeners(
        &mut self,
        event_listeners: Vec<Attribute<PMSG>>,
    ) {
        for ev in event_listeners {
            let mapped_ev = ev.map_msg(|pmsg| Msg::ParamMsg(pmsg));
            self.event_listeners.push(mapped_ev);
        }
    }

    fn play_sound(&self) {
        let audio = HtmlAudioElement::new_with_src("sounds/click.mp3")
            .expect("must not fail");
        let _ = audio.play().expect("must play");
    }

    pub fn style(&self) -> Vec<String> {
        let border_box_shadow_color = "rgba(2,157,187,0.65)";
        let corner_box_shadow_color = "rgba(38,218,253,0.65)";
        let border_border_color = "#029dbb";
        let corner_color = "#26dafd";
        let click_highlight_color = "#029dbb";
        let button_wrap_text_color = "rgba(4,35,41,0.65)";
        let button_text_color = "#acf9fb";
        let alt_border_color = "#090";
        let alt_corner_color = "#0f0";
        let alt_click_highlight_color = "#090";
        let alt_button_text_color = "#0f0";
        let alt_border_box_shadow_color = "rgba(0,153,0,0.65)";
        let alt_corner_box_shadow_color = "rgba(0,255,0,0.65)";
        let disabled_border_color = "#666";
        let disabled_corner_color = "#999";
        let disabled_corner_box_shadow_color = "rgba(153,153,153,0.65)";
        let disabled_border_box_shadow_color = "rgba(102,102,102,0.65)";
        let disabled_button_text_color = "#999";

        let css = jss!({
            ".fui_button": {
                "display": "inline-block",
                "padding": "1px",
                "position": "relative",
                "margin": "4px 4px"
            },

            ".skewed.fui_button": {
                "transform": "skewX(-45deg)",
                "transform-origin": "bottom left",
            },

            ".fui_button__border": {
                "border-color": border_border_color,
                "box-shadow": format!("0 0 4px {}",border_box_shadow_color),
            },

            // HOVER at the lower  part of the button
            ".fui_button__hover": {
                "border-color": border_border_color,
                "box-shadow": format!("0 0 4px {}",border_box_shadow_color),
            },

            ".fui_button__hover-bottom": {
                "width": 0,
                "left": "50%",
                "bottom": "2px",
                "transform": "translate(-50%, 0)",
                "border-width": "4px 0 0 0",
            },

            ".alt .fui_button__hover": {
                "border-color": alt_border_color,
                "box-shadow": format!("0 0 4px {}",alt_border_box_shadow_color),
            },

            ".hover .fui_button__hover": {
                "width": "96%",
            },

            ".fui_button__hover-anim": {
                "z-index": 1,
                "opacity": 1,
                "position": "absolute",
                "transition": "width 100ms ease-in",
                "border-style": "solid",
            },

            ".alt .fui_button__border": {
                "border-color": alt_border_color,
                "box-shadow": format!("0 0 4px {}",alt_border_box_shadow_color),
            },

            ".disabled .fui_button__border": {
                "border-color": disabled_border_color,
                "box-shadow": format!("0 0 4px {}",disabled_border_box_shadow_color),
            },

            ".hide .fui_button__border": {
                "height": 0,
                "width": 0,
            },

            ".fui_button__border-left": {
                "top": "50%",
                "left": 0,
                "height": "100%",
                "transform": "translate(0, -50%)",
                "border-width": "0 0 0 1px",
            },

            ".fui_button__border-anim": {
                "z-index": 1,
                "opacity": 1,
                "position": "absolute",
                "transition": "all 250ms ease-in",
                "border-style": "solid",
            },

            ".fui_button__border-right": {
                "top": "50%",
                "right": 0,
                "height": "100%",
                "transform": "translate(0, -50%)",
                "border-width": "0 0 0 1px",
            },

            ".fui_button__border-top": {
                "top": 0,
                "left": "50%",
                "width": "100%",
                "transform": "translate(-50%, 0)",
                "border-width": "1px 0 0 0",
            },

            ".fui_button__border-bottom": {
                "left": "50%",
                "width": "100%",
                "bottom": 0,
                "transform": "translate(-50%, 0)",
                "border-width": "1px 0 0 0",
            },

            ".fui_button__corner": {
                "width": "8px",
                "height": "8px",
                "border-color": corner_color,
                "box-shadow": format!("0 0 4px -2px {}",corner_box_shadow_color),
            },

            ".alt .fui_button__corner": {
                "border-color": alt_corner_color,
                "box-shadow": format!("0 0 4px {}",alt_corner_box_shadow_color),
            },

            ".disabled .fui_button__corner": {
                "border-color": disabled_corner_color,
                "box-shadow": format!("0 0 4px {}",disabled_corner_box_shadow_color),
            },

            ".hide .fui_button__corner": {
                "width": 0,
                "height": 0,
                "opacity": 0,
            },

            ".fui_button__corner-anim": {
                "z-index": 2,
                "opacity": 1,
                "position": "absolute",
                "transition": "all 250ms ease-in",
                "border-style": "solid",
            },

            ".fui_button_corner__top-left": {
                "left": "-2px",
                "top": "-2px",
                "border-width": "2px 0 0 2px",
            },

            ".fui_button_corner__bottom-left": {
                "left": "-2px",
                "bottom": "-2px",
                "border-width": "0 0 2px 2px",
            },

            ".fui_button_corner__top-right": {
                "right": "-2px",
                "top": "-2px",
                "border-width": "2px 2px 0 0",
            },

            ".fui_button_corner__bottom-right": {
                "right": "-2px",
                "bottom": "-2px",
                "border-width": "0 2px 2px 0",
            },

            ".fui_button-text": {
                "background-color": button_wrap_text_color,
            },

            ".hide .fui_button-text": {
                "background-color": "transparent",
            },

            ".fui_button-text-anim": {
                "z-index": 3,
                "display": "block",
                "position": "relative",
                "overflow": "hidden",
                "transition": "background-color 250ms ease-in",
            },

            ".fui_button__button": {
                "color": button_text_color,
                "cursor": "pointer"
            },

            ".alt .fui_button__button": {
                "color": alt_button_text_color,
            },

            ".disabled .fui_button__button": {
                "color": disabled_button_text_color,
                "cursor": "auto",
            },

            ".skewed .fui_button__button": {
                "transform": "skewX(45deg)",
            },

            ".fui_button__button-anim": {
                "margin": 0,
                "border": "none",
                "z-index": 2,
                "display": "inline-block",
                "padding": "10px 20px",
                "outline": "none",
                "position": "relative",
                "font-size": "15.75px",
                "background": "transparent",
                "transition": "all 250ms ease-out",
                "line-height": 1,
                "user-select": "none",
                "vertical-align": "middle",
            },

            ".fui_button__highlight": {
                  "z-index": 1,
                  "position": "absolute",
                  "left": 0,
                  "right": 0,
                  "top": 0,
                  "bottom": 0,
                  "background-color": "transparent",
                  "opacity": 0,
            },

            ".click .fui_button__highlight": {
                "opacity": 1,
                "background-color": click_highlight_color,
            },

            ".alt .fui_button__highlight": {
                "background-color": alt_click_highlight_color,
            },

            ".fui_button__highlight-anim": {
                "transition": "all 50ms ease-out",
            }
        });

        vec![css]
    }

    pub fn update(&mut self, msg: Msg<PMSG>) -> Option<PMSG> {
        match msg {
            Msg::Click => {
                self.play_sound();
                self.click = true;
                None
            }
            Msg::HoverIn => {
                if self.has_hover {
                    self.hover = true;
                }
                None
            }
            Msg::HoverOut => {
                if self.has_hover {
                    self.hover = false;
                }
                None
            }
            Msg::HighlightEnd => {
                self.click = false;
                None
            }
            Msg::ParamMsg(pmsg) => {
                // we return a parent msg, this is meant to be executed in the calling
                // component
                Some(pmsg)
            }
        }
    }

    pub fn view(&self) -> Node<Msg<PMSG>> {
        div(
            vec![
                class("fui_button"),
                classes_flag([
                    ("click", self.click),
                    ("hover", self.hover),
                    ("skewed", self.skewed),
                    ("alt", self.use_alt),
                    ("disabled", self.disabled),
                ]),
            ],
            vec![
                // hover
                view_if(self.has_hover,
                    div(vec![class("fui_button__hover fui_button__hover-anim fui_button__hover-bottom")], vec![]),
                ),
                //borders
                div(vec![class("fui_button__border fui_button__border-anim fui_button__border-bottom")], vec![]),
                div(vec![class("fui_button__border fui_button__border-anim fui_button__border-left")], vec![]),
                div(vec![class("fui_button__border fui_button__border-anim fui_button__border-right")], vec![]),
                div(vec![class("fui_button__border fui_button__border-anim fui_button__border-top")], vec![]),
                div(vec![class("fui_button__border fui_button__border-anim fui_button__border-bottom")], vec![]),
                // corners
                view_if(self.has_corners,
                    div(vec![class("fui_button__corner fui_button__corner-anim fui_button_corner__top-left")], vec![])
                ),
                view_if(self.has_corners,
                    div(
                        vec![class("fui_button__corner fui_button__corner-anim fui_button_corner__bottom-left")],
                        vec![],
                    )
                ),
                view_if(self.has_corners,
                    div(
                        vec![class("fui_button__corner fui_button__corner-anim fui_button_corner__top-right")],
                        vec![],
                    )
                ),
                view_if(self.has_corners,
                    div(
                        vec![class("fui_button__corner fui_button__corner-anim fui_button_corner__bottom-right")],
                        vec![],
                    )
                ),
                div(vec![class("fui_button__wrap")],
                    vec![
                        div(
                            vec![class("fui_button-text fui_button-text-anim")],
                            vec![
                                button(
                                    vec![
                                        class("fui_button__button fui_button__button-anim"),
                                        disabled(self.disabled),
                                        on_click(|_|Msg::Click),
                                        on_mouseover(|_|Msg::HoverIn),
                                        on_mouseout(|_|Msg::HoverOut),
                                    ],
                                    vec![text(&self.label)]
                                ).add_attributes(self.event_listeners.clone())
                            ],
                        ),
                        div(vec![
                            class("fui_button__highlight fui_button__highlight-anim"),
                            on_transitionend(|_|Msg::HighlightEnd),
                            ],
                            vec![]
                        )
                    ]
                ),
            ],
        )
    }
}
