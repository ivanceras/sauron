use sauron::html::attributes::{class, id, style};
use sauron::html::events::on_click;
use sauron::html::{div, text};
use sauron::prelude::*;
use sauron::{Cmd, Component, Node, Program};
use std::marker::PhantomData;
use web_sys::HtmlAudioElement;

#[derive(Clone)]
pub enum Msg<MSG> {
    AnimateIn,
    StopAnimation,
    NextAnimation(bool, f64, f64),
    ParamMsg(MSG),
}

pub struct AnimateList<MSG> {
    _phantom: PhantomData<MSG>,
    animated_layer: Option<Node<MSG>>,
    animating: bool,
}

impl<MSG> AnimateList<MSG>
where
    MSG: Clone,
{
    pub fn new() -> Self {
        AnimateList {
            animating: false,
            animated_layer: None,
            _phantom: PhantomData,
        }
    }

    fn children(&self) -> Node<MSG> {
        ul(
            vec![],
            vec![
                li(vec![], vec![text("List 1")]),
                li(vec![], vec![text("List 2")]),
                li(vec![], vec![text("List 3")]),
                li(vec![], vec![text("List 4")]),
                ul(
                    vec![],
                    vec![
                        li(vec![], vec![text("SubList 1")]),
                        li(vec![], vec![text("SubList 2")]),
                        li(vec![], vec![text("SubList 3")]),
                        li(vec![], vec![text("SubList 4")]),
                    ],
                ),
                li(vec![], vec![text("List 6")]),
                li(vec![], vec![text("List 7")]),
                li(vec![], vec![text("List 8")]),
                li(vec![], vec![text("List 9")]),
                li(vec![], vec![text("List 10")]),
                /*
                li(vec![], vec![text("List 11")]),
                li(vec![], vec![text("List 12")]),
                li(vec![], vec![text("List 13")]),
                li(vec![], vec![text("List 14")]),
                li(vec![], vec![text("List 15")]),
                li(vec![], vec![text("List 16")]),
                li(vec![], vec![text("List 17")]),
                li(vec![], vec![text("List 18")]),
                li(vec![], vec![text("List 19")]),
                li(vec![], vec![text("List 20")]),
                */
            ],
        )
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

        let content_len = Self::node_count(&self.children());

        if content_len == 0 {
            return Cmd::none();
        }

        let interval = 1_000.0 / 60.0;
        let real_duration = interval * content_len as f64;
        let timeout = 500.0;
        //let duration = real_duration.min(timeout);
        let duration = 3000.0;
        let start = crate::dom::now();

        self.animating = true;
        if is_in {
            self.animated_layer = None;
        }

        log::trace!("returning a cmd for next animation..");
        Cmd::new(move |program| {
            program.dispatch(crate::Msg::AnimateListMsg(Box::new(
                Msg::NextAnimation(is_in, start, duration),
            )))
        })
    }

    /// count the number of elements on this node tree
    fn node_count(node: &Node<MSG>) -> usize {
        let mut node_idx = 0;
        Self::node_count_recursive(node, &mut node_idx);
        node_idx
    }

    /// recursively count the number of elements on this node tree
    fn node_count_recursive(node: &Node<MSG>, node_idx: &mut usize) {
        if let Some(children) = node.get_children() {
            for child in children {
                *node_idx += 1;
                Self::node_count_recursive(child, node_idx);
            }
        }
    }

    /// include the the element from the src to dest
    /// as long as its node_idx is less than the node_idx_limit
    fn include_node(
        dest: &mut Node<MSG>,
        src: &Node<MSG>,
        node_idx_limit: usize,
    ) {
        let mut node_idx = 0;
        Self::include_node_recursive(dest, src, node_idx_limit, &mut node_idx);
    }

    /// recursively include the element from src to dest
    /// until all of the node_idx that is lesser than node_idx_limit is added.
    fn include_node_recursive(
        dest: &mut Node<MSG>,
        src: &Node<MSG>,
        node_idx_limit: usize,
        node_idx: &mut usize,
    ) {
        if let Some(children_src) = src.get_children() {
            for (index, child_src) in children_src.iter().enumerate() {
                *node_idx += 1;
                if *node_idx < node_idx_limit {
                    let dest_child =
                        if let Some(element) = child_src.as_element_ref() {
                            let child_src_tag =
                                child_src.tag().expect("must have a tag");
                            let child_src_attr = child_src
                                .get_attributes()
                                .expect("must have attributes");
                            html_element(
                                child_src_tag,
                                child_src_attr.to_vec(),
                                vec![],
                            )
                        } else {
                            child_src.clone()
                        };

                    dest.add_children_ref_mut(vec![dest_child]);
                    let dest_children =
                        dest.children_mut().expect("must have a dest children");
                    Self::include_node_recursive(
                        &mut dest_children[index],
                        child_src,
                        node_idx_limit,
                        node_idx,
                    );
                }
            }
        }
    }

    fn next_animation(
        &mut self,
        is_in: bool,
        start: f64,
        duration: f64,
    ) -> Cmd<crate::App, crate::Msg> {
        let timestamp = crate::dom::now();

        let content_len = Self::node_count(&self.children());
        log::trace!("content_len: {}", content_len);

        let mut anim_progress = (timestamp - start).max(0.0);
        if !is_in {
            anim_progress = duration - anim_progress;
        }

        log::trace!("duration: {}", duration);
        log::trace!("timestamp: {}", timestamp);
        log::debug!("content_len: {}", content_len);
        log::debug!("animation progress: {}", anim_progress);

        let new_length =
            (anim_progress * content_len as f64 / duration).round() as usize;

        log::trace!("new_length: {}", new_length);

        let node_idx = std::cmp::min(new_length, content_len);

        let children = self.children();
        let tag = children.tag().expect("must have a tag");
        let attributes =
            children.get_attributes().expect("must have attributes");

        let mut dest: Node<MSG> =
            html_element(tag, attributes.to_vec(), vec![]);

        Self::include_node(&mut dest, &self.children(), node_idx);
        self.animated_layer = Some(dest);

        let continue_animation = if is_in {
            new_length <= (content_len - 1)
        } else {
            new_length > 0
        };

        if continue_animation {
            log::trace!("continue animation");
            Cmd::new(move |program| {
                program.dispatch(crate::Msg::AnimateListMsg(Box::new(
                    Msg::NextAnimation(is_in, start, duration),
                )))
            })
        } else {
            log::trace!("stop the animation");
            Cmd::new(move |program| {
                program.dispatch(crate::Msg::AnimateListMsg(Box::new(
                    Msg::StopAnimation,
                )))
            })
        }
    }

    pub fn update(&mut self, msg: Msg<MSG>) -> Cmd<crate::App, crate::Msg> {
        log::trace!("animate_list updating..");
        match msg {
            Msg::AnimateIn => {
                log::trace!("animate in started...");
                self.animate_in()
            }
            Msg::StopAnimation => {
                log::trace!("animate_list stop_animation..");
                self.stop_animation();
                Cmd::none()
            }
            Msg::NextAnimation(is_in, start, duration) => {
                log::trace!("next animationg executed..");
                self.next_animation(is_in, start, duration)
            }
            Msg::ParamMsg(msg) => Cmd::none(),
        }
    }

    pub fn style(&self) -> Vec<String> {
        vec![r#"
            .animate_list {
                display: inline-block;
                position: relative;
            }

            .animated_layer_wrapper {
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

            .animating .animate_list_children {
                opacity: 0;
             }

            .animating .animated_layer_wrapper {
                opacity: 1;
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
                    class("animate_list"),
                    classes_flag([("animating", self.animating)]),
                ],
                vec![
                    span(
                        vec![class("animate_list_children")],
                        vec![self.children()],
                    ),
                    view_if(
                        self.animating,
                        span(
                            vec![class("animated_layer_wrapper")],
                            vec![
                                span(
                                    vec![class("animated_layer")],
                                    if let Some(animated_layer) =
                                        &self.animated_layer
                                    {
                                        vec![animated_layer.clone()]
                                    } else {
                                        vec![]
                                    },
                                ),
                                span(vec![class("blink")], vec![text("█")]),
                            ],
                        ),
                    ),
                ],
            )],
        )
    }
}
