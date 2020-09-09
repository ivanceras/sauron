use crate::sounds;
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

const COMPONENT_NAME: &str = "fui_button";

#[derive(Clone, Debug)]
pub enum Msg<PMSG> {
    Click,
    HoverIn,
    HoverOut,
    HighlightEnd,
    ParamMsg(PMSG),
}

pub struct FuiButton<PMSG> {
    audio: HtmlAudioElement,
    options: Options,
    label: String,
    click: bool,
    hover: bool,
    event_listeners: Vec<Attribute<Msg<PMSG>>>,
}

pub struct Options {
    pub hidden: bool,
    /// enable sound
    pub sound: bool,
    /// enable click effect, which changes the background color
    /// of the button with the highlight color
    pub click_highlights: bool,
    /// the button is slanted 45 degree to the right
    pub skewed: bool,
    /// has corners
    pub has_corners: bool,
    /// enable/disable hover effect
    pub has_hover: bool,
    /// expand corners when hovered
    pub expand_corners: bool,
    /// the button is disabled
    pub disabled: bool,
}

impl<PMSG> FuiButton<PMSG>
where
    PMSG: 'static,
{
    pub fn new_with_label(label: &str) -> Self {
        let options = Options::regular();
        FuiButton {
            audio: sounds::preload("sounds/click.mp3"),
            options,
            click: false,
            hover: false,
            label: label.to_string(),
            event_listeners: vec![],
        }
    }

    pub fn set_options(&mut self, options: Options) {
        self.options = options;
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

    pub fn style(&self) -> Vec<String> {
        let base = crate::Theme::default().controls;

        let base_css = jss_ns!(COMPONENT_NAME, {

            // the ROOT component style
            ".": {
                "display": "inline-block",
                "padding": "1px",
                "position": "relative",
                "margin": "4px 4px"
            },

            ".hidden" : {
                "visibility": "hidden",
            },

            // HOVER at the lower  part of the button
            ".hover": {
                "border-color": base.hover_color,
                "box-shadow": format!("0 -2px 4px {}",base.hover_shadow),
                "z-index": 4,
                "opacity": 1,
                "position": "absolute",
                "transition": "width 100ms ease-in",
                "border-style": "solid",
            },

            ".has_hover.hovered .hover": {
                "width": "96%",
            },

            ".hover-bottom": {
                "width": 0,
                "left": "50%",
                "bottom": "2px",
                "transform": "translate(-50%, 0)",
                "border-width": "4px 0 0 0",
            },


            // BORDERS these are styled divs wrapping the buttons
            ".border": {
                "border-color": base.border_color,
                "box-shadow": format!("0 0 4px {}",base.border_shadow),
                "z-index": 1,
                "opacity": 1,
                "position": "absolute",
                "transition": "all 250ms ease-in",
                "border-style": "solid",
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

            // CORNERS - the fancy divs which clips the button
            ".corner": {
                "width": "8px",
                "height": "8px",
                "border-color": base.corner_color,
                "box-shadow": format!("0 0 4px -2px {}",base.corner_shadow),
                "z-index": 2,
                "opacity": 1,
                "position": "absolute",
                "transition": "all 250ms ease-in",
                "border-style": "solid",
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

            ".button_wrap": {
                "background-color": base.content_background_color,
                "z-index": 3,
                "display": "block",
                "position": "relative",
                "overflow": "hidden",
                "transition": "background-color 250ms ease-in",
            },

            // The actual button
            ".button": {
                "color": base.button_text_color,
                "cursor": "pointer",
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

            // highlight when clicked and fades out shortly
            ".highlight": {
                  "z-index": 1,
                  "position": "absolute",
                  "left": 0,
                  "right": 0,
                  "top": 0,
                  "bottom": 0,
                  "background-color": base.highlight_color,
                  "opacity": 0,
                  "transition": "all 50ms ease-out",
            },

            ".clicked .highlight": {
                "opacity": 1,
            },

        });

        let skewed_css = jss_ns!(COMPONENT_NAME, {
            ".skewed": {
                "transform": "skewX(-45deg)",
                "transform-origin": "bottom left",
            },

            ".skewed .button": {
                "transform": "skewX(45deg)",
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
                "left": "-6px",
                "top": "-6px",
            },

            ".expand_corners.hovered .corner__bottom-left": {
                "left": "-6px",
                "bottom": "-6px",
            },

            ".expand_corners.hovered .corner__top-right": {
                "right": "-6px",
                "top": "-6px",
            },

            ".expand_corners.hovered .corner__bottom-right": {
                "right": "-6px",
                "bottom": "-6px",
            },
        });

        vec![base_css, skewed_css, expand_corner_css]
    }

    pub fn update(&mut self, msg: Msg<PMSG>) -> Option<PMSG> {
        match msg {
            Msg::Click => {
                if self.options.sound {
                    sounds::play(&self.audio);
                }
                self.click = true;
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
        let class_ns =
            |class_names| jss::class_namespaced(COMPONENT_NAME, class_names);

        let classes_ns_flag = |class_name_flags| {
            jss::classes_namespaced_flag(COMPONENT_NAME, class_name_flags)
        };

        div(
            vec![
                class(COMPONENT_NAME),
                classes_ns_flag([
                    ("clicked", self.click),
                    ("click_highlights", self.options.click_highlights),
                    ("expand_corners", self.options.expand_corners),
                    ("has_hover", self.options.has_hover),
                    ("hovered", self.hover),
                    ("skewed", self.options.skewed),
                    // setting this will also disable the div, therefore will not activate the
                    // events on it
                    ("disabled", self.options.disabled),
                    ("hidden", self.options.hidden),
                ]),
                // normally click should be attached to the actual button element
                on_click(|_| Msg::Click),
                // the mouseover events are attached here since the hover element z-index is
                // higher than the actual button, which will cause a janky animation
                // when the mouse is triggering alt hover in and out, since covered by the hover
                // layer effect
                on_mouseover(|_| Msg::HoverIn),
                on_mouseout(|_| Msg::HoverOut),
            ],
            vec![
                // hover
                view_if(
                    self.options.has_hover,
                    div(vec![class_ns("hover hover-bottom")], vec![]),
                ),
                //borders
                div(vec![class_ns("border border-bottom")], vec![]),
                div(vec![class_ns("border border-left")], vec![]),
                div(vec![class_ns("border border-right")], vec![]),
                div(vec![class_ns("border border-top")], vec![]),
                div(vec![class_ns("border border-bottom")], vec![]),
                // corners
                view_if(
                    self.options.has_corners,
                    div(vec![class_ns("corner corner__top-left")], vec![]),
                ),
                view_if(
                    self.options.has_corners,
                    div(vec![class_ns("corner corner__bottom-left")], vec![]),
                ),
                view_if(
                    self.options.has_corners,
                    div(vec![class_ns("corner corner__top-right")], vec![]),
                ),
                view_if(
                    self.options.has_corners,
                    div(vec![class_ns("corner corner__bottom-right")], vec![]),
                ),
                div(
                    vec![],
                    vec![
                        div(
                            vec![class_ns("button_wrap")],
                            vec![button(
                                vec![
                                    class_ns("button"),
                                    disabled(self.options.disabled),
                                ],
                                vec![text(&self.label)],
                            )
                            .add_attributes(self.event_listeners.clone())],
                        ),
                        div(
                            vec![
                                class_ns("highlight"),
                                on_transitionend(|_| Msg::HighlightEnd),
                            ],
                            vec![],
                        ),
                    ],
                ),
            ],
        )
    }
}

impl Options {
    /// bare minimum button
    /// no sound
    pub fn bare() -> Self {
        Options {
            sound: false,
            click_highlights: false,
            skewed: false,
            has_corners: false,
            expand_corners: false,
            has_hover: false,
            disabled: false,
            hidden: false,
        }
    }

    /// full effect, skewed
    pub fn full() -> Self {
        Options {
            sound: true,
            click_highlights: true,
            skewed: true,
            has_corners: true,
            expand_corners: true,
            has_hover: true,
            disabled: false,
            hidden: false,
        }
    }

    /// regular futuristic button
    pub fn regular() -> Self {
        Options {
            sound: true,
            click_highlights: true,
            skewed: false,
            has_corners: true,
            expand_corners: true,
            has_hover: true,
            disabled: false,
            hidden: false,
        }
    }

    /// just like regular but muted
    /// sound off
    pub fn muted() -> Self {
        Options {
            sound: false,
            click_highlights: true,
            skewed: false,
            has_corners: true,
            expand_corners: true,
            has_hover: true,
            disabled: false,
            hidden: false,
        }
    }

    /// no corners, no hover
    pub fn simple() -> Self {
        Options {
            sound: true,
            click_highlights: true,
            skewed: false,
            has_corners: false,
            expand_corners: false,
            has_hover: false,
            disabled: false,
            hidden: false,
        }
    }

    ///does not interact
    pub fn disabled() -> Self {
        Options {
            sound: false,
            click_highlights: false,
            skewed: false,
            has_corners: false,
            expand_corners: false,
            has_hover: false,
            disabled: true,
            hidden: false,
        }
    }

    pub fn skewed(mut self, skewed: bool) -> Self {
        self.skewed = skewed;
        self
    }

    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }
}
