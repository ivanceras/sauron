use crate::{
    html::attributes::AttributeValue,
    vdom::{Attribute, Element, Listener, Node},
};

/// Add mapping function for Node, Element, Attribute,
pub trait NodeMapMsg<MSG>
where
    MSG: 'static,
{
    /// map the msg of this node such that Node<MSG> becomes Node<MSG2>
    fn map_msg<F, MSG2>(self, cb: F) -> Node<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static;

    /// Return the callbacks present on this node
    fn get_callbacks(&self) -> Vec<&Listener<MSG>>;
}

/// Add mapping function for Element
pub trait ElementMapMsg<MSG>
where
    MSG: 'static,
{
    /// map the msg of this element such that `Element<MSG>` becomes `Element<MSG2>`
    fn map_msg<F, MSG2>(self, cb: F) -> Element<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static;
}

/// Add mapping function for Attribute
pub trait AttributeMapMsg<MSG>
where
    MSG: 'static,
{
    /// map the msg of this attribute such that `Attribute<MSG>` becomes `Attribute<MSG2>`
    fn map_msg<F, MSG2>(self, cb: F) -> Attribute<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static;

    /// return the callback values of this attribute
    fn get_callback(&self) -> Vec<&Listener<MSG>>;

    /// returns true if the value of this attribute is static str
    fn is_static_str(&self) -> bool;
}

impl<MSG> NodeMapMsg<MSG> for Node<MSG>
where
    MSG: 'static,
{
    fn map_msg<F, MSG2>(self, cb: F) -> Node<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static,
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

    fn get_callbacks(&self) -> Vec<&Listener<MSG>> {
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

impl<MSG> ElementMapMsg<MSG> for Element<MSG>
where
    MSG: 'static,
{
    fn map_msg<F, MSG2>(self, cb: F) -> Element<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static,
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

impl<MSG> AttributeMapMsg<MSG> for Attribute<MSG>
where
    MSG: 'static,
{
    fn map_msg<F, MSG2>(self, cb: F) -> Attribute<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static,
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

    fn get_callback(&self) -> Vec<&Listener<MSG>> {
        self.value
            .iter()
            .filter_map(|v| v.as_event_listener())
            .collect()
    }

    fn is_static_str(&self) -> bool {
        self.value.iter().all(|v|v.is_static_str())
    }
}

impl<MSG> AttributeValue<MSG>
where
    MSG: 'static,
{
    /// map the msg of this AttributeValue such that `AttributeValue<MSG>` becomes
    /// `AttributeValue<MSG2>`
    pub fn map_msg<F, MSG2>(self, cb: F) -> AttributeValue<MSG2>
    where
        F: Fn(MSG) -> MSG2 + Clone + 'static,
        MSG2: 'static,
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
