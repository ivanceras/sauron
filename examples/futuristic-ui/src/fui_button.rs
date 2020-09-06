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
    /// expand corners when hovered
    expand_corners: bool,
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
            expand_corners: false,
            disabled: false,
            event_listeners: vec![],
        }
    }

    /// whether the button is slanted 45 degree to the right
    pub fn skewed(&mut self, skewed: bool) {
        self.skewed = skewed;
    }

    /// whether to use the alternate color of the theme
    pub fn use_alt(&mut self, use_alt: bool) {
        self.use_alt = use_alt;
    }

    /// whether to show the fancy corners of this button
    /// default: true
    pub fn has_corners(&mut self, has_corners: bool) {
        self.has_corners = has_corners;
    }

    /// default: false
    pub fn disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
        self.has_corners(false);
        self.has_hover(false);
    }

    /// whether or not expand corners on hover
    /// default: false
    pub fn expand_corners(&mut self, expand_corners_on_hover: bool) {
        self.expand_corners = expand_corners_on_hover;
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
        let base = crate::Theme::base();
        let alt = crate::Theme::alt();
        let disabled = crate::Theme::disabled();

        let base_css = jss_ns!(COMPONENT_NAME, {

            // the ROOT component style
            ".": {
                "display": "inline-block",
                "padding": "1px",
                "position": "relative",
                "margin": "4px 4px"
            },



            // HOVER at the lower  part of the button
            ".hover": {
                "border-color": base.hover_color,
                "box-shadow": format!("0 0 4px {}",base.hover_shadow),
                "z-index": 4,
                "opacity": 1,
                "position": "absolute",
                "transition": "width 100ms ease-in",
                "border-style": "solid",
            },

            ".hovered .hover": {
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

        let alt_css = jss_ns!(COMPONENT_NAME, {
            ".alt .hover": {
                "border-color": alt.hover_color,
                "box-shadow": format!("0 0 4px {}",alt.hover_shadow),
            },

            ".alt .border": {
                "border-color": alt.border_color,
                "box-shadow": format!("0 0 4px {}",alt.border_shadow),
            },

            ".alt .corner": {
                "border-color": alt.corner_color,
                "box-shadow": format!("0 0 4px {}",alt.corner_shadow),
            },

            ".alt .button_wrap": {
                "background-color": alt.content_background_color,
            },

            ".alt .button": {
                "color": alt.button_text_color,
            },


            ".alt .highlight": {
                "background-color": alt.highlight_color,
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

        let disabled_css = jss_ns!(COMPONENT_NAME,{

            ".disabled .hover": {
                "border-color": disabled.hover_color,
                "box-shadow": format!("0 0 4px {}",disabled.hover_shadow),
            },
            ".disabled .border": {
                "border-color": disabled.border_color,
                "box-shadow": format!("0 0 4px {}",disabled.border_shadow),
            },

            ".disabled .corner": {
                "border-color": disabled.corner_color,
                "box-shadow": format!("0 0 4px {}",disabled.corner_shadow),
            },

            ".disabled .button": {
                "color": disabled.button_text_color,
                "cursor": "auto",
            },

            ".disabled .button_wrap": {
                "background-color": disabled.content_background_color,
            },

            ".disabled .button": {
                "color": disabled.button_text_color,
            },


            ".disabled .highlight": {
                "background-color": disabled.highlight_color,
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

        vec![
            base_css,
            alt_css,
            skewed_css,
            disabled_css,
            expand_corner_css,
        ]
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
                    ("expand_corners", self.expand_corners),
                    ("hovered", self.hover),
                    ("skewed", self.skewed),
                    ("alt", self.use_alt),
                    ("disabled", self.disabled),
                ]),
                on_click(|_| Msg::Click),
                on_mouseover(|_| Msg::HoverIn),
                on_mouseout(|_| Msg::HoverOut),
            ],
            vec![
                // hover
                view_if(
                    self.has_hover,
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
                    self.has_corners,
                    div(vec![class_ns("corner corner__top-left")], vec![]),
                ),
                view_if(
                    self.has_corners,
                    div(vec![class_ns("corner corner__bottom-left")], vec![]),
                ),
                view_if(
                    self.has_corners,
                    div(vec![class_ns("corner corner__top-right")], vec![]),
                ),
                view_if(
                    self.has_corners,
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
                                    disabled(self.disabled),
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
