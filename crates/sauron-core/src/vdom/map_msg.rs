use crate::{
    html::attributes::{AttributeValue, Listener},
    vdom::{Attribute, Element, Event, Leaf, Node},
};

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
            Node::Leaf(leaf) => Node::Leaf(leaf.map_callback(cb)),
            Node::NodeList(node_list) => Node::NodeList(
                node_list
                    .into_iter()
                    .map(|node| node.map_callback(cb.clone()))
                    .collect(),
            ),
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
    pub fn map_callback<MSG2>(self, cb: Listener<MSG, MSG2>) -> AttributeValue<MSG2>
    where
        MSG2: 'static,
    {
        match self {
            AttributeValue::FunctionCall(this) => AttributeValue::FunctionCall(this),
            AttributeValue::Simple(this) => AttributeValue::Simple(this),
            AttributeValue::Style(this) => AttributeValue::Style(this),
            AttributeValue::EventListener(this) => {
                AttributeValue::EventListener(this.map_callback(cb))
            }
            AttributeValue::Empty => AttributeValue::Empty,
        }
    }
}

impl<MSG> Leaf<MSG>
where
    MSG: 'static,
{
    fn map_callback<MSG2>(self, cb: Listener<MSG, MSG2>) -> Leaf<MSG2>
    where
        MSG2: 'static,
    {
        match self {
            Self::Text(v) => Leaf::Text(v),
            Self::SafeHtml(v) => Leaf::SafeHtml(v),
            Self::Comment(v) => Leaf::Comment(v),
            Self::Fragment(v) => {
                Leaf::Fragment(v.into_iter().map(|n| n.map_callback(cb.clone())).collect())
            }
            Self::DocType(v) => Leaf::DocType(v),
        }
    }
}
