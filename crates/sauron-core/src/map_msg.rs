use crate::html::attributes::AttributeValue;
use crate::html::attributes::Callback;
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
    fn map_callback<MSG2>(self, cb: Callback<MSG, MSG2>) -> Node<MSG2>
    where
        MSG2: 'static;
}

/// Add mapping function for Element
pub trait ElementMapMsg<MSG>
where
    MSG: 'static,
{
    /// map_callback the return of the callback from MSG to MSG2
    fn map_callback<MSG2>(self, cb: Callback<MSG, MSG2>) -> Element<MSG2>
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
    fn map_callback<MSG2>(self, cb: Callback<MSG, MSG2>) -> Attribute<MSG2>
    where
        MSG2: 'static;

    /// return the callback values of this attribute
    fn get_callback(&self) -> Vec<&Callback<Event, MSG>>;
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
        let cb = Callback::from(func);
        self.map_callback(cb)
    }

    /// map the msg of callback of this element node
    fn map_callback<MSG2>(self, cb: Callback<MSG, MSG2>) -> Node<MSG2>
    where
        MSG2: 'static,
    {
        match self {
            Node::Element(element) => Node::Element(element.map_callback(cb)),
            Node::Text(text) => Node::Text(text),
        }
    }
}

impl<MSG> ElementMapMsg<MSG> for Element<MSG>
where
    MSG: 'static,
{
    /// map_callback the return of the callback from MSG to MSG2
    fn map_callback<MSG2>(self, cb: Callback<MSG, MSG2>) -> Element<MSG2>
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
        let cb = Callback::from(func);
        self.map_callback(cb)
    }

    /// transform the callback of this attribute
    fn map_callback<MSG2>(self, cb: Callback<MSG, MSG2>) -> Attribute<MSG2>
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
    fn get_callback(&self) -> Vec<&Callback<Event, MSG>> {
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
        cb: Callback<MSG, MSG2>,
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
