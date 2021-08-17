use crate::{animate_list, AnimateList};
use sauron::{prelude::*, Node};

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
            animated_list: AnimateList::new_with_content(
                sauron_markdown::markdown(md),
            ),
        }
    }

    pub fn update(&mut self, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::AnimateIn => {
                let amsg =
                    self.animated_list.update(animate_list::Msg::AnimateIn);

                amsg.map(Msg::AnimateListMsg)
            }
            Msg::AnimateListMsg(amsg) => {
                self.animated_list.update(amsg).map(Msg::AnimateListMsg)
            }
        }
    }

    #[allow(unused)]
    pub fn view(&self) -> Node<MSG> {
        p(vec![], vec![self.animated_list.view()])
    }
}
