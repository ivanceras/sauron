use crate::dom::DomAttr;
use crate::dom::DomAttrValue;
use sauron::dom::Component;
use sauron::dom::DomNode;
use sauron::dom::StatefulComponent;
use sauron::prelude::*;
use sauron::vdom::AttributeName;

#[derive(Default)]
pub enum Msg {
    Click,
    ExternContMounted(DomNode),
    #[default]
    NoOp,
}

#[derive(Default)]
pub struct Button {
    /// holds the children while the external children node hasn't been mounted
    children: Vec<DomNode>,
    external_children_node: Option<DomNode>,
    cnt: i32,
}

impl Component for Button {
    type MSG = Msg;
    type XMSG = ();

    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        match msg {
            Msg::Click => self.cnt += 1,
            Msg::ExternContMounted(target_node) => {
                log::info!("Button: extenal container mounted...");
                target_node
                    .append_children(self.children.drain(..))
                    .expect("must append");
                self.external_children_node = Some(target_node);
            }
            Msg::NoOp => (),
        }
        Effects::none()
    }

    view! {
        <button on_click=|_|Msg::Click >
            <span>Hello!{text!("I'm just a button, clicked {} time(s)", self.cnt)}</span>
            <div class="external_children" on_mount=|me|Msg::ExternContMounted(me.target_node)></div>
        </button>
    }
}

impl StatefulComponent for Button {
    fn attribute_changed(
        &mut self,
        attr_name: &str,
        old_value: DomAttrValue,
        new_value: DomAttrValue,
    ) where
        Self: Sized,
    {
    }

    /// append a child into this component
    fn append_children(&mut self, children: Vec<DomNode>) {
        if let Some(external_children_node) = self.external_children_node.as_ref() {
            log::info!("Btn ok appending..");
            external_children_node
                .append_children(children)
                .expect("must append");
        } else {
            log::debug!(
                "Button: Just pushing to children since the external holder is not yet mounted"
            );
            self.children.extend(children);
        }
    }
}
