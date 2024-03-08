use crate::dom::template;
use crate::dom::DomAttr;
use crate::dom::DomAttrValue;
use sauron::dom::Component;
use sauron::dom::StatefulComponent;
use sauron::prelude::*;
use sauron::vdom::AttributeName;
use sauron::dom::DomNode;



pub enum Msg {
    Click,
    ExternContMounted(web_sys::Node),
}

#[derive(Default)]
pub struct Button {
    /// holds the children while the external children node hasn't been mounted
    children: Vec<web_sys::Node>,
    external_children_node: Option<web_sys::Node>,
    cnt: i32,
}

impl Component<Msg, ()> for Button {
    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        match msg {
            Msg::Click => self.cnt += 1,
            Msg::ExternContMounted(target_node) => {
                log::info!("Button: extenal container mounted...");
                for child in self.children.iter(){
                    target_node.append_child(child).expect("must append");
                }
                self.external_children_node = Some(target_node);
            }
        }
        Effects::none()
    }

    fn view(&self) -> Node<Msg> {
        node! {
            <button on_click=|_|Msg::Click >
                Hello!{text!("I'm just a button, clicked {} time(s)", self.cnt)}
                <div class="external_children" on_mount=|me|Msg::ExternContMounted(me.target_node)></div> 
            </button>
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
    fn append_child(&mut self, child: &web_sys::Node) {
        log::info!("Btn appending {:?}", child);
        if let Some(external_children_node) = self.external_children_node.as_ref(){
            log::info!("Btn ok appending..");
            external_children_node.append_child(child).expect("must append");
        }else{
            log::debug!("Button: Just pushing to children since the external holder is not yet mounted");
            self.children.push(child.clone());
        }
    }

    /// remove a child in this index
    fn remove_child(&mut self, index: usize) {}

    /// the component is attached to the dom
    fn connected_callback(&mut self) {}
    /// the component is removed from the DOM
    fn disconnected_callback(&mut self) {}

    /// the component is moved or attached to the dom
    fn adopted_callback(&mut self) {}
}
