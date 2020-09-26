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

#[derive(Clone, Debug)]
pub enum Msg {
    AnimateIn,
    StopAnimation,
    NextAnimation(bool, f64, f64),
}

pub struct AnimateList<MSG> {
    audio: HtmlAudioElement,
    animated_layer: Option<Node<MSG>>,
    children: Node<MSG>,
    animating: bool,
}

impl<MSG> AnimateList<MSG>
where
    MSG: Clone,
{
    pub fn new_with_content(children: Node<MSG>) -> Self {
        AnimateList {
            audio: sounds::preload("sounds/typing.mp3"),
            animating: false,
            animated_layer: None,
            children,
        }
    }

    fn children(&self) -> Node<MSG> {
        self.children.clone()
    }

    pub fn animate_in(&mut self) -> Option<Msg> {
        sounds::play(&self.audio);
        self.start_animation(true)
    }

    fn stop_animation(&mut self) -> Option<Msg> {
        self.animating = false;
        None
    }

    fn start_animation(&mut self, is_in: bool) -> Option<Msg> {
        let content_len = Self::node_count_chars(&self.children());

        if content_len == 0 {
            return None;
        }

        let interval = 1_000.0 / 60.0;
        let real_duration = interval * content_len as f64;
        let timeout = 500.0;
        let duration = real_duration.min(timeout);
        let start = crate::dom::now();

        self.animating = true;
        if is_in {
            self.animated_layer = None;
        }

        Some(Msg::NextAnimation(is_in, start, duration))
    }

    /// count the number of elements on this node tree
    fn node_count_chars(node: &Node<MSG>) -> usize {
        let mut current_cnt = 0;
        Self::node_count_chars_recursive(node, &mut current_cnt);
        current_cnt
    }

    /// recursively count the number of elements on this node tree
    /// 1 count for each character and each element
    fn node_count_chars_recursive(node: &Node<MSG>, current_cnt: &mut usize) {
        match node {
            Node::Element(element) => {
                for child in element.children.iter() {
                    Self::node_count_chars_recursive(child, current_cnt);
                }
            }
            Node::Text(txt) => {
                *current_cnt += txt.text.len();
            }
        }
    }

    /// include the the element from the src to dest
    /// as long as its current_cnt is less than the chars_limit
    fn include_node(dest: &mut Node<MSG>, src: &Node<MSG>, chars_limit: usize) {
        let mut current_cnt = 0;
        Self::include_node_recursive(dest, src, chars_limit, &mut current_cnt);
    }

    /// recursively include the element from src to dest
    /// until all of the current_cnt that is lesser than chars_limit is added.
    fn include_node_recursive(
        dest: &mut Node<MSG>,
        src: &Node<MSG>,
        chars_limit: usize,
        current_cnt: &mut usize,
    ) {
        match src {
            Node::Element(element) => {
                if *current_cnt < chars_limit {
                    let shallow_src = html_element(
                        element.tag,
                        element.attrs.clone(),
                        vec![],
                    );
                    dest.add_children_ref_mut(vec![shallow_src]);

                    let last_index = dest
                        .as_element_ref()
                        .expect("this is an element")
                        .children
                        .len()
                        - 1;

                    let mut just_added_child = &mut dest
                        .children_mut()
                        .expect("must have children, since just added 1")
                        .get_mut(last_index)
                        .expect("must get the last child");

                    for child in element.children.iter() {
                        Self::include_node_recursive(
                            &mut just_added_child,
                            child,
                            chars_limit,
                            current_cnt,
                        );
                    }
                }
            }
            Node::Text(txt) => {
                let txt_len = txt.text.len();
                let truncate_len = if chars_limit > *current_cnt {
                    std::cmp::min(txt_len, chars_limit - *current_cnt)
                } else {
                    0
                };

                if truncate_len > 0 {
                    let start = 0;
                    let end = truncate_len;

                    let truncated_txt = &txt.text[start..end];
                    let text_node = Node::Text(Text::new(truncated_txt));
                    dest.add_children_ref_mut(vec![text_node]);
                    // we append the blinking character to the end of the text
                    // here, and only when this node has not yet finish animating..
                    if truncate_len < txt_len {
                        let blink = span(vec![class("blink")], vec![text("█")]);
                        dest.add_children_ref_mut(vec![blink]);
                    }
                }
                *current_cnt += truncate_len;
            }
        }
    }

    fn next_animation(
        &mut self,
        is_in: bool,
        start: f64,
        duration: f64,
    ) -> Option<Msg> {
        let timestamp = crate::dom::now();

        let content_len = Self::node_count_chars(&self.children());

        let mut anim_progress = (timestamp - start).max(0.0);
        if !is_in {
            anim_progress = duration - anim_progress;
        }

        let new_length =
            (anim_progress * content_len as f64 / duration).round() as usize;

        let mut dest: Node<MSG> = div(vec![], vec![]);

        Self::include_node(&mut dest, &self.children(), new_length);
        self.animated_layer = Some(dest);

        let continue_animation = if is_in {
            new_length <= (content_len - 1)
        } else {
            new_length > 0
        };

        if continue_animation {
            Some(Msg::NextAnimation(is_in, start, duration))
        } else {
            Some(Msg::StopAnimation)
        }
    }

    pub fn update(&mut self, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::AnimateIn => self.animate_in(),
            Msg::StopAnimation => {
                self.stop_animation();
                None
            }
            Msg::NextAnimation(is_in, start, duration) => {
                self.next_animation(is_in, start, duration)
            }
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
              animation: animate_list_blink-anim 250ms step-end infinite;
            }

            .animating .animate_list_children {
                opacity: 0;
             }

            .animating .animated_layer_wrapper {
                opacity: 1;
            }

            @keyframes animate_list_blink-anim {
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

    // Note: opacity: 0 on span will have no effect on webkit browser
    // however, it has an effect on firefox
    pub fn view(&self) -> Node<MSG> {
        div(
            vec![],
            vec![div(
                vec![
                    class("animate_list"),
                    classes_flag([("animating", self.animating)]),
                ],
                vec![
                    div(
                        vec![class("animate_list_children")],
                        vec![self.children()],
                    ),
                    view_if(
                        self.animating,
                        div(
                            vec![class("animated_layer_wrapper")],
                            vec![div(
                                vec![class("animated_layer")],
                                if let Some(animated_layer) =
                                    &self.animated_layer
                                {
                                    vec![animated_layer.clone()]
                                } else {
                                    vec![]
                                },
                            )],
                        ),
                    ),
                ],
            )],
        )
    }
}
