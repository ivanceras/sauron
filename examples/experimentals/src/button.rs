use crate::dom::template;
use crate::dom::DomAttr;
use crate::dom::DomAttrValue;
use sauron::dom::Component;
use sauron::dom::StatefulComponent;
use sauron::prelude::*;
use sauron::vdom::AttributeName;

pub enum Msg {
    Click,
}

#[derive(Default)]
pub struct Button {
    cnt: i32,
}

impl Component<Msg, ()> for Button {
    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        match msg {
            Msg::Click => self.cnt += 1,
        }
        Effects::none()
    }

    fn view(&self) -> Node<Msg> {
        node! {
            <button on_click=|_|Msg::Click >Hello!{text!("I'm just a button, clicked {} time(s)", self.cnt)}</button>
        }
    }
}

impl StatefulComponent for Button {
    fn build(
        attrs: impl IntoIterator<Item = DomAttr>,
        children: impl IntoIterator<Item = web_sys::Node>,
    ) -> Self
    where
        Self: Sized,
    {
        Button::default()
    }

    fn attribute_changed(
        &mut self,
        attr_name: AttributeName,
        old_value: DomAttrValue,
        new_value: DomAttrValue,
    ) where
        Self: Sized,
    {
    }

    fn template(&self) -> web_sys::Node {
        template::build_template(&Component::view(self))
    }

    /// remove the attribute with this name
    fn remove_attribute(&mut self, attr_name: AttributeName) {}

    /// append a child into this component
    fn append_child(&mut self, child: web_sys::Node) {}

    /// remove a child in this index
    fn remove_child(&mut self, index: usize) {}

    /// the component is attached to the dom
    fn connected_callback(&mut self) {}
    /// the component is removed from the DOM
    fn disconnected_callback(&mut self) {}

    /// the component is moved or attached to the dom
    fn adopted_callback(&mut self) {}
}
