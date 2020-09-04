use crate::{
    animate_list,
    AnimateList,
};
use sauron::{
    html::{
        attributes::class,
        div,
        text,
    },
    prelude::*,
    Node,
};
use std::marker::PhantomData;
use web_sys::HtmlAudioElement;

#[derive(Clone, Debug)]
pub enum Msg {
    AnimateIn,
    AnimateListMsg(animate_list::Msg),
}

/// accepts a markdown and animate the content
pub struct Paragraph<MSG> {
    animated_list: AnimateList<MSG>,
}

impl<MSG> Paragraph<MSG>
where
    MSG: Clone,
{
    pub fn new_with_markdown(md: &str) -> Self {
        Paragraph {
            animated_list: AnimateList::new_with_content(sauron::markdown(md)),
        }
    }

    pub fn update(&mut self, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::AnimateIn => {
                self.animated_list
                    .update(animate_list::Msg::AnimateIn)
                    .map(Msg::AnimateListMsg)
            }
            Msg::AnimateListMsg(amsg) => {
                self.animated_list.update(amsg).map(Msg::AnimateListMsg)
            }
        }
    }

    pub fn view(&self) -> Node<MSG> {
        p(vec![], vec![self.animated_list.view()])
    }
}
