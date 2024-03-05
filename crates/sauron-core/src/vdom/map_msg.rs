use crate::{
    html::attributes::AttributeValue,
    vdom::{Attribute, Element, Listener, Node},
};



impl<MSG> Node<MSG>
{
    /// map the msg of this node such that Node<MSG> becomes Node<MSG2>
    pub fn map_msg<F, MSG2>(self, cb: F) -> Node<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static,
        MSG: 'static,
    {
        match self {
            Node::Element(element) => Node::Element(element.map_msg(cb)),
            Node::Leaf(leaf) => Node::Leaf(leaf),
            Node::Fragment(nodes) => Node::Fragment(
                nodes
                    .into_iter()
                    .map(|node| node.map_msg(cb.clone()))
                    .collect(),
            ),
            Node::NodeList(node_list) => Node::NodeList(
                node_list
                    .into_iter()
                    .map(|node| node.map_msg(cb.clone()))
                    .collect(),
            ),
        }
    }

    pub(crate) fn get_callbacks(&self) -> Vec<&Listener<MSG>> {
        if let Some(attributes) = self.attributes() {
            let callbacks = attributes
                .iter()
                .flat_map(|att| att.get_callback())
                .collect();
            callbacks
        } else {
            vec![]
        }
    }
}

impl<MSG> Element<MSG>
{
    /// map the msg of this element such that `Element<MSG>` becomes `Element<MSG2>`
    pub fn map_msg<F, MSG2>(self, cb: F) -> Element<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static,
        MSG: 'static,
    {
        Element {
            namespace: self.namespace,
            tag: self.tag,
            attrs: self
                .attrs
                .into_iter()
                .map(|attr| attr.map_msg(cb.clone()))
                .collect(),
            children: self
                .children
                .into_iter()
                .map(|child| child.map_msg(cb.clone()))
                .collect(),
            self_closing: self.self_closing,
        }
    }
}

impl<MSG> Attribute<MSG>
{
    /// map the msg of this attribute such that `Attribute<MSG>` becomes `Attribute<MSG2>`
    pub fn map_msg<F, MSG2>(self, cb: F) -> Attribute<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static,
        MSG: 'static,
    {
        Attribute {
            name: self.name,
            value: self
                .value
                .into_iter()
                .map(|v| v.map_msg(cb.clone()))
                .collect(),
            namespace: self.namespace,
        }
    }

    pub(crate) fn get_callback(&self) -> Vec<&Listener<MSG>> {
        self.value
            .iter()
            .filter_map(|v| v.as_event_listener())
            .collect()
    }

    pub(crate) fn is_static_str(&self) -> bool {
        self.value.iter().all(|v|v.is_static_str())
    }
}

impl<MSG> AttributeValue<MSG>
{
    /// map the msg of this AttributeValue such that `AttributeValue<MSG>` becomes
    /// `AttributeValue<MSG2>`
    pub fn map_msg<F, MSG2>(self, cb: F) -> AttributeValue<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static,
        MSG: 'static,
    {
        match self {
            AttributeValue::FunctionCall(this) => AttributeValue::FunctionCall(this),
            AttributeValue::Simple(this) => AttributeValue::Simple(this),
            AttributeValue::Style(this) => AttributeValue::Style(this),
            AttributeValue::EventListener(this) => AttributeValue::EventListener(this.map_msg(cb)),
            AttributeValue::Empty => AttributeValue::Empty,
        }
    }
}
