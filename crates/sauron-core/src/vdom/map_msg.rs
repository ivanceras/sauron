use crate::html::attributes::AttributeValue;
use crate::html::attributes::Listener;
use crate::{Attribute, Element, Event, Node};

/// Add mapping function for Node, Element, Attribute,
pub trait NodeMapMsg<MSG>
where
    MSG: 'static,
{
    /// map the msg of callback of this node
    fn map_msg<F, MSG2>(self, func: F) -> Node<MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static;

    /// map the msg of callback of this element node
    fn map_callback<MSG2>(self, cb: Listener<MSG, MSG2>) -> Node<MSG2>
    where
        MSG2: 'static;

    /// Return the callbacks present on this node
    fn get_callbacks(&self) -> Vec<&Listener<Event, MSG>>;

    /// returns true if this is a text node
    fn is_text(&self) -> bool;

    /// returns true if this is text node with safe html as the content
    fn is_safe_html(&self) -> bool;

    /// unwrap the text content of the text node, panics if it is not a text node
    fn unwrap_text(&self) -> &str;

    /// unwrap the html text content of this node, panics if it is not a safe html node
    fn unwrap_safe_html(&self) -> &str;
}

/// Add mapping function for Element
pub trait ElementMapMsg<MSG>
where
    MSG: 'static,
{
    /// map_callback the return of the callback from MSG to MSG2
    fn map_callback<MSG2>(self, cb: Listener<MSG, MSG2>) -> Element<MSG2>
    where
        MSG2: 'static;
}

/// Add mapping function for Attribute
pub trait AttributeMapMsg<MSG>
where
    MSG: 'static,
{
    /// map the msg
    fn map_msg<F, MSG2>(self, func: F) -> Attribute<MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static;

    /// transform the callback of this attribute
    fn map_callback<MSG2>(self, cb: Listener<MSG, MSG2>) -> Attribute<MSG2>
    where
        MSG2: 'static;

    /// return the callback values of this attribute
    fn get_callback(&self) -> Vec<&Listener<Event, MSG>>;
}

impl<MSG> NodeMapMsg<MSG> for Node<MSG>
where
    MSG: 'static,
{
    /// map the msg of callback of this element node
    fn map_msg<F, MSG2>(self, func: F) -> Node<MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static,
    {
        let cb = Listener::from(func);
        self.map_callback(cb)
    }

    /// map the msg of callback of this element node
    fn map_callback<MSG2>(self, cb: Listener<MSG, MSG2>) -> Node<MSG2>
    where
        MSG2: 'static,
    {
        match self {
            Node::Element(element) => Node::Element(element.map_callback(cb)),
            Node::Leaf(leaf) => Node::Leaf(leaf),
        }
    }

    fn get_callbacks(&self) -> Vec<&Listener<Event, MSG>> {
        if let Some(attributes) = self.get_attributes() {
            let callbacks = attributes
                .iter()
                .flat_map(|att| att.get_callback())
                .collect();
            callbacks
        } else {
            vec![]
        }
    }

    /// returns true if this is a text node
    fn is_text(&self) -> bool {
        match self {
            Self::Leaf(leaf) => leaf.is_text(),
            _ => false,
        }
    }

    /// returns true if this is a safe html text node
    fn is_safe_html(&self) -> bool {
        match self {
            Self::Leaf(leaf) => leaf.is_safe_html(),
            _ => false,
        }
    }
    fn unwrap_text(&self) -> &str {
        match self {
            Self::Leaf(ref leaf) => leaf.unwrap_text(),
            _ => panic!("not a leaf node"),
        }
    }
    fn unwrap_safe_html(&self) -> &str {
        match self {
            Self::Leaf(ref leaf) => leaf.unwrap_safe_html(),
            _ => panic!("not a leaf node"),
        }
    }
}

impl<MSG> ElementMapMsg<MSG> for Element<MSG>
where
    MSG: 'static,
{
    /// map_callback the return of the callback from MSG to MSG2
    fn map_callback<MSG2>(self, cb: Listener<MSG, MSG2>) -> Element<MSG2>
    where
        MSG2: 'static,
    {
        Element {
            namespace: self.namespace,
            tag: self.tag,
            attrs: self
                .attrs
                .into_iter()
                .map(|attr| attr.map_callback(cb.clone()))
                .collect(),
            children: self
                .children
                .into_iter()
                .map(|child| child.map_callback(cb.clone()))
                .collect(),
            self_closing: self.self_closing,
        }
    }
}

impl<MSG> AttributeMapMsg<MSG> for Attribute<MSG>
where
    MSG: 'static,
{
    /// map the msg
    fn map_msg<F, MSG2>(self, func: F) -> Attribute<MSG2>
    where
        F: Fn(MSG) -> MSG2 + 'static,
        MSG2: 'static,
    {
        let cb = Listener::from(func);
        self.map_callback(cb)
    }

    /// transform the callback of this attribute
    fn map_callback<MSG2>(self, cb: Listener<MSG, MSG2>) -> Attribute<MSG2>
    where
        MSG2: 'static,
    {
        Attribute {
            name: self.name,
            value: self
                .value
                .into_iter()
                .map(|v| v.map_callback(cb.clone()))
                .collect(),
            namespace: self.namespace,
        }
    }

    /// return the callback values of this attribute
    fn get_callback(&self) -> Vec<&Listener<Event, MSG>> {
        self.value
            .iter()
            .filter_map(|v| v.as_event_listener())
            .collect()
    }
}

impl<MSG> AttributeValue<MSG>
where
    MSG: 'static,
{
    /// map the callback of this attribute using another callback
    pub fn map_callback<MSG2>(
        self,
        cb: Listener<MSG, MSG2>,
    ) -> AttributeValue<MSG2>
    where
        MSG2: 'static,
    {
        match self {
            AttributeValue::FunctionCall(this) => {
                AttributeValue::FunctionCall(this)
            }
            AttributeValue::Simple(this) => AttributeValue::Simple(this),
            AttributeValue::Style(this) => AttributeValue::Style(this),
            AttributeValue::EventListener(this) => {
                AttributeValue::EventListener(this.map_callback(cb))
            }
            AttributeValue::Empty => AttributeValue::Empty,
        }
    }
}
