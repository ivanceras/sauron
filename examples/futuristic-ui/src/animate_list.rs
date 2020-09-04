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
            animating: false,
            animated_layer: None,
            children,
        }
    }

    fn children(&self) -> Node<MSG> {
        self.children.clone()
    }

    fn play_sound(&self) {
        let audio = HtmlAudioElement::new_with_src("sounds/typing.mp3")
            .expect("must not fail");
        let _ = audio.play().expect("must play");
    }

    pub fn animate_in(&mut self) -> Option<Msg> {
        self.play_sound();
        self.start_animation(true)
    }

    fn stop_animation(&mut self) -> Option<Msg> {
        self.animating = false;
        None
    }

    fn start_animation(&mut self, is_in: bool) -> Option<Msg> {
        let content_len = Self::node_count(&self.children());

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

        log::trace!("returning a cmd for next animation..");
        Some(Msg::NextAnimation(is_in, start, duration))
    }

    /// count the number of elements on this node tree
    fn node_count(node: &Node<MSG>) -> usize {
        let mut node_idx = 0;
        Self::node_count_recursive(node, &mut node_idx);
        node_idx
    }

    /// recursively count the number of elements on this node tree
    /// 1 count for each character and each element
    fn node_count_recursive(node: &Node<MSG>, node_idx: &mut usize) {
        if let Some(children) = node.get_children() {
            for child in children {
                match child {
                    Node::Element(_) => {
                        *node_idx += 1;
                        Self::node_count_recursive(child, node_idx);
                    }
                    Node::Text(txt) => {
                        *node_idx += txt.len();
                    }
                }
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
                match child_src {
                    Node::Element(element) => {
                        if *node_idx < node_idx_limit {
                            let child_src_tag = element.tag();
                            match *child_src_tag {
                                // add everything row by row,
                                // since the default behavior is adding cell by cell, which has an ugly
                                // sliding effect
                                "tr" => {
                                    Self::node_count_recursive(
                                        child_src, node_idx,
                                    );
                                    dest.add_children_ref_mut(vec![
                                        child_src.clone()
                                    ]);
                                }
                                // tags other than tr
                                _ => {
                                    *node_idx += 1;
                                    let child_src_attr =
                                        element.get_attributes();
                                    // dont include the children on this clone
                                    let child_clone = html_element(
                                        child_src_tag,
                                        child_src_attr.to_vec(),
                                        vec![],
                                    );
                                    //TODO: add a skip mechanism here to add the element right away
                                    //including it's children and increment the node_idx to the remaining
                                    //tree leaf count of the element
                                    // This is needed for <table> element, to avoid wobly animation
                                    // where columns seems to be flying from the left

                                    dest.add_children_ref_mut(vec![
                                        child_clone,
                                    ]);
                                    let dest_children = dest
                                        .children_mut()
                                        .expect("must have a dest children");

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
                    Node::Text(txt) => {
                        let txt_len = txt.len();
                        let truncate_len =
                            std::cmp::min(txt_len, node_idx_limit - *node_idx);

                        let start = 0;
                        let end = truncate_len;

                        log::trace!("txt_len: {}, node_idx: {}, node_idx_limit: {}, truncate_len: {},", txt_len, node_idx, node_idx_limit, truncate_len);
                        let truncated_txt = &txt[start..end];
                        let text_node = Node::Text(truncated_txt.to_string());
                        dest.add_children_ref_mut(vec![text_node]);
                        // we append the blinking character to the end of the text
                        // here, and only when this node has not yet finish animating..
                        if truncate_len < txt_len {
                            let blink =
                                span(vec![class("blink")], vec![text("█")]);
                            dest.add_children_ref_mut(vec![blink]);
                        }
                        *node_idx += txt_len;
                    }
                };
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

        let children = self.children();
        let tag = children.tag().expect("must have a tag");
        let attributes =
            children.get_attributes().expect("must have attributes");

        let mut dest: Node<MSG> =
            html_element(tag, attributes.to_vec(), vec![]);

        Self::include_node(&mut dest, &self.children(), new_length);
        self.animated_layer = Some(dest);

        let continue_animation = if is_in {
            new_length <= (content_len - 1)
        } else {
            new_length > 0
        };

        if continue_animation {
            log::trace!("continue animation");
            Some(Msg::NextAnimation(is_in, start, duration))
        } else {
            log::trace!("stop the animation");
            Some(Msg::StopAnimation)
        }
    }

    pub fn update(&mut self, msg: Msg) -> Option<Msg> {
        log::trace!("animate_list updating..");
        match msg {
            Msg::AnimateIn => {
                log::trace!("animate in started...");
                self.animate_in()
            }
            Msg::StopAnimation => {
                log::trace!("animate_list stop_animation..");
                self.stop_animation();
                None
            }
            Msg::NextAnimation(is_in, start, duration) => {
                log::trace!("next animationg executed..");
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
                            vec![span(
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
